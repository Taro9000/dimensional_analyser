# Changelog

## Unreleased

- Add `LICENSE` (MIT).
- Improve documentation: move illustrative tests to focused doctests and tidy `README.md`.
- Add package metadata to `Cargo.toml`: `description`, `readme`, `license`, `repository`, `documentation`, `keywords`, `categories`.

## 0.1.0 - Unreleased

- Initial crate contents and core API (`Dimension`, `Quantity`, unit sets, Bareiss solver).

## 0.1.1

- Updated the categories of the crate.

## 0.1.2

- Removed the debugging macro.

## 0.1.3

- Added another test at src/quantity.rs to fix a logical error at the exponent obtainment at src/dimension.rs.

## 0.2.0 - Revamped Scientific-notation Handling

- Added another test at src/quantity.rs.
- Removed an expect from the conversion of multiple quantities to a dimension.
- Fixed voltage being watts per coulomb instead of joules per second.
- **Removed** the sci-notation configuration to instead provide both Display and **LowerExp** traits