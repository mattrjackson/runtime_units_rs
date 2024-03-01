# runtime_units
The goal for this library is to serve as a run-time Rust library for working with units of measurement and conversions between them. Most robust units libraries for Rust are designed for compile-time constant analysis. If one of those crates meets your needs, you should probably use one of those instead.

Much of the code (particularly the unit definitions) were adapted from the excellent uom library (https://github.com/iliekturtles/uom). This library was designed to support cases where compile time analysis isn't a great fit. This can presently handle conversions of everything supported by uom except temperature (temperature intervals are supported).  

## Usage
```toml
[dependencies]
runtime_units = { version = "0.1.0", features = ["All", "serde"] }
```
no_std is supported if the `std` feature flag is removed.

Individual unit types are supported as features, allowing you to pare down the library to what you need. Serialization is optionally supported via the `serde` feature tag, and utoipa schemas can optionally be generated via the `utoipa` tag. By default, only the base SI units are enabled (https://en.wikipedia.org/wiki/SI_base_unit).

# Quantities and Units

Units are defined in the `units` module. This module adds a `Unit` suffix to each quantity processed (depending on the features you compile with). So a `LengthUnit` might have centimeter, meter, foot, etc. defined. A corresponding quantity is generated for each class of unit in the `quantities` module with a `Quantity` suffix (e.g. `LengthQuantity`). These are the fastest way to convert between units of a single type. The `Quantity` class itself uses the `Units` enumeration, which wraps units for all possible combinations - it can convert units from one type to another. 

Unit conversion is accomplished within the `Quantity` struct. Behind seven integers in a bitfield store the dimensions of a given unit type, along with the multiplier used to convert one from the other. If you only need to convert between units of a given quantity, use `convert` within the prefixed quantities (e.g. `LengthQuantity`, `EnergyQuantity`), as they are a bit faster.

runtime_units supports serializable quantities with the base units (e.g. `Length`, `Velocity`)

Example:

```rust
use runtime_units::Length;

fn main() {
    let length = Length::meter(1.0);
    let time = Time::second(5.0);
    let velocity  = Velocity::meter_per_second(0.2).to_quantity();
    assert!(length.to_quantity()/time.to_quantity() == velocity);
    
    let length_quantity = LengthQuantity::meter(10.0);
    let time_quantity = TimeQuantity::second(1.0);
    let velocity_quantity = VelocityQuantity::meter_per_second(10.0).to_foot_per_second();
    assert!(length_quantity.to_foot() / time_quantity == velocity_quantity);

    let quantity = *length_quantity;        
    assert!(quantity.convert(Units::Acceleration(crate::units::AccelerationUnit::centimeter_per_second_squared)).is_err());
    assert!(quantity.convert(Units::Length(crate::units::LengthUnit::angstrom)).is_ok());
}

```
# Performance
Currently conversions on my laptop cost < 1 nsec if converting between the same quantity, and up to 3 nsec using the convert method from `Quantity`. 
