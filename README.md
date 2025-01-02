# runtime_units
The goal for this library is to serve as a run-time Rust library for working with units of measurement and conversions between them. Most robust units libraries for Rust are designed for compile-time constant analysis. If one of those crates meets your needs, you should probably use one of those instead.

Much of the code (particularly the unit definitions) were adapted from the excellent uom library (https://github.com/iliekturtles/uom). This library was designed to support cases where compile time analysis isn't a great fit. This can presently handle conversions of everything supported by uom except temperature (temperature intervals are supported).  

## Usage

An example of including all units and serialization support:

```toml
[dependencies]
runtime_units = { version = "0.4.1", features = ["All", "serde"] }
```
no_std is supported if the `std` feature flag is removed.

Individual unit types are supported as features, allowing you to pare down the library to what you need. Serialization is optionally supported via the `serde` feature tag, and utoipa schemas can optionally be generated via the `utoipa` tag. By default, only the base SI units are enabled (https://en.wikipedia.org/wiki/SI_base_unit).

## Quantities and Units

This library consists of three sets of data structures:
1. Unit data structures. These store data regarding unit itself, and the storage of the unit definition itself (e.g. the dimensions of the unit, and the conversion factor to convert a given unit to its base quantity). These can be serialized (if serde is enabled), and converted to a utoipa schema (if the utoipa feature flag is enabled). 
2. Scalar Quantity data structures. These store a scalar value and an associated unit. Also serializable like the units, and most math operations can be performed on them directly (e.g. scalar Mul/Div, multipliying quantites by each other, equality operators).
3. Array Quantity data structures. These store an array and an associated unit. Analogous to (2). 

# More details:

All available units are contained in the `Units` enum. Lists of available units can be easily retrieved from the `UnitTypes` enum. For each quantity processed (depending on the features you compile with), a unit enumeration is created for each unit type (e.g. `LengthUnit`, `EnergyUnit`). 

Quantities contain a value (currently restricted to f64), and the unit enumeration mentioned above to store the unit. These are then converted to `QuantityBase`. All available quantities can be encapsulated in the `Quantities` enum. A struct is created for each quantity (e.g. `Length`, `Area`), and these contain methods to convert from a `Units`, or its own internal unit enumeration. As an example, `Length` can convert from a given `LengthUnit`, and contains helper methods to convert from its current unit to any of the other enumerations (e.g. `to_meters()`, `to_kilometers()`, etc.). 


Example:

```rust
use runtime_units::{Length, Acceleration, UnitTypes, Units, QuantityBase, Time};
use runtime_units::traits::FixedQuantity;
fn example()
{
    let length = Length::meter(10.0);
    let length_cm = length.to_centimeter();
    assert_eq!(length, length_cm);
    let velocity = Acceleration::meter_per_second_squared(1.0) * Time::second(10.0);
    assert_eq!(length / Time::second(1.0), velocity); 

    let unit = Units::Length(units::LengthUnit::angstrom);
    let _ = length.try_convert(unit).unwrap();

    // list units available for Length:
    for unit in UnitTypes::Length.units()
    {
        println!("{unit}");
    }

    let quantity = Quantities::Acceleration(Acceleration::centimeter_per_second_squared(10.0));
    assert_eq!(velocity, QuantityBase::from(quantity) * Time::second(100.0));

    // Get a unit type from a string
    let _units = UnitTypes::Length.to_unit("m").unwrap();

    // Different ways to print base units of a quantity
    println!("Base Units of Velocity = {}", velocity.definition().unit_string());
    println!("Base Units of Acceleration = {}", Acceleration::meter_per_second_squared(1.0).definition().unit_string());
    
}
```
# Performance
Currently conversions on my laptop cost < 1 nsec if converting between the same quantity, and up to 3 nsec using the convert method from `Quantity`. 

# API Stability
API is largely fixed at this point, but minor changes may occur until 1.0.