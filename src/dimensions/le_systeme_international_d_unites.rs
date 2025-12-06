//! The SI is the modern form of the metric system and the most widely used system of measurement.

use crate::dimensions::le_systeme_international_d_unites::base_units::{AMPERE, CANDELA, KILOGRAM, METER, MOLE, SECOND};
use crate::dimension;
use crate::dimension::Dimension;

/// The SI defines what a coherent system of units means in the context of this library - built on seven base units.
pub mod base_units {
    use std::sync::LazyLock;
    use super::{dimension, Dimension};
    
    dimension!(KILOGRAM "Unit of mass.");
    dimension!(METER "Unit of length.");
    dimension!(SECOND "Unit of time.");
    dimension!(AMPERE "Unit of electric current.");
    dimension!(KELVIN "Unit of temperature.");
    dimension!(MOLE "Unit of ammount of substance.");
    dimension!(CANDELA "Unit of luminous intensity.");
    
    /// The seven base units.
    #[allow(dead_code)]
    pub static BASE_UNITS: LazyLock<Box<[&LazyLock<Dimension>]>> = LazyLock::new(|| [&KILOGRAM, &METER, &SECOND, &AMPERE, &KELVIN, &MOLE, &CANDELA].into());
}

dimension!(STANDARD_GRAVITATIONAL_ACELERATION =;9.80665;METER SECOND^-2 "`9.80665` [`METER`]s per [`SECOND`] squared.");
dimension!(HERTZ = SECOND^-1 "Inverse [`SECOND`]s.");
dimension!(AVOGADRO_S_NUMBER =;6.022_140_8e23;MOLE^-1 "`6.022_140_8e23` per [`MOLE`].");
dimension!(LITER =,Deci METER^3 "[`Deci`] [`METER`] cubed.");

// Time multiples
dimension!(MINUTE =;60;SECOND "`60` [`SECOND`]s.");
dimension!(HOUR   =;60;MINUTE "`60` [`MINUTE`]s.");
dimension!(DAY    =;24;HOUR "`24` [`HOUR`]s.");
dimension!(YEAR   =;365.242_189_7;DAY "`365.242_189_7` [`DAY`]s.");

dimension!(,12,MONTH_AVERAGE  = YEAR "`12` in a [`YEAR`].");
dimension!(MONTH_LUNAR        =;29.53059;DAY "`29.53059` [`DAY`]s.");
dimension!(MONTH_CALENDAR     =;30;DAY "`30` [`DAY`]s.");
dimension!(MONTH_MENSTRUATION =;28;DAY "`28` [`DAY`]s.");

dimension!(DECADE     =;10;YEAR "`10` [`YEAR`]s.");
dimension!(CENTURY    =;100;YEAR "`100` [`YEAR`]s.");
dimension!(MILLENNIUM =;1000;YEAR "`1000` [`YEAR`]s.");

// Force
dimension!(NEWTON = KILOGRAM METER SECOND^-2 "[`KILOGRAM`] [`METER`]s per [`SECOND`] square.");
dimension!(JOULE  = NEWTON METER "[`NEWTON`] [`METER`]s.");
dimension!(WATT   = JOULE SECOND^-1 "[`JOULE`]s per [`SECOND`].");

// Electricity
dimension!(COULOMB = AMPERE SECOND "[`AMPERE`] [`SECOND`]s.");
dimension!(VOLT    = WATT COULOMB^-1 "[`WATT`]s per [`COULOMB`].");
dimension!(OHM     = VOLT AMPERE^-1 "[`VOLT`]s per [`AMPERE`].");
dimension!(TESLA   = KILOGRAM SECOND^-2 AMPERE^-1 "[`KILOGRAM`]s per [`SECOND`] squared per [`AMPERE`].");
dimension!(WEBER   = TESLA METER^2 "[`TESLA`] [`METER`]s squared.");
dimension!(FARAD   = COULOMB VOLT^-1 "[`COULOMB`]s per [`VOLT`].");
dimension!(HENRY   = WEBER AMPERE^-1 "[`WEBER`]s per [`AMPERE`].");

// Light
dimension!(LUMEN = CANDELA "[`CANDELA`].");
dimension!(LUX   = LUMEN METER^-2 "[`LUMEN`]s per [`METER`] square.");


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