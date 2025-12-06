//! My own joke unit system that strives to unite every dimension to ***The Perfect Cheezeburger***.

use crate::{dimension, dimensions::{centimeter_gram_second_units::base_units::GRAM, drunk_mathematician_units::INCH, le_systeme_international_d_unites::{base_units::{CANDELA, KELVIN, KILOGRAM, MOLE}, AVOGADRO_S_NUMBER, WATT}, natural_units::ELEMENTARY_CHARGE, the_seven_c_s::base_units::CALORIE}};
use std::{f64::consts::PI, sync::LazyLock};
use crate::dimension::Dimension;

dimension!(HAMBURGER_RADIUS =;2.75;INCH "2.75 [`INCH`]s.");
dimension!(HAMBURGER_HEIGHT =;3.5;INCH "2.75 [`INCH`]s.");

/// Contains the chosen properties of  ***The Perfect Cheezeburger***. 
pub mod base_units {
    use super::{dimension, HAMBURGER_RADIUS, HAMBURGER_HEIGHT, PI, KILOGRAM, CALORIE, GRAM, MOLE, ELEMENTARY_CHARGE, AVOGADRO_S_NUMBER, KELVIN, CANDELA, WATT, LazyLock, Dimension};
    
    dimension!(HAMBURGER_VOLUME =;PI;HAMBURGER_RADIUS^2 HAMBURGER_HEIGHT "Calculated as if it were a perfect cilinder.");
    dimension!(,8,HAMBURGER_MASS = KILOGRAM "8 in a [`KILOGRAM`]");
    dimension!(HAMBURGER_ENERGY =;378.75;,Kilo CALORIE "`378.75` [`Kilo`] [`CALORIE`]s");
    dimension!(HAMBURGER_MOLAR_MASS =;18.015;GRAM MOLE^-1 "`18.015` [`GRAM`]s per [`MOLE`] (simplified as if ***The Perfect Cheezeburger*** was pure water)");
    dimension!(HAMBURGER_MOLAR_CHARGE =;10;ELEMENTARY_CHARGE AVOGADRO_S_NUMBER "`10` electrons per molecule of water");
    dimension!(HAMBURGER_SPECIFIC_HEAT_CAPACITY = CALORIE GRAM^-1 KELVIN^-1 "[`CALORIE`]s per [`GRAM`] per [`KELVIN`]");
    dimension!(HAMBURGER_LUMINOUS_EFFICIENCY =;683.0 * 4.0 * PI;CANDELA WATT^-1 "`683` times `4` [`PI`] [`CANDELA`]s per [`WATT`] (the perfect efficiency)");
    
    /// The chosen properties of ***The Perfect Cheezeburger***.
    #[allow(dead_code)]
    pub static BASE_UNITS: LazyLock<Box<[&LazyLock<Dimension>]>> = LazyLock::new(|| [
        &HAMBURGER_VOLUME, &HAMBURGER_MASS, &HAMBURGER_ENERGY,
        &HAMBURGER_SPECIFIC_HEAT_CAPACITY, &HAMBURGER_MOLAR_MASS,
        &HAMBURGER_MOLAR_CHARGE, &HAMBURGER_LUMINOUS_EFFICIENCY,
    ].into());
}

#[cfg(test)]
mod tests {
    use crate::{debug_println, {dimension::{DeepDereferrenceable, DimensionalAnalysable}, dimensions::hamburger_units::base_units::BASE_UNITS}};
    
    #[test]
    fn coherency_of_system() {
        let exponents = BASE_UNITS.get().coherent_system7().expect("The hamburger unit system should be coherent");
        debug_println!("The unit for mass:                {}", BASE_UNITS.get().product_of_powers(&exponents[0]));
        debug_println!("The unit for length:              {}", BASE_UNITS.get().product_of_powers(&exponents[1]));
        debug_println!("The unit for time:                {}", BASE_UNITS.get().product_of_powers(&exponents[2]).portion(2.0f64.sqrt()));
        debug_println!("The unit for electrical current:  {}", BASE_UNITS.get().product_of_powers(&exponents[3]));
        debug_println!("The unit for temperature:         {}", BASE_UNITS.get().product_of_powers(&exponents[4]));
        debug_println!("The unit for ammount of particles:{}", BASE_UNITS.get().product_of_powers(&exponents[5]));
        debug_println!("The unit for luminous intensity:  {}", BASE_UNITS.get().product_of_powers(&exponents[6]).scale(2.0f64.sqrt()));
    }
}