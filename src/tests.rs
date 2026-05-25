#[cfg(test)]
mod test
{
    #[allow(unused_imports)]
    use alloc::vec;

    #[cfg(all(any(feature="All", all(feature="Energy"))))]
    #[test]    
    fn test_from_string()
    {        
        use std::str::FromStr;
        use crate::Energy;
        use crate::Quantities;
        let unit = Quantities::from_str("10.5 kJ").unwrap();
        let unit2: Energy = serde_json::from_str( "{\"unit\":\"kJ\",\"value\":10.5}").unwrap();
        assert_eq!(Energy::try_from(unit).unwrap(), unit2);


    
    }
    #[test]
    #[cfg(any(feature="All", all(feature="Length", feature="Time", feature="Velocity")))]
    fn test_conversion()
    {
        use crate::{Length, Time, Velocity};

        let distance = Length::meter(1.0);
        let velocity = Velocity::meter_per_second(10.0);
        let time = Time::second(0.1);
        assert_eq!(time, Time::from(distance / velocity));
    }
   
    #[test]
    #[cfg(all(any(feature="All", all(feature="Time", feature="Length", feature="Velocity", feature="Acceleration")), feature="std", feature="serde"))]
    fn test_unit_serialization()
    {
        use crate::{units::LengthUnit, Length, Quantities, Units};

        assert_eq!(serde_json::json!(Length::meter(10.0)).to_string(), "{\"unit\":\"meter\",\"value\":10.0}");
        assert_eq!(serde_json::json!(Quantities::Length(Length::angstrom(1.0))).to_string(), "{\"Length\":{\"unit\":\"angstrom\",\"value\":1.0}}");

        assert_eq!(serde_json::json!(LengthUnit::centimeter), "centimeter");
        assert_eq!(serde_json::json!(Units::Length(LengthUnit::centimeter)).to_string(), "{\"Length\":\"centimeter\"}");
    }

    #[test]
    #[cfg(all(any(feature="All", all(feature="Time", feature="Pressure"))))]
    fn test_try_convert()
    {
        use crate::traits::FixedQuantity;
        use crate::{Time, Units, Pressure};         
        let pressure = Pressure::poundal_per_square_foot(1.0);
        let time = Time::second(5.0);
        assert!(pressure.try_convert(Units::Pressure(crate::units::PressureUnit::pascal)).is_ok());
        assert!(pressure.try_convert(Units::Pressure(crate::units::PressureUnit::psi)).is_ok());
        assert!(time.try_convert(Units::Length(crate::units::LengthUnit::centimeter)).is_err());
    }
    #[test]
    #[cfg(all(any(feature="All", all(feature="Time", feature="Length", feature="Velocity", feature="Acceleration", feature="Force", feature="Mass")), feature="std"))]
    fn check_ops()
    {
        use crate::{quantity::Quantity, Acceleration, Force, Length, Mass, Time, Velocity}; 
        // Check equality operator
        assert_eq!(Force::newton(1.0), Force::millinewton(1000.0));
        assert_ne!(Force::nanonewton(1.0), Force::newton(100.0));

        // Test Add trait for both a quantity and Quantity Base
        assert_eq!(Length::meter(1.0) + Length::centimeter(100.0), Length::meter(2.0));
        assert_eq!(Length::meter(1.0) + Quantity::from(Length::centimeter(100.0)), Length::centimeter(2.0e2));
        
        // Test Sub trait for Quantity and Quantity Base
        assert_eq!(Length::meter(1.0) - Velocity::meter_per_second(1.0)*Time::second(0.5), Length::centimeter(50.0));

         // Test Sub trait for Quantity and itself
         assert_eq!(Length::meter(1.0) - Length::centimeter(50.0), Length::centimeter(50.0));


        // Test add assign trait for both a quantity and Quantity Base
        let mut length_add_assign = Length::meter(5.0);
        length_add_assign += Quantity::from(Length::centimeter(200.0));
        length_add_assign += Length::kilometer(0.003);
        assert_eq!(length_add_assign, Length::meter(10.0));

        // Test Sub trait for both a quantity and Quantity Base
        let mut length_sub_assign = Length::centimeter(100.0);
        length_sub_assign -= Quantity::from(Length::meter(0.5));
        length_sub_assign -= Length::centimeter(50.0);
        assert_eq!(length_sub_assign, Length::megameter(0.0));

        // Test DivAssign and MulAssign traits.
        let mut assign_op_length = Length::meter(5.0);
        assign_op_length *= 10.0;
        assign_op_length /= 5.0;
        assert_eq!(assign_op_length, Length::meter(10.0));

        // Test Mul trait
        assert_eq!(Force::newton(1.0), Mass::kilogram(1.0) * Acceleration::meter_per_second_squared(1.0));
        
        // Test Div trait
        assert_eq!(Length::meter(2.0) / Time::second(1.0), Velocity::centimeter_per_second(200.0));

    }


    #[test]
    #[cfg(any(feature="All", feature="Length"))]
    /// Test unit enumeration -> `Units` conversion
    fn convert_unit_length_to_units()
    {
        use crate::{units::LengthUnit, Units};

        let length = LengthUnit::meter;
        let _units: Units = length.into();
    }

    #[test]
    #[cfg(any(feature="All", feature="Length"))]
    fn test_quantities_creation()
    {
        use crate::{units::LengthUnit, Length, Quantities, Units};

        let units = Units::Length(LengthUnit::angstrom);
        let quantity: crate::quantities::Quantity = Quantities::new(1.0, units).into();
        assert_eq!(quantity, Length::angstrom(1.0));
    }
    #[test]
    #[cfg(any(feature="All", feature="Length"))]
    /// Test From<Units> for Quantities
    fn test_quantities_from_units()
    {
        use crate::{units::LengthUnit, Quantities, Units};

        let units = Units::Length(LengthUnit::angstrom);
        let quantity = Quantities::from(units);        
        assert_eq!(quantity.value(), 0.0);
    }
    #[test]
    #[cfg(any(feature="All", all(feature="Length", feature="Time")))]
    /// Test `UnitTypes` -> `Units`
    fn test_convert_unit_type_to_units()
    {       
        
        use crate::{quantities::Quantity, units::LengthUnit, Length, Time, UnitTypes};

        let unit_type = UnitTypes::Length;
        let _result = Quantity::from(Length::meter(5.0)) / Quantity::from(Time::second(1.0));
        assert_eq!(unit_type.to_unit("m").unwrap(),crate::Units::Length(LengthUnit::meter));
        assert_eq!(unit_type.to_unit("meter").unwrap(), crate::Units::Length(LengthUnit::meter));
        assert_ne!(unit_type.to_unit("meter").unwrap(), crate::Units::Length(LengthUnit::kilometer));        
    }
    #[test]
    #[cfg(any(feature="All", feature="Length"))]
    /// Test `UnitTypes` -> `Units`
    fn test_convert_length()
    {               
        use crate::{quantities::Quantity, units::LengthUnit, Length, UnitTypes};
        let unit_type = UnitTypes::Length;
        let _result = Quantity::from(Length::meter(5.0));
        assert_eq!(unit_type.to_unit("m").unwrap(),crate::Units::Length(LengthUnit::meter));
        assert_eq!(unit_type.to_unit("meter").unwrap(), crate::Units::Length(LengthUnit::meter));
        assert_ne!(unit_type.to_unit("meter").unwrap(), crate::Units::Length(LengthUnit::kilometer));        
    }
    #[test]
    #[cfg(any(feature="All", feature="Length"))]
    fn test_quantities_to_base_quantity()
    {
        use crate::{quantity::Quantity, Length, Quantities};

        let quantity = Quantities::Length(Length::meter(10.0));
        assert_eq!(Quantity::from(quantity), Quantity::from(Length::centimeter(1e3)));
    }
    #[test]
    #[cfg(any(feature="All", all(feature="Length", feature="Time")))]
    fn test_quantities_conversion()
    {
        use crate::{units::LengthUnit, Length, Quantities, Time, Units};

        let length = Quantities::Length(Length::kilometer(1.0));
        let time = Quantities::Time(Time::millisecond(100.0));
        
        assert!(length.try_convert(time.unit()).is_err());
        assert!(length.try_convert(Units::Length(LengthUnit::meter)).is_ok());
        assert_eq!(length.try_convert(Units::Length(LengthUnit::meter)).unwrap().value(), 1.0e3);
    }
    
    #[test]
    #[should_panic(expected = "Addition failed due to incompatible units `m` and `s`")]
    #[cfg(any(feature="All", all(feature="Length", feature="Time")))]
    fn test_invalid_addition()
    {
        use crate::{quantity::Quantity, Length, Quantities, Time};

        let length = Quantities::Length(Length::meter(10.0));
        let time = Quantities::Time(Time::second(5.0));
        let length_quantity: Quantity = length.into();
        let time_quantity: Quantity = time.into();
        
        // Test an invalid addition
        let _result_add = length_quantity + time_quantity;
        
    }

    #[test]
    #[cfg(any(feature="All", all(feature="Length")))]
    fn test_quantity_subtraction2()
    {
        use crate::{Length, LengthArray};
        let _scalar = Length::meter(2.0) - Length::centimeter(100.0);
        let meter = LengthArray::meter([2.0;2]);
        let centimeter = LengthArray::centimeter([100.0;2]);
        let _difference = meter - centimeter;
    }
    #[test]
    #[cfg(any(feature="All", all(feature="Length", feature="Volume")))]
    fn test_quantity_addition()
    {
        use crate::traits::ArbitraryQuantity;
        use crate::units::VolumeUnit;
        use crate::{quantity::Quantity, Volume, Units};
        use crate::units_base::UnitDefinition;
        let water = Quantity::from(Volume::liter(2.0));
        let salt = Quantity::from(Volume::cubic_centimeter(100.0));

        let total = water + salt;

        let total_in_liters = total.try_convert(UnitDefinition::from(Units::Volume(VolumeUnit::liter))).unwrap();

        let total_in_m3 = total.try_convert(UnitDefinition::from(Units::Volume(VolumeUnit::cubic_meter))).unwrap();

        assert!((total_in_liters.value() - 2.1).abs() < 1e-6, "Expected approximately 2.1 liters, got {}", total_in_liters.value());
        assert!((total_in_m3.value() - 0.0021).abs() < 1e-6, "Expected approximately 0.0021 m³, got {}", total_in_m3.value());        
    }

    #[test]
    #[cfg(any(feature="All", all(feature="Length", feature="Volume")))]
    fn test_quantity_subtraction()
    {
        use crate::traits::ArbitraryQuantity;
        use crate::units::VolumeUnit;
        use crate::{quantity::Quantity, Volume, Units};
        use crate::units_base::UnitDefinition;
        let water = Quantity::from(Volume::liter(2.0));
        let salt = Quantity::from(Volume::cubic_centimeter(100.0));

        let total = water - salt;

        let total_in_liters = total.try_convert(UnitDefinition::from(Units::Volume(VolumeUnit::liter))).unwrap();

        let total_in_m3 = total.try_convert(UnitDefinition::from(Units::Volume(VolumeUnit::cubic_meter))).unwrap();

        assert!((total_in_liters.value() - 1.9).abs() < 1e-6, "Expected approximately 1.9 liters, got {}", total_in_liters.value());
        assert!((total_in_m3.value() - 0.0019).abs() < 1e-6, "Expected approximately 0.0019 m³, got {}", total_in_m3.value());
        
    }

    #[test]
    #[cfg(any(feature="All", all(feature="Length", feature="Volume")))]
    fn test_quantity_addassign()
    {
        use crate::traits::ArbitraryQuantity;
        use crate::units::VolumeUnit;
        use crate::{quantity::Quantity, Volume, Units};
        use crate::units_base::UnitDefinition;
        let mut total = Quantity::from(Volume::liter(2.0));
        let salt = Quantity::from(Volume::cubic_centimeter(100.0));

        total += salt;

        let total_in_liters = total.try_convert(UnitDefinition::from(Units::Volume(VolumeUnit::liter))).unwrap();

        let total_in_m3 = total.try_convert(UnitDefinition::from(Units::Volume(VolumeUnit::cubic_meter))).unwrap();

        assert!((total_in_liters.value() - 2.1).abs() < 1e-6, "Expected approximately 2.1 liters, got {}", total_in_liters.value());
        assert!((total_in_m3.value() - 0.0021).abs() < 1e-6, "Expected approximately 0.0021 m³, got {}", total_in_m3.value());
        
    }

    #[test]
    #[cfg(any(feature="All", all(feature="Length", feature="Volume")))]
    fn test_quantity_subassign()
    {
        use crate::traits::ArbitraryQuantity;
        use crate::units::VolumeUnit;
        use crate::{quantity::Quantity, Volume, Units};
        use crate::units_base::UnitDefinition;
        let mut total = Quantity::from(Volume::liter(2.0));
        let salt = Quantity::from(Volume::cubic_centimeter(100.0));

        total -= salt;

        let total_in_liters = total.try_convert(UnitDefinition::from(Units::Volume(VolumeUnit::liter))).unwrap();

        let total_in_m3 = total.try_convert(UnitDefinition::from(Units::Volume(VolumeUnit::cubic_meter))).unwrap();

        assert!((total_in_liters.value() - 1.9).abs() < 1e-6, "Expected approximately 1.9 liters, got {}", total_in_liters.value());
        assert!((total_in_m3.value() - 0.0019).abs() < 1e-6, "Expected approximately 0.0019 m³, got {}", total_in_m3.value());
        
    }

    #[test]
    #[cfg(any(feature="All", all(feature="Length", feature="Time")))]
    fn test_quantity_conversion()
    {
        use crate::{quantity::Quantity, traits::ArbitraryQuantity, units::LengthUnit, Length, Units};
        let length = Length::meter(10.0);
        let length_quantity: Quantity = length.into();
        let unit = Units::Length(LengthUnit::meter);
        assert_eq!(length_quantity.try_convert_unit(unit).unwrap().value(), 10.0);
        assert_eq!(length_quantity.try_convert_unit(Units::Length(LengthUnit::centimeter)).unwrap().value(), 1.0e3);
        let length_quantity2: Quantity = Length::centimeter(1.0).into();
        assert_eq!(length_quantity2.convert(LengthUnit::centimeter.into()).value(), 1.0);
    }
    #[test]
    #[cfg(any(feature="All", feature ="Energy"))]
    fn test_quantities_to_string()
    {
        use crate::{Energy, Quantities};
        let quantity = Quantities::Energy(Energy::joule(10.3));
        let string = quantity.to_string();
        assert_eq!(string, "10.3 J");
    }
    #[test]
    #[cfg(any(feature="All", feature="Length"))]
    fn available_units()
    {
        assert!(!crate::units::LengthUnit::units().is_empty());
    }
    #[test]
    #[cfg(feature="serde")]    
    #[cfg(any(feature="All", feature="Length"))]
    fn serialize_units()
    {
        use serde_json::json;
        assert_eq!(json!(crate::Length::new(1.0, crate::units::LengthUnit::centimeter)).to_string(), "{\"unit\":\"centimeter\",\"value\":1.0}");
    }
    #[cfg(all(feature="utoipa", any(feature="All", feature="Length")))]
    #[test]
    fn utoipa_test()
    {
        use utoipa::PartialSchema;
        use crate::{units::LengthUnit, LengthVec};
        let _length =LengthVec::new(vec![0.0,2.0], LengthUnit::meter);
        let _schema = LengthVec::schema();

    }
    

    #[test]
    #[cfg(any(feature="All", feature="Length"))]
    fn test_vector_quantity()
    {
        use crate::LengthVec;
        let length_meters = LengthVec::meter(vec![1.0, 2.0, 3.0]);        
        let length_centimeters = length_meters.to_centimeter();
        for (&val_m, &val_cm) in length_meters.values.iter().zip(&length_centimeters.values)
        {
            assert_eq!(val_m*100.0, val_cm);
        }        
    }

    #[test]
    #[cfg(any(feature="All", feature="Length"))]
    fn arbitrary_vec_quantity_conversion_updates_unit()
    {
        use crate::traits::ArbitraryQuantity;
        use crate::units::LengthUnit;
        use crate::vector_quantity::VecQuantity;

        let length = VecQuantity {
            unit: LengthUnit::meter.into(),
            values: vec![1.0, 2.0],
        };

        let converted = length
            .try_convert(LengthUnit::centimeter.into())
            .unwrap();

        assert_eq!(converted.values, vec![100.0, 200.0]);
        assert_eq!(converted.unit, LengthUnit::centimeter.into());
    }

    #[test]
    #[cfg(any(feature="All", feature="Length"))]
    fn arbitrary_array_quantity_conversion_updates_unit()
    {
        use crate::array_quantity::ArrayQuantity;
        use crate::traits::ArbitraryQuantity;
        use crate::units::LengthUnit;

        let length = ArrayQuantity {
            unit: LengthUnit::meter.into(),
            values: [1.0, 2.0],
        };

        let converted = length
            .try_convert(LengthUnit::centimeter.into())
            .unwrap();

        assert_eq!(converted.values, [100.0, 200.0]);
        assert_eq!(converted.unit, LengthUnit::centimeter.into());
    }

    #[test]
    #[cfg(any(feature="All", all(feature="Length", feature="Time")))]
    fn arbitrary_vec_quantity_mul_and_div_update_units()
    {
        use crate::units_base::UnitDefinition;
        use crate::units::{LengthUnit, TimeUnit};
        use crate::vector_quantity::VecQuantity;

        let length = VecQuantity {
            unit: LengthUnit::meter.into(),
            values: vec![6.0, 8.0],
        };
        let time = VecQuantity {
            unit: TimeUnit::second.into(),
            values: vec![2.0, 4.0],
        };

        let product = length.clone() * time.clone();
        assert_eq!(product.values, vec![12.0, 32.0]);
        assert_eq!(
            product.unit,
            UnitDefinition::from(LengthUnit::meter) * UnitDefinition::from(TimeUnit::second)
        );

        let quotient = length / time;
        assert_eq!(quotient.values, vec![3.0, 2.0]);
        assert_eq!(
            quotient.unit,
            UnitDefinition::from(LengthUnit::meter) / UnitDefinition::from(TimeUnit::second)
        );
    }

    #[test]
    #[cfg(any(feature="All", all(feature="Length", feature="Time")))]
    fn unit_try_convert_matches_quantity_conversion_direction()
    {
        use crate::traits::Unit;
        use crate::units::LengthUnit;

        assert_eq!(
            LengthUnit::meter
                .try_convert(LengthUnit::centimeter.into())
                .unwrap(),
            100.0
        );
    }
}
