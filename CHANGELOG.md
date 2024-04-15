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

## [0.2.4] — 2024-04-15

### Security
### Added
Implemented `From<Units>` for `Quantities` to allow creation of a default unit from the quantity.
Added `new` method for `Quantities` to permit construction for a given value and `Units`. 
### Changed
### Deprecated
### Removed

## [0.2.3] — 2024-03-20

### Security
### Added
Added `try_convert` and `unit` methods to `Quantities`.
### Changed
### Deprecated
### Removed
Removed unused imports in several unit tests.

## [0.2.2] — 2024-03-19

### Security
### Added
Added `to_string` implementation for `Quantities`.
### Changed
### Deprecated
### Removed

## [0.2.1] — 2024-03-13

Small bugfixes to correct integration issues I ran into.

### Security
### Added
Added `unit()` method for quantities.
Added `unit_mut()` and `definition_mut()` methods for `Quantity`.
Added `From<UnitDefinition>` for unit quantity enumerations. 
Allow conversion from `Quantity` back to a specialized quantity (e.g. `Length`)
### Changed
Changed `QuantityBase` back to `Quantity`. 
### Deprecated
### Removed


## [0.2.0] — 2024-03-13

### Security
### Added
Added `to_unit` method for `UnitTypes` to allow conversion from an `&str` to `Units`.
Added ability to convert from individual unit enumerations (e.g. `LengthUnit`) to `Units`.
Added additional documentation to many methods.
### Changed
Major refactor to clean up messy data structures:
1. `Quantity` has been renamed to `QuantityBase`. This still represents the base data structure used to handle unit conversions.
2. Renamed `Unit` to `UnitDefinition`, to more accurately reflect what the data structure represents.
3. Corrected bug in serde dependency (added derive feature flag dependency).
### Deprecated
### Removed
Removed Quantity-suffixed data structures (e.g. `LengthQuantity`) and merged its capabilities into the  quantity enumerations (e.g. `Length`, `Area`, etc.). 
Refactored and removed dependencies on once-cell and strum.

## [0.1.6] — 2024-03-12

### Security
### Added
Added `to_unit` method for `UnitTypes` to allow conversion from an `&str` to `Units`.
Added ability to convert from individual unit enumerations (e.g. `LengthUnit`) to `Units`.
Added additional documentation to many methods.
### Changed
Corrected bug in `readme_example()`. Added tag since it is dependent on `std`.
Changed From<&str> for individual unit enumerations to TryFrom<&str>.
### Deprecated
### Removed
Removed `unit_base` and `multiplier` from `Units`. These were originally used for internal operations, but are no longer needed.


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