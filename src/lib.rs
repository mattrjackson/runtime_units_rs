use quantity::Quantity;


pub mod errors;
pub(crate) mod quantity;
pub(crate) mod macros;
pub mod units_base;
mod unit_definitions;
pub use crate::unit_definitions::*;

#[cfg(test)]
#[cfg(any(feature="All", all(feature="Time", feature="Length", feature="Velocity", feature="Acceleration")))]
mod test
{

    
    #[test]
    #[cfg(any(feature="All", all(feature="Time", feature="Length", feature="Velocity", feature="Acceleration")))]
    fn readme_example()
    {
        use crate::quantities::{LengthQuantity, TimeQuantity };
        use crate::{Length, Time, Units, Velocity};         
        let length = Length::meter(1.0);
        let time = Time::second(5.0);
        let velocity  = Velocity::meter_per_second(0.2).to_quantity();
        assert!(length.to_quantity()/time.to_quantity() == velocity);


        let length_quantity = LengthQuantity::meter(10.0);
        let time_quantity = TimeQuantity::second(1.0);
        let quantity = length_quantity*time_quantity;
        let velocity_quantity = crate::quantities::VelocityQuantity::meter_per_second(5.0).to_foot_per_second();
        assert!(length_quantity.to_foot() / time_quantity == velocity_quantity * 2.0);
        let start = std::time::Instant::now();
        for _i in 0..1e9 as usize
        {
            let _quantity = length_quantity.to_foot();
        }
        let ending = std::time::Instant::now();
        println!("{}", ending.duration_since(start).as_nanos());
        let quantity = *length_quantity;        
        assert!(quantity.convert(Units::Acceleration(crate::units::AccelerationUnit::centimeter_per_second_squared)).is_err());
        println!("{:?}", quantity.convert(Units::Luminance(crate::units::LuminanceUnit::candela_per_square_centimeter)).err());
        assert!(quantity.convert(Units::Length(crate::units::LengthUnit::angstrom)).is_ok());
    }

    #[test]
    #[cfg(any(feature="All", feature="Length"))]
    fn available_units()
    {
        for unit in crate::units::LengthUnit::units()
        {
            println!("{unit}");
        }
    }
    #[test]
    #[cfg(feature="serde")]    
    #[cfg(any(feature="All", feature="Length"))]
    fn serialize_units()
    {
        use serde_json::json;
        println!("{}",json!(crate::Length::new(1.0, crate::units::LengthUnit::centimeter)));
    }
}