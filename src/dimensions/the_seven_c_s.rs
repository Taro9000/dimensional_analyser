use crate::{dimensions::le_systeme_international_d_unites::{base_units::{AMPERE, CANDELA, KELVIN, METER, SECOND}, AVOGADRO_S_NUMBER, HERTZ, JOULE}};
use std::sync::LazyLock;
use crate::dimension::Dimension;

pub mod base_units {
    use crate::dimension;

    use super::*;
    
    dimension!(C_AS_THE_SPEED_OF_LIGHT =;299_792_458;METER SECOND^-1);
    dimension!(CALORIE =;4.184;JOULE);
    dimension!(C_AS_STANDARD_MIDDLE_TUNING =;220.0 * 2.0f64.sqrt().sqrt();HERTZ); // Do
    dimension!(CELSIUS = KELVIN); // I won't bother
    dimension!(COULOMB = AMPERE SECOND);
    dimension!(C_ROMAN_NUMERAL =;100;AVOGADRO_S_NUMBER^-1);
    
    #[allow(dead_code)]
    pub const BASE_UNITS: LazyLock<Box<[LazyLock<Dimension>]>> = LazyLock::new(|| [
        C_AS_THE_SPEED_OF_LIGHT, CALORIE, C_AS_STANDARD_MIDDLE_TUNING,
        CELSIUS, CANDELA,
        COULOMB, C_ROMAN_NUMERAL
    ].into());
}


#[cfg(test)]
mod tests {
    use crate::{debug_println, {dimension::{DeepDereferrenceable, DimensionalAnalysable}, dimensions::the_seven_c_s::base_units::BASE_UNITS}};
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