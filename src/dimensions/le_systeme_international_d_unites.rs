use crate::dimensions::le_systeme_international_d_unites::base_units::{AMPERE, CANDELA, KILOGRAM, METER, MOLE, SECOND};
use crate::dimension;
use crate::dimension::Dimension;

pub mod base_units {
    use std::sync::LazyLock;
    use super::*;
    
    dimension!(KILOGRAM);
    dimension!(METER);
    dimension!(SECOND);
    dimension!(AMPERE);
    dimension!(KELVIN);
    dimension!(MOLE);
    dimension!(CANDELA);
    
    #[allow(dead_code)]
    pub const BASE_UNITS: LazyLock<Box<[LazyLock<Dimension>]>> = LazyLock::new(|| [KILOGRAM, METER, SECOND, AMPERE, KELVIN, MOLE, CANDELA].into());
}

dimension!(HERTZ = SECOND^-1);
dimension!(AVOGADRO_S_NUMBER =;6.0221408e23;MOLE^-1);
dimension!(LITER =,Deci METER^3);

// Time multiples
dimension!(MINUTE =;60;SECOND);
dimension!(HOUR   =;60;MINUTE);
dimension!(DAY    =;24;HOUR);
dimension!(YEAR   =;365.2421897;DAY);

dimension!(,12,MONTH_AVERAGE  = YEAR);
dimension!(MONTH_LUNAR        =;29.53059;DAY);
dimension!(MONTH_CALENDAR     =;30;DAY);
dimension!(MONTH_MENSTRUATION =;28;DAY); 

dimension!(DECADE     =;10;YEAR);
dimension!(CENTURY    =;100;YEAR);
dimension!(MILLENNIUM =;1000;YEAR);

// Force
dimension!(NEWTON = KILOGRAM METER SECOND^-2);
dimension!(JOULE  = NEWTON METER);
dimension!(WATT   = JOULE SECOND^-1);

// Electricity
dimension!(COULOMB = AMPERE SECOND);
dimension!(VOLT    = WATT COULOMB^-1);
dimension!(OHM     = VOLT AMPERE^-1);
dimension!(TESLA   = KILOGRAM SECOND^-2 AMPERE^-1);
dimension!(WEBER   = TESLA METER^2);
dimension!(FARAD   = COULOMB VOLT^-1);
dimension!(HENRY   = WEBER AMPERE^-1);

// Light
dimension!(LUMEN = CANDELA);
dimension!(LUX   = LUMEN METER^-2);


#[cfg(test)]
mod tests {
    use crate::{debug_println, {dimension::{DeepDereferrenceable, DimensionalAnalysable}, dimensions::le_systeme_international_d_unites::base_units::BASE_UNITS}};

    #[test]
    fn coherency_of_system() {
        let exponents = BASE_UNITS.get().coherent_system7().expect("The SI system should be coherent");
        debug_println!("The unit for mass:                {}", BASE_UNITS.get().product_of_powers(&exponents[0]));
        debug_println!("The unit for length:              {}", BASE_UNITS.get().product_of_powers(&exponents[1]));
        debug_println!("The unit for time:                {}", BASE_UNITS.get().product_of_powers(&exponents[2]));
        debug_println!("The unit for electrical current:  {}", BASE_UNITS.get().product_of_powers(&exponents[3]));
        debug_println!("The unit for temperature:         {}", BASE_UNITS.get().product_of_powers(&exponents[4]));
        debug_println!("The unit for ammount of particles:{}", BASE_UNITS.get().product_of_powers(&exponents[5]));
        debug_println!("The unit for luminous intensity:  {}", BASE_UNITS.get().product_of_powers(&exponents[6]));
    }
}