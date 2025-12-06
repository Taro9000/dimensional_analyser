//! A joke measurement system that helps to serve as an example of dimensional analysis.

use crate::{dimensions::le_systeme_international_d_unites::{base_units::{AMPERE, CANDELA, KELVIN, METER, SECOND}, AVOGADRO_S_NUMBER, HERTZ, JOULE}};
use std::sync::LazyLock;
use crate::dimension::Dimension;

/// Contains the seven units starting with the letter *C*.
pub mod base_units {
    use crate::dimension;

    use super::{METER, SECOND, JOULE, HERTZ, KELVIN, AMPERE, AVOGADRO_S_NUMBER, LazyLock, Dimension, CANDELA};
    
    dimension!(C_AS_THE_SPEED_OF_LIGHT =;299_792_458;METER SECOND^-1 "`299_792_458` [`METER`]s per [`SECOND`].");
    dimension!(CALORIE =;4.184;JOULE "`4.184` [`JOULE`]s.");
    dimension!(C_AS_STANDARD_MIDDLE_TUNING =;220.0 * 2.0f64.sqrt().sqrt();HERTZ "Also know as *Do*. `220` fourth roots of `2` [`HERTZ`].");
    dimension!(CELSIUS = KELVIN "Just [`KELVIN`] becuase I don't know an approach that satisfies me with multiplication.");
    dimension!(COULOMB = AMPERE SECOND "[`AMPERE`] [`SECOND`]s.");
    dimension!(C_ROMAN_NUMERAL =;100;AVOGADRO_S_NUMBER^-1 "`100` per [`AVOGADRO_S_NUMBER`].");
    
    /// The seven units starting with the letter *C*.
    pub static BASE_UNITS: LazyLock<Box<[&LazyLock<Dimension>]>> = LazyLock::new(|| [
        &C_AS_THE_SPEED_OF_LIGHT, &CALORIE, &C_AS_STANDARD_MIDDLE_TUNING,
        &CELSIUS, &CANDELA,
        &COULOMB, &C_ROMAN_NUMERAL
    ].into());
}


#[cfg(test)]
mod tests {
    use crate::{debug_println, dimension::{DeepDereferrenceable, DimensionalAnalysable}, dimensions::the_seven_c_s::base_units::BASE_UNITS};
    #[test]
    fn coherency_of_system() {
        let exponents = BASE_UNITS.get().coherent_system7().expect("The seven c's system should be coherent");
        debug_println!("The unit for mass:                {}", BASE_UNITS.get().product_of_powers(&exponents[0]));
        debug_println!("The unit for length:              {}", BASE_UNITS.get().product_of_powers(&exponents[1]));
        debug_println!("The unit for time:                {}", BASE_UNITS.get().product_of_powers(&exponents[2]));
        debug_println!("The unit for electrical current:  {}", BASE_UNITS.get().product_of_powers(&exponents[3]));
        debug_println!("The unit for temperature:         {}", BASE_UNITS.get().product_of_powers(&exponents[4]));
        debug_println!("The unit for ammount of particles:{}", BASE_UNITS.get().product_of_powers(&exponents[5]));
        debug_println!("The unit for luminous intensity:  {}", BASE_UNITS.get().product_of_powers(&exponents[6]));
    }
}