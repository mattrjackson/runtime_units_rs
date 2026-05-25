# runtime_units

`runtime_units` is a run-time Rust library for values with units of measurement.
Most Rust unit libraries focus on compile-time dimensional analysis. If that is
what you need, a crate such as [`uom`](https://github.com/iliekturtles/uom) may
be a better fit. This crate is intended for cases where units are selected,
parsed, serialized, or converted at run time.

Much of the unit-definition data was adapted from `uom`.

## Quick Start

With default features enabled, the crate includes `std` plus a compact set of
common quantities such as `Length`, `Time`, `Mass`, and `Velocity`.

```toml
[dependencies]
runtime_units = "0.6.2"
```

```rust
use runtime_units::{Length, Time, UnitTypes, Units, Velocity};
use runtime_units::traits::FixedQuantity;
use runtime_units::units::LengthUnit;

fn main() {
    let distance = Length::meter(10.0);
    let distance_cm = distance.to_centimeter();

    assert_eq!(distance_cm.value(), 1000.0);
    assert_eq!(distance, distance_cm);

    let duration = Time::second(2.0);
    let speed = Velocity::from(distance / duration);

    assert_eq!(speed, Velocity::meter_per_second(5.0));

    let angstroms = distance
        .try_convert(Units::Length(LengthUnit::angstrom))
        .unwrap();

    assert_eq!(angstroms.unit(), LengthUnit::angstrom);

    let parsed = UnitTypes::Length.to_unit("cm").unwrap();
    assert_eq!(parsed, Units::Length(LengthUnit::centimeter));
}
```

## Features

Enable all quantities and serde support:

```toml
[dependencies]
runtime_units = { version = "0.6.2", features = ["All", "serde"] }
```

Enable only selected quantities:

```toml
[dependencies]
runtime_units = { version = "0.6.2", default-features = false, features = ["Length", "Time"] }
```

Useful feature flags:

| Feature | Purpose |
| --- | --- |
| `std` | Enabled by default. Adds `std::error::Error` support and standard-library integration. |
| `serde` | Enables serialization/deserialization for units and quantities. |
| `utoipa` | Enables OpenAPI schema generation. This feature also enables `std`. |
| `All` | Enables every generated quantity and unit type. |
| Quantity features | Enable individual quantities, such as `Length`, `Energy`, `Pressure`, or `Velocity`. Some derived quantities enable their dependencies. |

The current default feature set is `std`, `Dimensionless`, `Length`, `Mass`,
`Angle`, `Luminance`, `TemperatureInterval`, `AmountOfSubstance`,
`ElectricCurrent`, `Time`, and `Velocity`.

## no_std

`no_std` builds are supported by disabling default features:

```toml
[dependencies]
runtime_units = { version = "0.6.2", default-features = false, features = ["Length"] }
```

The crate uses `core` and `alloc` without `std`, so targets that use APIs backed
by `Vec`, `String`, parsing, or formatted errors must provide an allocator.

## Core Types

There are four common ways to work with values:

| API | Use when |
| --- | --- |
| Typed scalar quantities, such as `Length` or `Velocity` | You know the quantity type at compile time. This is the easiest and most ergonomic API. |
| `runtime_units::quantities::Quantity` | You need a run-time unit definition and scalar value. |
| `Quantities` | You need an enum that can hold any enabled typed scalar quantity. This is useful for serialization and dynamic dispatch. |
| `LengthVec`, `LengthArray<N>`, `VecQuantity`, `ArrayQuantity<N>` | You need many values that share one unit. |

Each enabled quantity generates:

- A typed scalar, such as `Length`.
- A typed unit enum, such as `LengthUnit`.
- A typed vector quantity, such as `LengthVec`.
- A typed const-generic array quantity, such as `LengthArray<N>`.
- Constructor helpers named after units, such as `Length::meter(1.0)`.
- Conversion helpers named `to_<unit>`, such as `to_centimeter()`.

## Conversions

Typed quantities use the `FixedQuantity` trait for fallible conversions from the
general `Units` enum:

```rust
use runtime_units::{Length, Units};
use runtime_units::traits::FixedQuantity;
use runtime_units::units::LengthUnit;

let length = Length::meter(1.0);
let converted = length
    .try_convert(Units::Length(LengthUnit::centimeter))
    .unwrap();

assert_eq!(converted, Length::centimeter(100.0));
```

The dynamic scalar `Quantity` uses `ArbitraryQuantity` and `UnitDefinition`:

```rust
use runtime_units::Units;
use runtime_units::quantities::Quantity;
use runtime_units::traits::ArbitraryQuantity;
use runtime_units::units::LengthUnit;

let length: Quantity = Units::Length(LengthUnit::meter).to_quantity(2.0);
let centimeters = length
    .try_convert(LengthUnit::centimeter.into())
    .unwrap();

assert_eq!(centimeters.value(), 200.0);
```

Use the `Unit` trait when you need the numeric conversion factor directly:

```rust
use runtime_units::traits::Unit;
use runtime_units::units::LengthUnit;

let factor = LengthUnit::meter
    .try_convert(LengthUnit::centimeter.into())
    .unwrap();

assert_eq!(factor, 100.0);
```

## Operations

Quantities compare equal when their base-unit values match:

```rust
use runtime_units::Length;

assert_eq!(Length::meter(1.0), Length::centimeter(100.0));
```

Addition and subtraction convert the right-hand side into the left-hand unit.
Incompatible addition/subtraction panics. Use `try_convert` first when working
with user-provided or otherwise uncertain units.

Multiplication and division combine dimensions. The direct result is the dynamic
`Quantity`; convert it into a typed quantity when the result type is known:

```rust
use runtime_units::{Length, Time, Velocity};

let distance = Length::meter(10.0);
let duration = Time::second(2.0);

let speed = Velocity::from(distance / duration);
assert_eq!(speed, Velocity::meter_per_second(5.0));
```

Because values are `f64`, prefer approximate comparisons for calculations that
accumulate floating-point error.

## Vector and Array Quantities

Typed vector and array quantities store many values with one shared unit:

```rust
use runtime_units::{LengthArray, LengthVec};

let samples = LengthVec::meter(vec![1.0, 2.0]).to_centimeter();
assert_eq!(samples.values().as_slice(), &[100.0, 200.0]);

let fixed = LengthArray::meter([1.0, 2.0]).to_centimeter();
assert_eq!(fixed.values(), &[100.0, 200.0]);
```

Use arbitrary vector/array quantities when the unit is only known at run time:

```rust
use runtime_units::quantities_vector::VecQuantity;
use runtime_units::traits::ArbitraryQuantity;
use runtime_units::units::LengthUnit;

let values = VecQuantity {
    values: vec![1.0, 2.0],
    unit: LengthUnit::meter.into(),
};

let centimeters = values
    .try_convert(LengthUnit::centimeter.into())
    .unwrap();

assert_eq!(centimeters.values.as_slice(), &[100.0, 200.0]);
```

## Dynamic Quantity Enums

`Quantities` can hold an arbitrary scalar quantity:

```rust
use runtime_units::{Length, Quantities, Units};
use runtime_units::units::LengthUnit;

let quantity = Quantities::Length(Length::meter(1.0));
let converted = quantity
    .try_convert(Units::Length(LengthUnit::centimeter))
    .unwrap();

assert_eq!(converted.value(), 100.0);
```

`UnitTypes` lists and parses units for each enabled quantity type:

```rust
use runtime_units::{UnitTypes, Units};
use runtime_units::units::LengthUnit;

assert!(UnitTypes::Length.units().contains(&"cm"));
assert_eq!(
    UnitTypes::Length.to_unit("centimeter").unwrap(),
    Units::Length(LengthUnit::centimeter)
);
```

## Serialization

Enable `serde` to serialize units and quantities:

```toml
[dependencies]
runtime_units = { version = "0.6.2", features = ["Length", "serde"] }
serde_json = "1"
```

```rust
use runtime_units::Length;

let json = serde_json::to_string(&Length::centimeter(25.0)).unwrap();
assert_eq!(json, r#"{"unit":"centimeter","value":25.0}"#);

let parsed: Length = serde_json::from_str(r#"{"unit":"cm","value":25.0}"#).unwrap();
assert_eq!(parsed, Length::centimeter(25.0));
```

Unit abbreviations, singular names, and plural names are accepted for
deserialization where aliases are generated.

## Temperature

Temperature intervals and thermodynamic temperatures are available through the
`TemperatureInterval` and `ThermodynamicTemperature` features. Be careful when
mixing absolute temperatures and temperature intervals; they represent different
physical concepts.

## Performance

Conversions are designed to be lightweight. Simple conversions are arithmetic on
`f64` values and unit multipliers. For exact performance in your workload, use
the included Criterion benchmarks or benchmark your own feature set.

## API Stability

The API is largely stable, but minor changes may occur before `1.0`.
