//! Also know as the *imperial system* - the drunk mathematician units are a convoluted system with weird units.
//! 
//! The stubborness of a few countries to still use this outdated system birthed the infamous Mars Climate Orbiter incident.

use crate::dimension;
use crate::dimensions::{drunk_mathematician_units::base_units::{FOOT, POUND}, le_systeme_international_d_unites::{STANDARD_GRAVITATIONAL_ACELERATION, base_units::{KILOGRAM, METER, SECOND}}};
use std::sync::LazyLock;
use crate::dimension::Dimension;

/// Based on the Foot Pound Second system of units.
pub mod base_units {
    use super::{dimension, METER, KILOGRAM, LazyLock, Dimension, SECOND};
    
    dimension!(FOOT =;0.3048;METER "`0.3048` [`METER`]s");
    dimension!(POUND =;0.453_592_37;KILOGRAM "`0.453_592_37` [`KILOGRAM`]s");
    
    /// The three base units.
    pub static BASE_UNITS: LazyLock<Box<[&LazyLock<Dimension>]>> = LazyLock::new(|| [&FOOT, &POUND, &SECOND].into());
}

// Mass submultiples / multiples
dimension!(,16,OUNCE = POUND "`16` in a [`POUND`].");
dimension!(STONE =;14;POUND "`14` [`POUND`]s.");
dimension!(TON =;2000;POUND "`2000` [`POUND`]s.");
dimension!(HUNDREDWEIGHT =;112;POUND "`112` [`POUND`]s (not a hundred).");

// Length submultiples / multiples
dimension!(,12,INCH = FOOT "`12` in a [`FOOT`].");
dimension!(YARD =;3;FOOT "`3` [`FOOT`]s.");
dimension!(CHAIN =;66;FOOT "`66` [`FOOT`]s.");
dimension!(FURLONG =;660;FOOT "`660` [`FOOT`]s.");
dimension!(MILE =;5280;FOOT "`5280` [`FOOT`]s.");
dimension!(LEAGUE =;15840;FOOT "`15840` [`FOOT`]s.");

// Force / Energy
dimension!(POUND_FORCE = POUND STANDARD_GRAVITATIONAL_ACELERATION "The force a [`POUND`] excerts at [`STANDARD_GRAVITATIONAL_ACELERATION`].");
dimension!(FOOT_POUND = POUND_FORCE FOOT "The energy used to apply a [`POUND_FORCE`] over a distanca of a [`FOOT`].");
dimension!(HORSEPOWER =;550;FOOT_POUND SECOND^-1 "`299,792,458` [`METER`]s per [`SECOND`].");


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