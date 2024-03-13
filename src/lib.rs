use quantity::QuantityBase;


pub mod errors;
pub(crate) mod quantity;
pub(crate) mod macros;
pub mod units_base;
mod unit_definitions;
pub use crate::unit_definitions::*;
mod tests;

// #[cfg(all(any(feature="All", all(feature="Time", feature="Length", feature="Velocity", feature="Acceleration")), feature="std"))]
// #[test]
// fn example()
// {
//     let length = Length::meter(10.0);
//     let length_cm = length.to_centimeter();
//     assert_eq!(length, length_cm);
//     let velocity = Acceleration::meter_per_second_squared(1.0) * Time::second(10.0);
//     assert_eq!(length / Time::second(1.0), velocity); 

//     let unit = Units::Length(units::LengthUnit::angstrom);
//     let _ = length.try_convert(unit).unwrap();

//     // list units available for Length:
//     for unit in UnitTypes::Length.units()
//     {
//         println!("{unit}");
//     }

//     let quantity = Quantities::Acceleration(Acceleration::centimeter_per_second_squared(10.0));
//     assert_eq!(velocity, QuantityBase::from(quantity) * Time::second(100.0));

//     // Get a unit type from a string
//     let _units = UnitTypes::Length.to_unit("m").unwrap();

//     // Different ways to print base units of a quantity
//     println!("Base Units of Velocity = {}", velocity.definition().unit_string());
//     println!("Base Units of Acceleration = {}", Acceleration::meter_per_second_squared(1.0).definition().unit_string());
    

// }
