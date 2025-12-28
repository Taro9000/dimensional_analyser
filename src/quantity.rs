//! Provides dimensional arithmetic and conversion between physical [`Quantity`]'s
//! expressed in arbitrary units. Built on top of [`Dimension`].
use crate::{debug_println};
use crate::dimension::{ConversionExponentError, DIMENSIONLESS, Dimension, UnconvertableDimensionsError, DimensionalAnalysable};
use core::fmt;
use std::fmt::{Display, Formatter, LowerExp};
use std::iter::Product;
use std::ops::{Add, Div, Mul, Sub};
use std::error::Error;


/// Error when a single [`Quantity`] can't be converted to a [`Dimension`].
#[derive(Debug, Clone, PartialEq)]
pub struct UnconvertableQuantityError {
    base_quantity: Quantity,
    conversion_exponent_error: ConversionExponentError,
}
impl Display for UnconvertableQuantityError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Self { base_quantity, conversion_exponent_error } = self;
        write!(f, "Failed to convert {base_quantity}. {conversion_exponent_error}")
    }
}
impl Error for UnconvertableQuantityError {}

/// Error when multiple [`Quantity`]s can't be converted to a [`Dimension`].
#[derive(Debug, Clone, PartialEq)]
pub struct UnconvertableQuantitiesError {
    base_quantities: Vec<Quantity>,
    dimension_error: UnconvertableDimensionsError,
}
impl Display for UnconvertableQuantitiesError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Self { base_quantities, dimension_error } = self;
        write!(f, "Failed to convert {}. {dimension_error}", Quantities(base_quantities))
    }
}
impl Error for UnconvertableQuantitiesError {}


/// Error when both [`Dimension`]s are incompatible.
#[derive(Debug, Clone, PartialEq)]
pub struct DifferentDimensionError {
    left_dimension: Dimension,
    right_dimension: Dimension,
}
impl Display for DifferentDimensionError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Self { left_dimension, right_dimension } = self;
        write!(f, "Uncompatible dimensions: {left_dimension} and {right_dimension}")
    }
}
impl Error for DifferentDimensionError {}

/// Distinguishes the different ways [`Quantity`]s are related to one another.
#[derive(PartialEq, Debug)]
pub enum Equality {
    /// Same value and same dimension: `2 s = 2 s`.
    Identical,
    /// Equivalent multiple or submultiple: `120 s = 2 min`.
    ScalarMultiple(f64),
    /// Analogous implied by raising to a power: `2 s = 0.5` Hz or `10 m = 100 m^2`.
    PowerProyection(f64),
    /// None of the above.
    Different,
}

/// Represents a physical quantity consisting of a scalar value and a [`Dimension`].
/// # Example
/// ```rust
/// use dimensional_analyser::{quantity::Quantity, dimensions::le_systeme_international_d_unites::{JOULE, base_units::{KILOGRAM, METER, SECOND}}};
///
/// let mass = Quantity::new(5, &KILOGRAM);
/// let velocity = Quantity::new(10, &(&*METER / &*SECOND));
/// let kinetic_energy = (&mass * &velocity.power(2)) / 2;
/// assert_eq!(kinetic_energy, Quantity::new(250, &JOULE));
/// ```
#[derive(Debug, Clone)]
pub struct Quantity {
    value: f64,
    dimension: Dimension,
}
impl Quantity {
    /// Creates a new [`Quantity`] with the given `value` and [`Dimension`].
    pub fn new<T: Into<f64>>(value: T, dimension: &Dimension) -> Self {
        Self { value: value.into(), dimension: dimension.clone() }
    }
    /// Raises the quantity to the specified power.
    #[must_use]
    pub fn power<T: Into<f64> + Copy>(&self, exponent: T) -> Self {
        Self {
            value: self.value.powf(exponent.into()),
            dimension: self.dimension.power(exponent),
        }
    }
    /// Attempts to convert the quantity to another compatible dimension.
    /// # Errors
    /// [`UnconvertableQuantityError`] when the base [`Quantity`] can't be converted to the target [`Dimension`]
    /// # Example
    /// ```rust
    /// use dimensional_analyser::{dimension::Prefix::Centi, quantity::Quantity, dimensions::le_systeme_international_d_unites::base_units::METER};
    ///
    /// let one_square_centimeter = Quantity::new(1, &METER.prefix(&Centi).square());
    /// let area_in_square_meters = one_square_centimeter.convert_to(&METER.square()).unwrap();
    /// assert_eq!(area_in_square_meters, Quantity::new(0.0001, &METER.square()));
    /// ```
    pub fn convert_to(&self, other: &Dimension) -> Result<Self, UnconvertableQuantityError> {
        Ok(Self {
            value: (self.value * self.dimension.scaling_factor()).powf(
                self.dimension.get_conversion_exponent(other).map_err(|conversion_exponent_error|
                    UnconvertableQuantityError { base_quantity: self.clone(), conversion_exponent_error }
                )?
            ) / other.scaling_factor(),
            dimension: other.clone(),
        })
    }
    /// Returns the relationship between both [`Quantity`]'s.
    /// # Panics
    /// Not enough data is known after converting `self` to the `other.dimension` so an expect is used
    /// # Example
    /// ```rust
    /// use dimensional_analyser::{quantity::Quantity, dimensions::le_systeme_international_d_unites::{MINUTE, base_units::SECOND}, quantity::Equality};
    ///
    /// let one_minute = Quantity::new(1, &MINUTE);
    /// let sixty_seconds = Quantity::new(60, &SECOND);
    /// match one_minute.get_equality_with(&sixty_seconds) {
    ///     Equality::ScalarMultiple(factor) => assert_eq!(factor, 60.0),
    ///     _ => panic!("1 min and 60 s should be scalar multiples"),
    /// }
    /// ```
    #[must_use]
    pub fn get_equality_with(&self, other: &Self) -> Equality {
        debug_println!("Comparing {} and {}", self, other);
        if self == other {
            return Equality::Identical;
        }
        self.convert_to(&other.dimension).map_or(Equality::Different, |converted| {
            debug_println!("Converted to: {}", converted);
            if &converted != other {
                Equality::Different
            } else if [&self.dimension, &other.dimension].have_same_exponents() {
                Equality::ScalarMultiple(self.dimension.scaling_factor() / other.dimension.scaling_factor())
            } else {
                let exponent = self.dimension.get_conversion_exponent(&other.dimension).expect("Should have an exponent if we got here");
                Equality::PowerProyection(exponent)
            }
        })
    }
    /// Helper function to print how both [`Quantity`]'s are related.
    pub fn show_comparizon_results_with(&self, other: &Self) {
        match self.get_equality_with(other) {
            Equality::Identical => {
                println!("{self} and {other} are identical");
            }
            Equality::ScalarMultiple(factor) => {
                println!("{self} and {other} are scalar multiples (factor {factor})");
            }
            Equality::PowerProyection(exponent) => {
                println!("{self} and {other} are power symmetric (exponent {exponent})");
            }
            Equality::Different => {
                println!("{self} and {other} are different dimensions");
            }
        }
    }
}
impl PartialEq for Quantity {
    fn eq(&self, other: &Self) -> bool {
        (self.dimension == other.dimension) && (self.value / other.value - 1.0).abs() < f64::from(f32::EPSILON)
    }
}
/// A helper struct to show multiple [`Quantity`]'s is a concise manner.
pub struct Quantities<'a>(pub &'a Vec<Quantity>);
impl Display for Quantities<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[{}]", self
            .0
            .iter()
            .map(|d| format!("{d}"))
            .collect::<Vec<_>>()
            .join(", ")
        )
    }
}
impl LowerExp for Quantity {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let value = self.value;
        let dimension = &self.dimension;
        write!(f, "{value:e}[{dimension:e}]")
    }
}
impl Display for Quantity {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let value = self.value;
        let dimension = &self.dimension;
        write!(f, "{value}[{dimension}]")
    }
}
impl Mul for &Quantity {
    type Output = Quantity;
    fn mul(self, rhs: Self) -> Self::Output {
        Quantity {
            value: self.value * rhs.value,
            dimension: &self.dimension * &rhs.dimension,
        }
    }
}
impl<T: Into<f64>> Mul<T> for Quantity {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            value: self.value * rhs.into(),
            dimension: self.dimension,
        }
    }
}
impl Div for &Quantity {
    type Output = Quantity;
    fn div(self, rhs: Self) -> Self::Output {
        Quantity {
            value: self.value / rhs.value,
            dimension: &self.dimension / &rhs.dimension,
        }
    }
}
impl<T: Into<f64>> Div<T> for Quantity {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self {
            value: self.value / rhs.into(),
            dimension: self.dimension,
        }
    }
}
impl Add for &Quantity {
    type Output = Result<Quantity, DifferentDimensionError>;
    fn add(self, rhs: Self) -> Self::Output {
        let exponents = [&self.dimension, &rhs.dimension].exponents();
        if exponents[0] != exponents[1] {
            return Err(DifferentDimensionError {
                left_dimension: self.dimension.clone(),
                right_dimension: rhs.dimension.clone(),
            })
        }
        Ok(Quantity {
            value: self.value + rhs.value,
            dimension: self.dimension.clone(),
        })
    }
}
impl Sub for &Quantity {
    type Output = Result<Quantity, DifferentDimensionError>;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.dimension != rhs.dimension {
            return Err(DifferentDimensionError {
                left_dimension: self.dimension.clone(),
                right_dimension: rhs.dimension.clone(),
            })
        }
        Ok(Quantity {
            value: self.value - rhs.value,
            dimension: self.dimension.clone(),
        })
    }
}
impl Product for Quantity {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        
        iter.fold(Self::new(1.0, &DIMENSIONLESS), |acc, x| &acc * &x)
    }
}

/// Provides dimensional analysis capabilities for [`Quantity`]'s.
pub trait DimensionalAnalysableQuantity {
    /// Converts [`Quantity`]'s to a [`Dimension`].
    /// # Errors
    /// [`UnconvertableQuantitiesError`] when the [`Quantity`]s can't be converted to a [`Dimension`] 
    fn convert_to(&self, other: &Dimension) -> Result<Quantity, UnconvertableQuantitiesError>;
    /// Converts [`Quantity`]'s to each [`Dimension`] separately.
    /// # Errors
    /// [`UnconvertableQuantitiesError`] when the [`Quantity`]s can't be converted to each [`Dimension`] 
    fn convertable_to(&self, others: &[&Dimension]) -> Result<Box<[Quantity]>, UnconvertableQuantitiesError>;
}
impl DimensionalAnalysableQuantity for [&Quantity] {
    fn convert_to(&self, other: &Dimension) -> Result<Quantity, UnconvertableQuantitiesError> {
        let quantities: Box<[&Dimension]> = self.iter().map(|quantity| &quantity.dimension).collect();
        let same_units: Quantity = quantities.exponents_to(other).map_err(
            |dimension_error|UnconvertableQuantitiesError {
                base_quantities: self.iter().copied().cloned().collect(), dimension_error
            }
        )?.iter().enumerate().map(|(index, &power)| self[index].power(power)).product();
        Ok(Quantity{
            value: same_units.value * same_units.dimension.scaling_factor() / other.scaling_factor(),
            dimension: other.clone()
        })
    }
    fn convertable_to(&self, others: &[&Dimension]) -> Result<Box<[Quantity]>, UnconvertableQuantitiesError> {
        others.iter().map(|other| self.convert_to(other)).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        debug_println, dim, dimension::Prefix::{
                Hecto,
                Kilo,
                Milli,
                Pico
            }, dimensions::{le_systeme_international_d_unites::{
                    HERTZ, HOUR, JOULE, MINUTE, base_units::{
                        KILOGRAM,
                        METER, SECOND
                    }
                }, the_seven_c_s::base_units::C_AS_THE_SPEED_OF_LIGHT}, quantity::*
    };

    #[test]
    fn test_add() {
        let lhs = Quantity::new(120, &SECOND);
        let rhs = Quantity::new(2, &MINUTE).convert_to(&lhs.dimension).expect("Seconds and minutes are compatible");
        let sum = &lhs + &rhs;
        assert!(sum.is_ok());
        let sum = sum.unwrap();
        assert_eq!(sum, Quantity::new(240, &SECOND));
    }

    // The following tests remain in the unit test suite. A number of
    // illustrative examples were moved to doctests in the module header
    // to improve the crate documentation and reduce bloat in this test
    // module.

    #[test]
    #[allow(clippy::float_cmp)]
    fn curseder_units_optic_fiber_example() {
        let pulse_broadening = Quantity::new(1.2, &(&SECOND.prefix(&Pico) / &METER.prefix(&Kilo).power(0.5)));
        let propagation_distance = Quantity::new(100, &(METER.prefix(&Kilo)));

        let total_spread = &pulse_broadening * &propagation_distance.power(0.5);
        debug_println!("Total pulse spread (in picoseconds): {}", total_spread);
        assert_eq!(total_spread.value, 12.0);
    }

    #[test]
    fn bomb_explosion_radius_example() {
        let energy = Quantity::new(100_000, &JOULE);
        let explosion_time = Quantity::new(1, &SECOND);
        let air_density = Quantity::new(1, &(&*KILOGRAM / &METER.cube()));
        let radius = (&(&energy / &air_density) * &explosion_time.power(2.0)).convert_to(&METER).expect("Resulting dimension should be length");
        debug_println!("Estimated explosion radius (in meters): {}", radius);
        assert!((radius.value - 10.0).abs() < 1.0);
    } 

    #[test]
    #[allow(clippy::float_cmp)]
    fn complex_equalty_example() {
        let hectareas = Quantity::new(100, &METER.prefix(&Hecto).square());
        let length = Quantity::new(1_000_000, &METER.prefix(&Milli));
        match length.get_equality_with(&hectareas) {
            Equality::PowerProyection(exponent) => {
                assert_eq!(exponent, 2.0);
            }
            _ => {
                panic!("100 hectareas and 1,000,000 millimeters should be power symmetric");
            }
        }
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn another_contrived_example() {
        let frequency = Quantity::new(2, &HERTZ.prefix(&Kilo));
        let period = Quantity::new(0.5, &SECOND.prefix(&Milli)).power(2);
        match frequency.get_equality_with(&period) {
            Equality::PowerProyection(exponent) => {
                assert_eq!(exponent, -2.0);
            }
            _ => {
                panic!("2 kHz and 0.25 ms^2 should be power symmetric");
            }
        }
    }
    
    #[test]
    fn incompatible_addition_example() {
        let length = Quantity::new(1, &METER);
        let time = Quantity::new(1, &SECOND);
        let result = &length + &time;
        assert!(result.is_err());
        debug_println!("Error message: {}", result.err().unwrap());
    }
    
    #[test]
    fn incompatible_conversion_example() {
        let length = Quantity::new(1, &(&*METER / &*SECOND));
        let time = &*SECOND;
        let result = length.convert_to(time);
        assert!(result.is_err());
        debug_println!("Error message: {}", result.err().unwrap());
    }

    #[test]
    fn bomb_explosion_radius_example_as_dimensional_analysis() {
        let energy = Quantity::new(100_000, &JOULE);
        let explosion_time = Quantity::new(1, &SECOND);
        let air_density = Quantity::new(1, &(&*KILOGRAM / &METER.power(3)));
        let radius = [&energy, &explosion_time, &air_density].convert_to(&METER).expect("Units to be convertible");
        debug_println!("Estimated explosion radius (in meters): {}", radius);
        assert!((radius.value - 10.0).abs() < 1.0);
    } 

    #[test]
    fn unordered_but_equal() {
        let letter = Dimension::new("letter");
        let minute = &*MINUTE;
        let typing_speed_a = Quantity::new(24, &(&letter * &minute.inverse()));
        let typing_speed_b = Quantity::new(24, &(&minute.inverse() * &letter));
        assert_eq!(typing_speed_a, typing_speed_b);
    }

    #[test]
    fn unordered_but_identical() {
        let letter = Dimension::new("letter");
        let minute = &*MINUTE;
        let typing_speed_a = Quantity::new(24, &(&letter * &minute.inverse()));
        let typing_speed_b = Quantity::new(24, &(&minute.inverse() * &letter));
        assert_eq!(typing_speed_a.get_equality_with(&typing_speed_b), Equality::Identical);
    }

    #[test]
    fn unordered_scalar_multiples() {
        let letter = Dimension::new("letter");
        let word = letter.scale(5);
        let minute = &*MINUTE;
        let typing_speed_a = Quantity::new(24, &(&word * &minute.inverse()));
        let typing_speed_b = Quantity::new(120, &(&minute.inverse() * &letter));
        assert_eq!(typing_speed_a.get_equality_with(&typing_speed_b), Equality::ScalarMultiple(5.0));
    }

    #[test]
    fn unordered_power_proyections() {
        let letter = Dimension::new("letter");
        let word = letter.scale(5);
        let minute = &*MINUTE;
        let typing_speed_a = Quantity::new(24, &(&word * &minute.inverse()));
        let typing_speed_b = typing_speed_a.power(0.3);
        assert_eq!(typing_speed_a.get_equality_with(&typing_speed_b), Equality::PowerProyection(0.3));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn conversion_with_different_multipliers() {
        let dollar = Dimension::new("dollar");
        let money_gained = Quantity::new(40, &dollar);
        let match_duration = Quantity::new(7, &MINUTE);
        let salary = [&money_gained, &match_duration].convert_to(&(&dollar / &*HOUR)).expect("Convertable");
        assert_eq!(salary.dimension, &dollar / &*HOUR);
        assert_eq!(salary.value, 3.428_571_428_571_428e2);
    }

    /*
    "frecuencia"	"velocidad de la luz"	    "longitud de onda"
    540[Tera.HERTZ]	1[C_AS_THE_SPEED_OF_LIGHT]	[A2, B2] => nano .meter
    */
    #[test]
    fn wavelength_from_frequency_and_the_speed_of_light() {
        let frequency = Quantity::new(540, dim!(,1,,Tera HERTZ));
        let speed_of_light = Quantity::new(1, dim!(C_AS_THE_SPEED_OF_LIGHT));
        let length = dim!(,1,,Nano METER);
        let wavelength = [&frequency, &speed_of_light]
            .convert_to(length)
            .expect("Should be convertable");
        assert_eq!(wavelength, Quantity::new(555.171_218_518_518_6, length));
    }
}