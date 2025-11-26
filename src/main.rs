/*
use crate::{dimension::{Dimension, DIMENSIONLESS}, dimensions::le_systeme_international_d_unites::{base_units::{KILOGRAM, METER, SECOND}, HOUR, JOULE, MINUTE, LITER}, quantity::{DimensionalAnalysableQuantity, Quantity, Result}};
*/
fn main() /*-> Result<()>*/ {
    /*let height       = Quantity::new(5 , dim!(METER));
    println!("Height:       {}", height);
    let mass         = Quantity::new(15, dim!(KILOGRAM));
    println!("Mass:         {}", mass);
    let acceleration = Quantity::new(9.81,dim!(METER SECOND^-2));
    println!("Acceleration: {}", acceleration);
    let speed        = Quantity::new(20, dim!(METER SECOND^-1));
    println!("Speed:        {}", speed);

    let energy = dim!(JOULE);

    let potential_energy = [&height, &mass, &acceleration].convert_to(energy)?;
    println!("Potential energy: {}", potential_energy);
    let kinetic_energy = [&mass, &speed].convert_to(energy)? / 2;
    println!("Kinetic energy:   {}", kinetic_energy);
    let total_energy = (&potential_energy + &kinetic_energy)?;
    println!("Total energy:     {}", total_energy);
    
    let minute = &*MINUTE;
    println!("Minute: {:?}", minute.exponents());
    Dimension::new("other_letter");
    Dimension::new("otasdasdher_letter");
    Dimension::new("otasdasasddher_letter");
    Dimension::new("otasdasdhdaser_letter");
    Dimension::new("otasdasdhasdaser_letter");
    Dimension::new("otasdasdhasdaser_letter");
    let letter = Dimension::new("letter");
    println!("Letter: {:?}", letter.exponents());
    let word = letter.multiple(5);
    let typing_speed = Quantity::new(24, dim!(word minute^-1));
    println!("Minute: {:?}", minute.exponents());
    println!("Typing speed: {}", typing_speed);

    let dollar = Dimension::new("dollar");
    let money_gained = Quantity::new(40, dim!(dollar));
    let match_duration = Quantity::new(7, dim!(MINUTE));
    println!("Hour: {}", dim!(HOUR));
    println!("Per hour: {}", dim!(HOUR^-1));
    println!("Per hour: {}", dim!(DIMENSIONLESS HOUR^-1));
    println!("Salary: {}", [&money_gained, &match_duration].convert_to(dim!(dollar HOUR^-1))?);
    
    println!("Salary: {}", [&money_gained, &match_duration].convert_to(dim!(METER HOUR^-1)).err().expect("msg"));

    let liter = METER.cube().prefix(dimension::Prefix::Milli);
    println!("Prefix before exponentiation: {}", liter == *LITER);

    Ok(())*/
}