# Change log
<!-- Template:
## [version] — YYYY-MM-DD

### Security
### Added
### Changed
### Deprecated
### Removed
### Fixed
-->

## [0.1.5] — 2024-03-05

### Security
### Added
`unit(&self)` method for unit enumerator (e.g. `LengthUnit`) that now returns a `Unit` (e.g. `UnitBase` + multiplier). 
Continue to add more unit conversion tests back in.
### Changed
Method previously labeled `unit(&self)`and returns `UnitBase` renamed to `unit_base()` for clarity. 
### Deprecated
### Removed

## [0.1.4] — 2024-03-02

### Security
### Added
### Changed
The constructor for `Quantity` now takes `Unit` rather than `Units`.
### Deprecated
### Removed

## [0.1.3] — 2024-03-02

### Security
### Added
1. Added a public `convert_unit` method for `Quantity` class that uses `Unit` as the parameter to specify the destination units. The `convert` method still exists to convert to units specified by a `Units` type.
2. Continue chipping away at converting old uom unit tests for individual quantities. 
3. Corrected coefficients for statampere, statvolt, and statfarad. Insufficient precision in the uom version was causing conversion inaccuracies.
### Changed
The `unit` and `value` fields of `Quantity` are now pub(crate) rather that public. Use `unit()` or `value()` instead.
The `get_unit_base()` method, which retreives the corresponding `Unit` for a derived unit (i.e. `LengthUnit`), has been renamed to `unit()`.
### Deprecated
### Removed
Removed duplicate copy of unit definitions.
### Fixed

## [0.1.2] — 2024-03-01

### Security
### Added
Support for `no_std` can be provided by removing the `std` feature. 
Added implementation of `std::error::Error` I previously neglected for `RuntimeUnitError`.
### Changed
### Deprecated
### Removed
### Fixed

## [0.1.1] — 2024-02-28

### Security
### Added
Added ability to perform operations (Add, Sub Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign) to unit-specific quantities (e.g. `LengthQuantity`).
### Changed
Improved get_unit_base() functionality, which no longer requires a lazy static inititalization.
### Deprecated
### Removed
### Fixed

## [0.1.0] — 2024-02-25

### Security
### Added
Initial commit of base capabilities
### Changed
### Deprecated
### Removed
### Fixed