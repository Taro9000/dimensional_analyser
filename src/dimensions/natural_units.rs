use crate::{dimension, {dimension::{DeepDereferrenceable, DimensionalAnalysable}, dimensions::{le_systeme_international_d_unites::{JOULE, base_units::{AMPERE, KELVIN, KILOGRAM, METER, SECOND}}, natural_units::base_units::BASE_UNITS, the_seven_c_s::base_units::{C_AS_THE_SPEED_OF_LIGHT, COULOMB}}}};
use std::{f64::consts::PI, sync::LazyLock};
use crate::dimension::Dimension;

pub mod base_units {
    use super::*;
    dimension!(REDUCED_PLANCK_CONSTANT =;1.054_571_817e-34;JOULE SECOND);
    dimension!(GRAVITATIONAL_CONSTANT =;6.674_30e-11;METER^3 KILOGRAM^-1 SECOND^-2);
    dimension!(VACUUM_PERMITTIVITY =;8.854_187_8188e-12;SECOND^4 KILOGRAM^-1 METER^-3 AMPERE^2);
    dimension!(BOLTZMANN_CONSTANT =;1.380_649e-23;JOULE KELVIN^-1);
    
    #[allow(dead_code)]
    pub const BASE_UNITS: LazyLock<Box<[LazyLock<Dimension>]>> = LazyLock::new(|| [
        C_AS_THE_SPEED_OF_LIGHT, REDUCED_PLANCK_CONSTANT, GRAVITATIONAL_CONSTANT,
        VACUUM_PERMITTIVITY, BOLTZMANN_CONSTANT
    ].into());
}

dimension!(,137.035999177,FINE_STRUCTURE_CONSTANT =);
dimension!(DERIVED_CHARGE = BASE_UNITS.get().product_of_powers(&BASE_UNITS.get().exponents_to(&*COULOMB).expect("Convertible")));
dimension!(PLANK_CHARGE =;(4.0 * PI).sqrt();DERIVED_CHARGE);
dimension!(ELEMENTARY_CHARGE = PLANK_CHARGE FINE_STRUCTURE_CONSTANT^0.5);

#[cfg(test)]
mod tests {
    use crate::{debug_println, {dimension::{DeepDereferrenceable, DimensionalAnalysable}, dimensions::natural_units::{DERIVED_CHARGE, ELEMENTARY_CHARGE, PLANK_CHARGE, base_units::BASE_UNITS}}};
    #[test]
    fn coherency_of_system() {
        let exponents = BASE_UNITS.get().coherent_system5().expect("The natural units system should be coherent");
        debug_println!("The unit for mass:                {}", BASE_UNITS.get().product_of_powers(&exponents[0]));
        debug_println!("The unit for length:              {}", BASE_UNITS.get().product_of_powers(&exponents[1]));
        debug_println!("The unit for time:                {}", BASE_UNITS.get().product_of_powers(&exponents[2]));
        debug_println!("The unit for electrical current:  {}", BASE_UNITS.get().product_of_powers(&exponents[3]));
        debug_println!("The unit for temperature:         {}", BASE_UNITS.get().product_of_powers(&exponents[4]));
        debug_println!("The unit for charge:              {}", *DERIVED_CHARGE);
        debug_println!("The unit for plank charge:        {}", *PLANK_CHARGE);
        debug_println!("The unit for elementary charge:   {}", *ELEMENTARY_CHARGE);
    }
}