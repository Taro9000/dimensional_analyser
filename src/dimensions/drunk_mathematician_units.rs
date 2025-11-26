use crate::dimension;
use crate::dimensions::drunk_mathematician_units::base_units::{FOOT, POUND};
use std::sync::LazyLock;
use crate::dimension::Dimension;
use crate::dimensions::le_systeme_international_d_unites::base_units::{KILOGRAM, METER, SECOND};

pub mod base_units {
    use super::*;
    
    dimension!(,2.20462,POUND = KILOGRAM);
    dimension!(,3.28084,FOOT = METER);
    
    #[allow(dead_code)]
    pub const BASE_UNITS: LazyLock<Box<[LazyLock<Dimension>]>> = LazyLock::new(|| [POUND, FOOT, SECOND].into());
}

// Mass submultiples / multiples
dimension!(,16,OUNCE = POUND);
dimension!(STONE =;14;POUND);
dimension!(TON =;2000;POUND);
dimension!(HUNDREDWEIGHT =;112;POUND);

// Length submultiples / multiples
dimension!(,12,INCH = FOOT);
dimension!(YARD =;3;FOOT);
dimension!(CHAIN =;66;FOOT);
dimension!(FURLONG =;660;FOOT);
dimension!(MILE =;5280;FOOT);
dimension!(LEAGUE =;15840;FOOT);

// Force / Energy
dimension!(POUND_FORCE = POUND FOOT SECOND^-2);
dimension!(FOOT_POUND = POUND_FORCE FOOT);
dimension!(HORSEPOWER =;550;FOOT_POUND SECOND^-1);


#[cfg(test)]
mod tests {
    use crate::{debug_println, {dimension::{DeepDereferrenceable, DimensionalAnalysable}, dimensions::drunk_mathematician_units::base_units::BASE_UNITS}};
    #[test]
    fn coherency_of_system() {
        let exponents = BASE_UNITS.get().coherent_system3().expect("The drunk mathematician unit system shouldn't be coherent but oh well...");
        debug_println!("The unit for mass:  {}", BASE_UNITS.get().product_of_powers(&exponents[0]));
        debug_println!("The unit for length:{}", BASE_UNITS.get().product_of_powers(&exponents[1]));
        debug_println!("The unit for time:  {}", BASE_UNITS.get().product_of_powers(&exponents[2]));
    }
}