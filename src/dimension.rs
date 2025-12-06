//! Provides some arithmetic and conversion between [`Dimension`]'s expressed in arbitrary units.
use core::f32;
use std::error::Error;
use std::fmt::{Display, Formatter, self};
use std::iter::{once, Product};
use std::ops::{Div, Mul};
pub use std::sync::LazyLock;
use crate::bareiss_eliminator::{BareissEliminatorError, RectangularMatrix};

/// A detailed breakdown of the error occurred when getting the conversion exponent from one [`Dimension`]s to another one.
#[derive(Debug, Clone, PartialEq)]
pub enum ConversionExponentError {
    /// Error when both [`Dimension`]s don't have a pair where both exponents are non-zero.
    NoNonZeroExponentPair {
        /// The [`Dimension`] that was trying to be converted from.
        left_dimension: Dimension,
        /// The [`Dimension`] that was trying to be converted to.
        right_dimension: Dimension,
    },
    /// Error when both [`Dimension`]s have an exponent pair whose ratio isn't the same as all the other ones.
    InconsistentExponentRatio {
        /// The [`Dimension`] that was trying to be converted from.
        left_dimension: Dimension,
        /// The [`Dimension`] that was trying to be converted to.
        right_dimension: Dimension,
        /// The numerator of the fraction from the right [`Dimension`].
        right: f64,
        /// The numerator of the fraction from the left [`Dimension`].
        left: f64,
        /// The expected ratio of all exponent pairs whose both exponents are non-zero.
        expected: f64,
        /// The found ratio of the exponent pair that didn't fit the expected.
        found: f64,
    },
}
impl Display for ConversionExponentError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::NoNonZeroExponentPair { left_dimension, right_dimension } =>
                write!(f, "No exponent pair was found where both sides are nonzero: {left_dimension} and {right_dimension}"),
            Self::InconsistentExponentRatio { left_dimension, right_dimension, right, left, expected, found } =>
                write!(f, "Inconsistent ratio exponent between {left_dimension} and {right_dimension}: expected {right}/{left} to be {expected} but found {found}"),
        }
    }
}
impl Error for ConversionExponentError {}

/// Error when the base [`Dimension`]s can't be converted to the target [`Dimension`].
#[derive(Debug, Clone, PartialEq)]
pub struct UnconvertableDimensionsError {
    base_dimensions: Vec<Dimension>,
    target_dimension: Dimension,
    bareiss_eliminator_error: BareissEliminatorError,
}
impl Display for UnconvertableDimensionsError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Self { base_dimensions, target_dimension, bareiss_eliminator_error } = self;
        write!(f, "Couldn't convert from {} to {target_dimension}. {bareiss_eliminator_error}", Dimensions(base_dimensions))
    }
}
impl Error for UnconvertableDimensionsError {}

/// The SI decimal prefixes with exact powers of ten.
pub enum Prefix {
    /// 10^30 `1e30`.
    Quetta,
    /// 10^27 `1e27`.
    Ronna,
    /// 10^24 `1e24`.
    Yotta,
    /// 10^21 `1e21`.
    Zetta,
    /// 10^18 `1e18`.
    Exa,
    /// 10^15 `1e15`.
    Peta,
    /// 10^12 `1e12`.
    Tera,
    /// 10^9 `1e9`.
    Giga,
    /// 10^6 `1e6`.
    Mega,
    /// 10^3 `1e3`.
    Kilo,
    /// 10^2 `1e2`.
    Hecto,
    /// 10^1 `1e1`.
    Deca,
    /// 10^-1 `1e-1`.
    Deci,
    /// 10^-2 `1e-2`.
    Centi,
    /// 10^-3 `1e-3`.
    Milli,
    /// 10^-6 `1e-6`.
    Micro,
    /// 10^-9 `1e-9`.
    Nano,
    /// 10^-12 `1e-12`.
    Pico,
    /// 10^-15 `1e-15`.
    Femto,
    /// 10^-18 `1e-18`.
    Atto,
    /// 10^-21 `1e-21`.
    Zepto,
    /// 10^-24 `1e-24`.
    Yocto,
    /// 10^-27 `1e-27`.
    Ronto,
    /// 10^-30 `1e-30`.
    Quecto,
}

/// Represents a physical derived dimension (e.g., length, acceleration, psi).
/// A [`Dimension`] combines a scalar `scaling_factor` and a set of unordered, unnormalized named `exponents`.
#[derive(Debug, Clone)]
pub struct Dimension {
    scaling_factor: f64,
    exponents: Box<[(String, f64)]>,
}
impl Dimension {
    /// Returns the [`Dimension`]s `scaling_factor`.
    #[must_use]
    pub const fn scaling_factor(&self) -> &f64 {
        &self.scaling_factor
    }
    /// Returns the [`Dimension`]s base exponents.
    #[must_use]
    pub const fn exponents(&self) -> &[(String, f64)] {
        &self.exponents
    }
    /// Creates a new base [`Dimension`] with a single exponent equal to `1.0` and a `scaling_factor` of `1.0`.
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            scaling_factor: 1.0,
            exponents: [(name.to_string(), 1.0)].into()
        }
    }

    /// Raises the [`Dimension`] to an arbitrary power.
    #[must_use]
    pub fn power<T: Into<f64> + Copy>(&self, exponent: T) -> Self {
        Self {
            scaling_factor: self.scaling_factor.powf(exponent.into()),
            exponents: self.exponents.iter().map(|current_exponent| {
                let scaled = current_exponent.1 * exponent.into();
                (current_exponent.0.clone(), if scaled.abs() < 1e-12 { 0.0 } else { scaled })
            }).collect(),
        }
    }
    /// Returns the cube of the [`Dimension`].
    #[must_use]
    pub fn cube(&self) -> Self {
        self.power(3)
    }
    /// Returns the square of the [`Dimension`].
    #[must_use]
    pub fn square(&self) -> Self {
        self.power(2)
    }
    /// Returns the square root of the [`Dimension`].
    #[must_use]
    pub fn square_root(&self) -> Self {
        self.power(0.5)
    }
    /// Returns the multiplicative inverse (reciprocal) of the [`Dimension`].
    #[must_use]
    pub fn inverse(&self) -> Self {
        self.power(-1)
    }

    /// Returns a portion of the [`Dimension`] divided by the given scalar.
    #[must_use]
    pub fn portion<T: Into<f64>>(&self, divisor: T) -> Self {
        Self { scaling_factor: self.scaling_factor / divisor.into(), ..self.clone() }
    }
    /// Returns a scaled version of the [`Dimension`] multiplied by the given scalar.
    #[must_use]
    pub fn scale<T: Into<f64>>(&self, scaling_factor: T) -> Self {
        Self { scaling_factor: self.scaling_factor * scaling_factor.into(), ..self.clone() }
    }
    /// Applies a metric prefix.
    #[must_use]
    pub fn prefix(&self, prefix: &Prefix) -> Self {
        self.scale(match prefix {
            Prefix::Quetta=> 1e30,
            Prefix::Ronna => 1e27,
            Prefix::Yotta => 1e24,
            Prefix::Zetta => 1e21,
            Prefix::Exa   => 1e18,
            Prefix::Peta  => 1e15,
            Prefix::Tera  => 1e12,
            Prefix::Giga  => 1e9,
            Prefix::Mega  => 1e6,
            Prefix::Kilo  => 1e3,
            Prefix::Hecto => 1e2,
            Prefix::Deca  => 1e1,
            Prefix::Deci  => 1e-1,
            Prefix::Centi => 1e-2,
            Prefix::Milli => 1e-3,
            Prefix::Micro => 1e-6,
            Prefix::Nano  => 1e-9,
            Prefix::Pico  => 1e-12,
            Prefix::Femto => 1e-15,
            Prefix::Atto  => 1e-18,
            Prefix::Zepto => 1e-21,
            Prefix::Yocto => 1e-24,
            Prefix::Ronto => 1e-27,
            Prefix::Quecto=> 1e-30,
        })
    }
    /// Computes the exponent ratio needed to convert `self` to `other`.
    /// # Errors
    /// - [`ConversionExponentError::NoNonZeroExponentPair`] when there is not a pair with both of the numbers not being zero.
    /// - [`ConversionExponentError::InconsistentExponentRatio`] when there is a pair with a different exponent ratio from the first.
    pub fn get_conversion_exponent(&self, other: &Self) -> Result<f64, ConversionExponentError> {
        let exponents = [self, other].exponents();
        let mut unit_iter = exponents[0].iter().zip(exponents[1].iter());
        let exponent: f64 = loop {
            match unit_iter.next() {
                Some((left, right)) => {
                    if left.abs() > f64::from(f32::EPSILON) && right.abs() > f64::from(f32::EPSILON) {
                        break right / left
                    }
                }
                None => {
                    return Err(ConversionExponentError::NoNonZeroExponentPair {
                        left_dimension: self.clone(),
                        right_dimension: other.clone()
                    });
                }
            }
        };
        for (&left, &right) in unit_iter {
            if left.abs() > f64::from(f32::EPSILON) && right != 0.0 && (dbg!(right / left / exponent) - 1.).abs() > f64::from(f32::EPSILON) {
                return Err(ConversionExponentError::InconsistentExponentRatio {
                    left_dimension: self.clone(),
                    right_dimension: other.clone(),
                    right,
                    left,
                    expected: exponent,
                    found: right / left
                });
            }
        };
        Ok(exponent)
    }
}
impl PartialEq for Dimension {
    fn eq(&self, other: &Self) -> bool {
        let exponents = [self, other].exponents();
        (exponents[0] == exponents[1]) && (self.scaling_factor / other.scaling_factor - 1.0).abs() < (f64::from(f32::EPSILON))
    }
}

/// A helper struct to show multiple [`Dimension`]s is a concise manner.
pub struct Dimensions<'a>(pub &'a Vec<Dimension>);
impl Display for Dimensions<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self
            .0
            .iter()
            .map(|d| format!("{d}"))
            .collect::<Vec<_>>()
            .join(", ")
        )
    }
}


#[cfg(feature = "sci-notation")]
#[macro_export]
/// A helper macro to show the numbers in scientific notation or not.
macro_rules! uwrite {
    ($f:expr, $fmt:literal, $($arg:expr),* $(,)?) => {
        write!($f, $fmt, $(format!("{:e}", $arg)),*)
    };
}
#[cfg(not(feature = "sci-notation"))]
#[macro_export]
/// A helper macro to show the numbers in scientific notation or not.
macro_rules! uwrite {
    ($f:expr, $fmt:literal, $($arg:expr),* $(,)?) => {
        write!($f, $fmt, $($arg),*)
    };
}
impl Display for Dimension {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if (self.scaling_factor - 1.0).abs() > f64::from(f32::EPSILON) {
            uwrite!(f, "* {} ", self.scaling_factor)?;
        }
        write!(f, "{}", self
            .exponents
            .iter()
            .filter(|(_, unit)| unit.abs() > f64::from(f32::EPSILON))
            .map(|(name, unit)| {
                if (unit - 1.0).abs() < f64::from(f32::EPSILON) {
                    name.to_string()
                } else {
                    format!("{name}^{unit}")
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
        )
    }
}
impl Mul for &Dimension {
    type Output = Dimension;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut exponents = self.exponents.clone().into_vec();
        for (name, exponent) in &rhs.exponents {
            if let Some((_, existing_exp)) =
                exponents.iter_mut().find(|(n, _)| n == name)
            {
                *existing_exp += exponent;
            } else {
                exponents.push((name.clone(), *exponent));
            }
        }
        Self::Output {
            scaling_factor: self.scaling_factor * rhs.scaling_factor,
            exponents: exponents.into(),
        }
    }
}
impl Div for &Dimension {
    type Output = Dimension;
    fn div(self, rhs: Self) -> Self::Output {
        let mut exponents = self.exponents.clone().into_vec();
        for (name, exponent) in &rhs.exponents {
            if let Some((_, existing_exp)) =
                exponents.iter_mut().find(|(n, _)| n == name)
            {
                *existing_exp -= exponent;
            } else {
                exponents.push((name.clone(), -*exponent));
            }
        }
        Self::Output {
            scaling_factor: self.scaling_factor / rhs.scaling_factor,
            exponents: exponents.into(),
        }
    }
}
impl Product for Dimension {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(DIMENSIONLESS.clone(), |acc, x| &acc * &x)
    }
}
/// Provides the capability to use constant base dimensions with the rest of the API.
pub trait DeepDereferrenceable {
    /// Converts to a usable format for the rest of the API.
    fn get(&self) -> Box<[&Dimension]>;
}
impl DeepDereferrenceable for LazyLock<Box<[&LazyLock<Dimension>]>> {
    fn get(&self) -> Box<[&Dimension]> {
        self.iter().map(|&dimension| &**dimension).collect()
    }
}

/// Provides the capability to analysis capabilities for [`Dimension`]s.
pub trait DimensionalAnalysable {
    /// Processes the exponents for them to be comparable.
    fn exponents(&self) -> Box<[Box<[f64]>]>;
    /// Returns wether all [`Dimension`]s' exponents are equal.
    fn have_same_exponents(&self) -> bool;
    /// Computes the exponents required for each [`Dimension`] in order to be converted to the target [`Dimension`].
    /// # Errors
    /// [`UnconvertableDimensionsError`] when the base Dimensions can't be converted to the target [`Dimension`].
    fn exponents_to(&self, other: &Dimension) -> Result<Box<[f64]>, UnconvertableDimensionsError>;
    /// Computes all the exponents required for each [`Dimension`] in order to be converted to each target [`Dimension`] separately.
    /// # Errors
    /// [`UnconvertableDimensionsError`] when the base Dimensions can't be converted to the target [`Dimension`]s.
    fn all_exponents_to(&self, others: &[&Dimension]) -> Result<Box<[Box<[f64]>]>, UnconvertableDimensionsError>;
    /// Returns the combined [`Dimension`] obtained by multiplying each [`Dimension`] by its corresponding power in `others`.
    fn product_of_powers(&self, others: &[f64]) -> Dimension;
    /// Uses `exponents_to` to figure out if a set of [`Dimension`]s is coherent by covnerting them to each of the 7 base SI units (M, L, T, I, Θ, N, J)
    /// # Errors
    /// [`UnconvertableDimensionsError`] when the base Dimensions can't be converted to the seven base SI units.
    fn coherent_system7(&self) -> Result<Box<[Box<[f64]>]>, UnconvertableDimensionsError>;
    /// Uses `exponents_to` to figure out if a set of [`Dimension`]s is coherent by covnerting them to 5 base SI units (M, L, T, I, Θ)
    /// # Errors
    /// [`UnconvertableDimensionsError`] when the base Dimensions can't be converted to the five base SI units.
    fn coherent_system5(&self) -> Result<Box<[Box<[f64]>]>, UnconvertableDimensionsError>;
    /// Uses `exponents_to` to figure out if a set of [`Dimension`]s is coherent by covnerting them to 7 base SI units (M, L, T)
    /// # Errors
    /// [`UnconvertableDimensionsError`] when the base Dimensions can't be converted to the three base SI units.
    fn coherent_system3(&self) -> Result<Box<[Box<[f64]>]>, UnconvertableDimensionsError>;
}
use crate::dimensions::le_systeme_international_d_unites::base_units::{AMPERE, CANDELA, KELVIN, KILOGRAM, METER, MOLE, SECOND};
use crate::{dimension};

impl DimensionalAnalysable for [&Dimension] {
    fn exponents(&self) -> Box<[Box<[f64]>]> {
        let mut exponent_names: Vec<&str> = Vec::new();
        let mut exponent_matrix: Vec<Vec<f64>> = Vec::new();
        let mut width: usize = 0;
        for (row, exponents) in self.iter().map(|dimension| &dimension.exponents).enumerate() {
            exponent_matrix.push(vec![0.0; width]);
            for (name, exponent) in exponents {
                if let Some(index) =
                    exponent_names.iter().position(|n| n == name)
                {
                    exponent_matrix[row][index] = *exponent;
                } else {
                    for exponent_row in &mut exponent_matrix[0..row] {
                        exponent_row.push(0.0);
                    }
                    exponent_matrix[row].push(*exponent);
                    exponent_names.push(name);
                    width += 1;
                }
            }
        }
        exponent_matrix.into_iter().map(Into::into).collect()
    }
    fn have_same_exponents(&self) -> bool {
        if self.is_empty() {
            return true
        }
        let all_exponents = self.exponents();
        let first_exponents = &all_exponents[0];
        all_exponents.iter().all(|exponents| exponents == first_exponents)
    }
    fn exponents_to(&self, other: &Dimension) -> Result<Box<[f64]>, UnconvertableDimensionsError> {
        let dimension: Box<[&Dimension]> = self.iter().chain(once(&other)).copied().collect();
        let rows: &[Box<[f64]>] = &dimension.exponents();
        macro_rules! unconvertable_dimensions_error {
            () => {
                |error|
                UnconvertableDimensionsError {
                    base_dimensions: self.iter().copied().cloned().collect(),
                    target_dimension: other.clone(),
                    bareiss_eliminator_error: error.into(),
                }
            };
        }
        RectangularMatrix::try_from(rows)
            .map_err(unconvertable_dimensions_error!())?
            .switch_dimensions()
            .bareiss_solve()
            .map_err(unconvertable_dimensions_error!())
    }
    fn all_exponents_to(&self, others: &[&Dimension]) -> Result<Box<[Box<[f64]>]>, UnconvertableDimensionsError> {
        others.iter().map(|other| self.exponents_to(other)).collect()
    }
    fn product_of_powers(&self, rows: &[f64]) -> Dimension {
        rows.iter().enumerate().map(|(index, &power)| self[index].power(power)).product()
    }
    fn coherent_system7(&self) -> Result<Box<[Box<[f64]>]>, UnconvertableDimensionsError> {
        self.all_exponents_to(&[&KILOGRAM, &METER, &SECOND, &AMPERE, &KELVIN, &MOLE, &CANDELA])
    }
    fn coherent_system5(&self) -> Result<Box<[Box<[f64]>]>, UnconvertableDimensionsError> {
        self.all_exponents_to(&[&KILOGRAM, &METER, &SECOND, &AMPERE, &KELVIN])
    }
    fn coherent_system3(&self) -> Result<Box<[Box<[f64]>]>, UnconvertableDimensionsError> {
        self.all_exponents_to(&[&KILOGRAM, &METER, &SECOND])
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        debug_println,
        {dimension::Prefix::Kilo, dimensions::{
            centimeter_gram_second_units::base_units::GRAM, le_systeme_international_d_unites::base_units::{
                KILOGRAM, METER, SECOND
            }
        }}
    };

    #[test]
    fn meters_per_second_squared_comma_meters_per_second_per_second_and_meters_per_second_seconds() {
        let lhs = &*METER / &SECOND.square();
        let mhs = &(&*METER / &*SECOND) / &*SECOND;
        let rhs = &*METER / &(&*SECOND * &*SECOND);
        debug_println!("lhs:{}", lhs);
        debug_println!("mhs:{}", mhs);
        debug_println!("rhs:{}", rhs);
        assert_eq!(lhs, mhs);
        assert_eq!(mhs, rhs);
    }

    #[test]
    fn kilograms_and_grams() {
        let lhs = &*KILOGRAM;
        let rhs = &GRAM.prefix(&Kilo);
        debug_println!("lhs:{}", lhs);
        debug_println!("rhs:{}", rhs);
        assert_eq!(lhs, rhs);
    }
}


/// Returns the product of each [`Dimension`] optionally prefixed prefixed and then optionally raised to a power
#[macro_export]
macro_rules! product_of_powers {
    (
        $( ;$scaling_factor:expr; )?
        $( ,$divisor:expr, )? ->
        $( $( ,$prefix:ident )? $rest:ident $( ^$rest_exp:literal )? )*
    ) => {{
        use std::ops::Mul;
        $crate::dimension::DIMENSIONLESS
        $(
            .mul(
                &$rest
                    $(.prefix(&$crate::dimension::Prefix::$prefix))?
                    $(.power($rest_exp))?
            )
        )*
        $(.scale($scaling_factor))?
        $(.portion($divisor))?
    }};
}

#[macro_export]
/// Reorganizes [`product_of_powers`] to be more easily used inside code
macro_rules! dim {
    (
        $( ;$scaling_factor:expr; )?
        $( ,$divisor:expr, )?
        $( $( ,$prefix:ident )? $rest:ident $( ^$rest_exp:literal )? )*
    ) => {
        &$crate::product_of_powers!(
            $( ;$scaling_factor; )?
            $( ,$divisor, )? ->
            $( $( ,$prefix )? $rest $( ^$rest_exp )? )*
        )
    };
}

#[macro_export]
/// Creates a `static` [`LazyLock`][`Dimension`]
macro_rules! dimension {
    ($name:ident $($doc:literal)?) => {
        dimension!($name = $crate::dimension::Dimension::new(&stringify!($name).to_lowercase()) $(=> $doc)?);
    };
    (
        $( ,$divisor:expr, )?
        $name:ident =
        $( ;$scaling_factor:expr; )?
        $( $( ,$prefix:ident )? $rest:ident $( ^$rest_exp:literal )? )*
        $($doc:literal)?
    ) => {
        dimension!(
            $name =
            $crate::product_of_powers!(
                $( ;$scaling_factor; )?
                $( ,$divisor, )? ->
                $( $( ,$prefix )? $rest $( ^$rest_exp )? )*
            )
            $(=> $doc)?
        );
    };
    ($name:ident = $unit:expr $(=> $doc:literal)?) => {
        $(#[doc=$doc])?
        #[allow(unused)]
        pub static $name: std::sync::LazyLock<$crate::dimension::Dimension> = std::sync::LazyLock::new(|| $unit);
    };
}

dimension!(DIMENSIONLESS = Dimension {
    scaling_factor: 1.0,
    exponents: Vec::new().into(),
} => "The [`Dimension`] of a plain number");