use crate::dimension;
use crate::dimensions::centimeter_gram_second_units::base_units::{CENTI_METER, GRAM};
use std::sync::LazyLock;
use crate::dimension::Dimension;
use crate::dimensions::le_systeme_international_d_unites::base_units::{KILOGRAM, METER, SECOND};

pub mod base_units {
    use super::*;
    
    dimension!(,1000,GRAM = KILOGRAM);
    dimension!(CENTI_METER =,Centi METER);
    
    #[allow(dead_code)]
    pub const BASE_UNITS: LazyLock<Box<[LazyLock<Dimension>]>> = LazyLock::new(|| [GRAM, CENTI_METER, SECOND].into());
}


dimension!(DYNE = GRAM CENTI_METER SECOND^-2);
dimension!(ERG = DYNE CENTI_METER); 
dimension!(BAR = DYNE CENTI_METER^2);


#[cfg(test)]
mod tests {
    use crate::{debug_println, {dimension::{DeepDereferrenceable, DimensionalAnalysable}, dimensions::centimeter_gram_second_units::base_units::BASE_UNITS}};

    #[test]
    fn coherency_of_system() {
        let exponents = BASE_UNITS.get().coherent_system3().expect("The CGS system should be coherent");
        debug_println!("The unit for mass:  {}", BASE_UNITS.get().product_of_powers(&exponents[0]));
        debug_println!("The unit for length:{}", BASE_UNITS.get().product_of_powers(&exponents[1]));
        debug_println!("The unit for time:  {}", BASE_UNITS.get().product_of_powers(&exponents[2]));
    }
}