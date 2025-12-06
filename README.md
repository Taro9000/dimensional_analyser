# dimensional_analyser

A small Rust library for runtime dimensional analysis and unit conversion.

This crate provides:

- A `Dimension` type representing derived physical dimensions (exponents and scaling factor).
- A `Quantity` type that combines a numeric value with a `Dimension` and supports arithmetic and conversion.
- Algorithms to compute exponents required to express a target dimension as a product of base dimensions (uses a Bareiss elimination solver internally).
- Several predefined unit systems (SI and example unit sets) in `src/dimensions/`.

The library is designed to be flexible: you can create new base dimensions at runtime, apply metric prefixes, raise dimensions to powers, and convert between compatible quantities (including exponent-aware conversions, e.g. area/length conversions).

## Features

- Create base dimensions dynamically with `Dimension::new`.
- Compose and manipulate derived dimensions with multiplication, division and powering.
- Convert between quantities that are compatible via exponent solving (e.g., derive energy from mass, length and time).
- Includes unit collections in `src/dimensions/` such as an SI-like set. Tests demonstrate conversions with imperial and other example units.

## Quick example

Example usage (based on `src/main.rs` and the library docs):

```rust
use dimensional_analyser::{Result, dim, dimension::{DIMENSIONLESS, Dimension, Prefix}, dimensions::le_systeme_international_d_unites::{HOUR, JOULE, LITER, MINUTE, base_units::{KILOGRAM, METER, SECOND}}, quantity::{DimensionalAnalysableQuantity, Quantity}};

fn main() -> Result {
	let height       = Quantity::new(5 , dim!(METER));
	let mass         = Quantity::new(15, dim!(KILOGRAM));
	let acceleration = Quantity::new(9.81,dim!(METER SECOND^-2));
	let speed        = Quantity::new(20, dim!(METER SECOND^-1));

	let energy = dim!(JOULE);
	let potential_energy = [&height, &mass, &acceleration].convert_to(energy)?;
	let kinetic_energy = [&mass, &speed].convert_to(energy)? / 2;
	let total_energy = (&potential_energy + &kinetic_energy)?;

	println!("Potential energy: {}", potential_energy);
	println!("Kinetic energy: {}", kinetic_energy);
	println!("Total energy: {}", total_energy);

	Ok(())
}
```

## Crate layout / API overview

- `src/dimension.rs` — core `Dimension` type, prefixes, exponent algebra, conversion-exponent calculation.
- `src/quantity.rs` — `Quantity` type, arithmetic (+, -, *, /), and conversion helpers for multiple quantities.
- `src/bareiss_eliminator.rs` — helper matrix code and Bareiss elimination solver used to compute exponents.
- `src/dimensions/` — predefined unit sets (e.g., SI-like units, examples).

Publicly exported items include (non exhaustive):

- `Dimension` and helpers like `Prefix` and convenience methods (`square`, `cube`, `power`, `prefix`).
- `Quantity` with `new`, `convert_to`, arithmetic and inspection helpers.
- `dim!` macro and constant-backed unit sets in `dimensions`.

Refer to the crate-level docs in `src/lib.rs` for a short annotated example.

There are optional features defined in `Cargo.toml`: `debug-print` and `sci-notation`

## Contributing

- Bug reports and pull requests are welcome. Please include small, focused changes and tests when appropriate.
- Prefer small, well-documented commits.

## Notes

- This project includes an internal Bareiss matrix solver implementation used for solving the exponent linear systems; tests cover many of those cases.