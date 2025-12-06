//! A library made for dynamic dimensional analysis capable of:
//! - Generating new base units
//! - Dimensional analysis by getting the exponents needed to multiply some base quantities to a target dimension
//! 
//! # Example usage
//! ```
//! use dimensional_analyser::{Result, dim, dimension::{DIMENSIONLESS, Dimension, Prefix}, dimensions::le_systeme_international_d_unites::{HOUR, JOULE, LITER, MINUTE, base_units::{KILOGRAM, METER, SECOND}}, quantity::{DimensionalAnalysableQuantity, Quantity}};
//! 
//! fn main() -> Result {
//!     let height       = Quantity::new(5 , dim!(METER));
//!     println!("Height:       {}", height);
//!     let mass         = Quantity::new(15, dim!(KILOGRAM));
//!     println!("Mass:         {}", mass);
//!     let acceleration = Quantity::new(9.81,dim!(METER SECOND^-2));
//!     println!("Acceleration: {}", acceleration);
//!     let speed        = Quantity::new(20, dim!(METER SECOND^-1));
//!     println!("Speed:        {}", speed); 
//!     let energy = dim!(JOULE); 
//!     let potential_energy = [&height, &mass, &acceleration].convert_to(energy)?;
//!     println!("Potential energy: {}", potential_energy);
//!     let kinetic_energy = [&mass, &speed].convert_to(energy)? / 2;
//!     println!("Kinetic energy:   {}", kinetic_energy);
//!     let total_energy = (&potential_energy + &kinetic_energy)?;
//!     println!("Total energy:     {}", total_energy);
//!     
//!     let minute = &*MINUTE;
//!     println!("Minute: {:?}", minute.exponents());
//!     let letter = Dimension::new("letter");
//!     println!("Letter: {:?}", letter.exponents());
//!     let word = letter.scale(5);
//!     let typing_speed = Quantity::new(24, dim!(word minute^-1));
//!     println!("Minute: {:?}", minute.exponents());
//!     println!("Typing speed: {}", typing_speed); 
//!     let dollar = Dimension::new("dollar");
//!     let money_gained = Quantity::new(40, dim!(dollar));
//!     let match_duration = Quantity::new(7, dim!(MINUTE));
//!     println!("Salary: {}", [&money_gained, &match_duration].convert_to(dim!(dollar HOUR^-1))?);
//!     Ok(())
//! }
//! ```
#![warn(missing_docs)]

use std::fmt::{self, Display, Formatter};

#[allow(unused)]
use crate::{dimension::{ConversionExponentError, Dimension, UnconvertableDimensionsError}, quantity::{DifferentDimensionError, UnconvertableQuantitiesError, UnconvertableQuantityError, Quantity}};

pub mod quantity;
pub mod dimension;
mod bareiss_eliminator;

pub mod dimensions;

/// The type returned the the `main` function
pub type Result = std::result::Result<(), Error>;

macro_rules! error_enum {
    ($(#[doc = $doc:literal] $variant:ident($error:ident))*) => {
        /// An error enum that encompasses every error in this library
        #[derive(Debug, Clone, PartialEq)]
        pub enum Error { $(
            #[doc = $doc]
            $variant($error),
        )* }
        impl Display for Error {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                match self {
                    $(Self::$variant(error) =>
                        write!(f, "{error}"),)*
                }
            }
        }
        $(impl From<$error> for Error {
            fn from(value: $error) -> Self {
                Self::$variant(value)
            }
        })*
    };
}

error_enum! {
    /// A detailed breakdown of the error occurred when getting the conversion exponent from one [`Dimension`]s to another one.
    ConversionExponent(ConversionExponentError)
    /// Error when the base [`Dimension`]s can't be converted to the target [`Dimension`].
    UnconvertableDimensions(UnconvertableDimensionsError)
    /// Error when a single [`Quantity`] can't be converted to a [`Dimension`].
    UnconvertableQuantity(UnconvertableQuantityError)
    /// Error when multiple [`Quantity`]s can't be converted to a [`Dimension`].
    UnconvertableQuantities(UnconvertableQuantitiesError)
    /// Error when both [`Dimension`]s are incompatible.
    DifferentDimension(DifferentDimensionError)
}
impl std::error::Error for Error {}