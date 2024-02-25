use std::ops::{Deref, DerefMut};

use quantity::Quantity;


pub(crate) mod quantity;
pub(crate) mod macros;
pub mod units_base;
mod unit_definitions;
pub use crate::unit_definitions::*;

#[cfg(test)]
#[cfg(any(feature="all", all(feature="Time", feature="Length", feature="Energy")))]
mod test
{
    use crate::quantities::{LengthQuantity, TimeQuantity, Quantity};
    use crate::units::EnergyUnit;
    #[test]
    fn time_conversion()
    {                
        let energy = EnergyUnit::joule;
        let mut quantity = LengthQuantity::new(1.0, crate::units::LengthUnit::meter);
        println!("{:?}", quantity.to_centimeter());
        let start = std::time::Instant::now();
        for _i in 0..1e9 as usize
        {
            quantity = quantity.to_centimeter().to_meter();
            assert!(f64::abs(1.0-quantity.value/1.0) < 1e-6);
        }
        let ending = std::time::Instant::now();
        println!("{}", ending.duration_since(start).as_nanos());
        println!("{:?}", quantity);
    }
    #[test]
    fn time_conversion2()
    {
        let energy = crate::units::EnergyUnit::joule;
        let mut quantity = *LengthQuantity::new(1.0, crate::units::LengthUnit::meter);        
        let start = std::time::Instant::now();
        for _i in 0..1e7 as usize
        {
            quantity /= *TimeQuantity::second(1.0);
            quantity *= *TimeQuantity::second(1.0);
            assert!(f64::abs(1.0-quantity.value/1.0) < 1e-6);
        }
        let ending = std::time::Instant::now();
        println!("{}", ending.duration_since(start).as_nanos());
        println!("{:?}", quantity);
    } 

    #[test]
    fn available_units()
    {
        for unit in crate::units::EnergyUnit::units()
        {
            println!("{unit}");
        }
    }
    #[test]
    #[cfg(feature="serde")]    
    fn serialize_units()
    {
        use serde_json::json;
        println!("{}",json!(crate::Length::new(1.0, crate::units::LengthUnit::centimeter)));
    }
}