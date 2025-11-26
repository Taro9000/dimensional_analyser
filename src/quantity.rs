//! Provides dimensional arithmetic and conversion between physical [`Quantity`]'s
//! expressed in arbitrary units. Built on top of [`Dimension`].
use crate::{debug_println, uwrite};
use crate::dimension::{Dimension, DimensionError, DimensionalAnalysable, DIMENSIONLESS};
use core::fmt;
use std::f32::EPSILON;
use std::fmt::{Display, Formatter};
use std::iter::Product;
use std::ops::{Add, Div, Mul, Sub};
use std::error::Error;


/// A detailed breakdown of the error occurred when handling [`Quantity`]'s.
#[derive(Debug, Clone, PartialEq)]
pub enum QuantityError {
    /// Error when trying to convert a single quantity to a [`Dimension`].
    UnconvertableQuantityError {
        /// The quantity that was tried to convert.
        base_quantity: Quantity,
        /// The rest of the error details.
        dimension_error: DimensionError,
    },
    /// Error when trying to convert multiple [`Quantity`]'s to a [`Dimension`].
    UnconvertableQuantitiesError {
        /// The [`Quantity`]'s that were tried to convert.
        base_quantities: Vec<Quantity>,
        /// The rest of the error details.
        dimension_error: DimensionError,
    },
    /// Error when both [`Dimension`]'s are incompatible.
    DifferentDimensionError {
        /// The left hand side [`Dimension`].
        left_dimension: Dimension,
        /// The right hand side [`Dimension`].
        right_dimension: Dimension,
    },
}
impl Display for QuantityError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::DifferentDimensionError { left_dimension, right_dimension } =>
                write!(f, "Uncompatible dimensions: {left_dimension} and {right_dimension}"),
            Self::UnconvertableQuantityError { base_quantity, dimension_error } =>
                write!(f, "Failed to convert {base_quantity}. {dimension_error}"),
            Self::UnconvertableQuantitiesError { base_quantities, dimension_error } =>
                write!(f, "Failed to convert {}. {dimension_error}", Quantities(base_quantities)),
        }
    }
}
impl Error for QuantityError {}

/// The result of operations regarding [`Quantity`]'s.
pub type Result<T> = std::result::Result<T, QuantityError>;

/// Distinguishes the different ways [`Quantity`]'s are related to one another.
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
    pub fn power<T: Into<f64> + Copy>(&self, exponent: T) -> Self {
        Self {
            value: self.value.powf(exponent.into()),
            dimension: self.dimension.power(exponent),
        }
    }
    /// Attempts to convert the quantity to another compatible dimension.
    pub fn convert_to(&self, other: &Dimension) -> Result<Self> {
        match self.dimension.get_conversion_exponent(other) {
            Err(_) if self.dimension.exponents() != other.exponents() =>
                Err(QuantityError::DifferentDimensionError {
                    left_dimension: self.dimension.clone(),
                    right_dimension: other.clone(),
                }),
            Ok(conversion_exponent) =>
                Ok(Self {
                    value: (self.value * self.dimension.scaling_factor()).powf(conversion_exponent) / other.scaling_factor(),
                    dimension: other.clone(),
                }),
            Err(error) => Err(QuantityError::UnconvertableQuantityError { base_quantity: self.clone(), dimension_error: error })
        }
    }
    /// Returns the relationship between both [`Quantity`]'s.
    pub fn get_equality_with(&self, other: &Self) -> Equality {
        debug_println!("Comparing {} and {}", self, other);
        if self == other {
            return Equality::Identical;
        }
        match self.convert_to(&other.dimension) {
            Ok(converted) => {
                debug_println!("Converted to: {}", converted);
                if &converted != other {
                    return Equality::Different;
                } else if [&self.dimension, &other.dimension].have_same_exponents() {
                    return Equality::ScalarMultiple(self.dimension.scaling_factor() / other.dimension.scaling_factor());
                } else {
                    let exponent = self.dimension.get_conversion_exponent(&other.dimension).expect("Should have an exponent if we got here");
                    return Equality::PowerProyection(exponent);
                }
            }
            Err(_) => return Equality::Different
        }
    }
    /// Helper function to print how both [`Quantity`]'s are related.
    pub fn show_comparizon_results_with(&self, other: &Self) {
        match self.get_equality_with(other) {
            Equality::Identical => {
                println!("{} and {} are identical", self, other);
            }
            Equality::ScalarMultiple(factor) => {
                println!("{} and {} are scalar multiples (factor {})", self, other, factor);
            }
            Equality::PowerProyection(exponent) => {
                println!("{} and {} are power symmetric (exponent {})", self, other, exponent);
            }
            Equality::Different => {
                println!("{} and {} are different dimensions", self, other);
            }
        }
    }
}
impl PartialEq for Quantity {
    fn eq(&self, other: &Self) -> bool {
        (self.dimension == other.dimension) && (self.value / other.value - 1.0).abs() < (EPSILON as f64)
    }
}
/// A helper struct to show multiple [`Quantity`]'s is a concise manner.
pub struct Quantities<'a>(pub &'a Vec<Quantity>);
impl<'a> Display for Quantities<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[{}]", self
            .0
            .iter()
            .map(|d| format!("{}", d))
            .collect::<Vec<_>>()
            .join(", ")
        )
    }
}
impl Display for Quantity {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        uwrite!(f, "{}", self.value)?;
        write!(f, " {}", self.dimension)
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
    type Output = Quantity;
    fn mul(self, rhs: T) -> Self::Output {
        Quantity {
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
    type Output = Quantity;
    fn div(self, rhs: T) -> Self::Output {
        Quantity {
            value: self.value / rhs.into(),
            dimension: self.dimension,
        }
    }
}
impl Add for &Quantity {
    type Output = Result<Quantity>;
    fn add(self, rhs: Self) -> Self::Output {
        let exponents = [&self.dimension, &rhs.dimension].exponents();
        if exponents[0] != exponents[1] {
            return Err(QuantityError::DifferentDimensionError {
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
    type Output = Result<Quantity>;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.dimension != rhs.dimension {
            return Err(QuantityError::DifferentDimensionError {
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
        iter.fold(Quantity::new(1.0, &*DIMENSIONLESS), |acc, x| &acc * &x)
    }
}

/// Provides dimensional analysis capabilities for [`Quantity`]'s.
#[allow(unused)]
pub trait DimensionalAnalysableQuantity {
    /// Converts [`Quantity`]'s to a [`Dimension`].
    fn convert_to(&self, other: &Dimension) -> Result<Quantity>;
    /// Converts [`Quantity`]'s to each [`Dimension`] separately.
    fn convertable_to(&self, others: &[&Dimension]) -> Result<Box<[Quantity]>>;
}
impl DimensionalAnalysableQuantity for [&Quantity] {
    fn convert_to(&self, other: &Dimension) -> Result<Quantity> {
        let quantities: Box<[&Dimension]> = self.iter().map(|quantity| &quantity.dimension).collect();
        match quantities.exponents_to(other) {
            Ok(rows) => {
                let same_units: Quantity = rows.iter().enumerate().map(|(index, &power)| self[index].power(power)).product();
                Ok(
                    same_units.convert_to(other).expect("They should aready have the same units")
                )
            }
            Err(error) => {
                Err(QuantityError::UnconvertableQuantitiesError { base_quantities: self.iter().cloned().cloned().collect(), dimension_error: error })
            }
        }
    }
    fn convertable_to(&self, others: &[&Dimension]) -> Result<Box<[Quantity]>> {
        others.iter().map(|other| self.convert_to(other)).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        debug_println,
        {
            dimension::Prefix::{Centi, Hecto, Kilo, Milli, Pico},
            dimensions::{
                centimeter_gram_second_units::base_units::{
                    CENTI_METER,
                    GRAM
                }, drunk_mathematician_units::base_units::{
                    FOOT,
                    POUND,
                }, le_systeme_international_d_unites::{
                    base_units::{
                        KILOGRAM,
                        METER, SECOND
                    }, HERTZ, HOUR, JOULE, MINUTE
                }
            }, quantity::*
        },
    };

    #[test]
    fn test_add() {
        let lhs = Quantity::new(120, &*SECOND);
        let rhs = Quantity::new(2, &*MINUTE).convert_to(&lhs.dimension).expect("Seconds and minutes are compatible");
        let sum = &lhs + &rhs;
        assert!(sum.is_ok());
        let sum = sum.unwrap();
        assert_eq!(sum, Quantity::new(240, &*SECOND));
    }

    #[test]
    fn energy_example() {
        let mass = Quantity::new(5, &*KILOGRAM);
        let velocity = Quantity::new(10, &(&*METER / &*SECOND));
        let kinetic_energy = (&mass * &velocity.power(2)) / 2;
        debug_println!("kinetic_energy: {}", kinetic_energy);
        assert_eq!(kinetic_energy, Quantity::new(250, &*JOULE));

        let gravitational_acceleration = Quantity::new(9.81, &(&*METER / &SECOND.square()));
        let height = Quantity::new(2, &*METER);
        let potential_energy = &(&mass * &gravitational_acceleration) * &height;
        debug_println!("potential_energy: {}", potential_energy);
        assert_eq!(potential_energy, Quantity::new(98.1, &*JOULE));

        let energy = (&kinetic_energy + &potential_energy).expect("equal dimensions should be convertable");
        assert_eq!(energy, Quantity::new(98.1 + 250.0, &*JOULE));
        debug_println!("total_energy: {}", energy);
    }

    #[test]
    fn multiples_comma_submultiples_and_imperial_units_example() {
        let one_meter = Quantity::new(1, &*METER);
        let length_in_feet = one_meter.convert_to(&*FOOT).expect("meters and feet are compatible");
        let length_in_centimeters = one_meter.convert_to(&*CENTI_METER).expect("meters and centimeters are compatible");
        debug_println!("{} = {} = {}", one_meter, length_in_feet, length_in_centimeters);
        assert_eq!(length_in_feet.value, 3.28084);
        assert_eq!(length_in_centimeters.value, 100.0);

        let grams = Quantity::new(100, &*GRAM);
        let pounds = grams.convert_to(&*POUND).expect("grams and pounds are compatible");
        debug_println!("{} = {}", grams, pounds);
        assert_eq!(pounds.value, 0.220462);
    }

    #[test]
    fn exponent_aware_conversion_example() {
        let one_square_centimeter = Quantity::new(1, &METER.prefix(Centi).square());
        let area_in_square_meters = one_square_centimeter.convert_to(&METER.square()).expect("square centimeters and square meters are compatible");
        let length_in_meters = one_square_centimeter.convert_to(&METER).expect("square centimeters and meters are compatible");
        debug_println!("{} = {} = {}", one_square_centimeter, area_in_square_meters, length_in_meters);
        assert_eq!(area_in_square_meters.value, 0.0001);
        assert_eq!(length_in_meters.value, 0.01);

        let frequency = Quantity::new(50, &*HERTZ);
        let period = frequency.convert_to(&*SECOND).expect("Hertz and seconds are compatible");
        debug_println!("{} = {}", frequency, period);
        assert_eq!(period.value, 0.02);
    }

    #[test]
    fn curseder_units_optic_fiber_example() {
        let pulse_broadening = Quantity::new(1.2, &(&SECOND.prefix(Pico) / &METER.prefix(Kilo).power(0.5)));
        let propagation_distance = Quantity::new(100, &(METER.prefix(Kilo)));

        let total_spread = &pulse_broadening * &propagation_distance.power(0.5);
        debug_println!("Total pulse spread (in picoseconds): {}", total_spread);
        assert_eq!(total_spread.value, 12.0);
    }

    #[test]
    fn bomb_explosion_radius_example() {
        let energy = Quantity::new(100_000, &*JOULE);
        let explosion_time = Quantity::new(1, &*SECOND);
        let air_density = Quantity::new(1, &(&*KILOGRAM / &METER.cube()));

        let radius = (&(&energy / &air_density) * &explosion_time.power(2.0)).convert_to(&METER).expect("Resulting dimension should be length");
        debug_println!("Estimated explosion radius (in meters): {}", radius);
        assert!((radius.value - 10.0).abs() < 1.0);
    } 

    #[test]
    fn equality_example() {
        let one_minute = Quantity::new(1, &*MINUTE);
        let sixty_seconds = Quantity::new(60, &*SECOND);
        match one_minute.get_equality_with(&sixty_seconds) {
            Equality::ScalarMultiple(factor) => {
                assert_eq!(factor, 60.0);
            }
            _ => {
                panic!("1 min and 60 s should be scalar multiples");
            }
        }
    }

    #[test]
    fn different_dimension_example() {
        let length = Quantity::new(1, &*METER);
        let time = Quantity::new(1, &*SECOND);
        match length.get_equality_with(&time) {
            Equality::Different => {}
            _ => {
                panic!("1 m and 1 s should be different dimensions");
            }
        }
    }

    #[test]
    fn complex_equalty_example() {
        let hectareas = Quantity::new(100, &METER.prefix(Hecto).square());
        let length = Quantity::new(1_000_000, &METER.prefix(Milli));
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
    fn another_contrived_example() {
        let frequency = Quantity::new(2, &HERTZ.prefix(Kilo));
        let period = Quantity::new(0.5, &SECOND.prefix(Milli)).power(2);
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
        let length = Quantity::new(1, &*METER);
        let time = Quantity::new(1, &*SECOND);
        let result = &length + &time;
        assert!(result.is_err());
        debug_println!("Error message: {}", result.err().unwrap());
    }

    #[test]
    fn bomb_explosion_radius_example_as_dimensional_analysis() {
        let energy = Quantity::new(100_000, &*JOULE);
        let explosion_time = Quantity::new(1, &*SECOND);
        let air_density = Quantity::new(1, &(&*KILOGRAM / &METER.power(3)));

        let radius = [&energy, &explosion_time, &air_density].convert_to(&*METER).expect("Units to be convertible");
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
    fn conversion_with_different_multipliers() {
        let dollar = Dimension::new("dollar");
        let money_gained = Quantity::new(40, &dollar);
        let match_duration = Quantity::new(7, &*MINUTE);
        let salary = [&money_gained, &match_duration].convert_to(&(&dollar / &*HOUR)).expect("Convertable");
        assert_eq!(salary.dimension, &dollar / &*HOUR);
        assert_eq!(salary.value, 3.428571428571428e2);
    }
}