//! Areal number rate (base UnitDefinition 1 per square meter second, m⁻² · s⁻¹).
use crate::{prefix, quantity};
quantity! {
    /// Areal number rate (base UnitDefinition 1 per square meter second, m⁻² · s⁻¹).
    quantity: ArealNumberRate; "areal number rate";
    /// Dimension of areal number rate, L⁻²T⁻¹ (base UnitDefinition 1 per square meter second, m⁻² · s⁻¹).
    dimension: ISQ[
        -2.0,     // length
        0.0,     // mass
        -1.0,     // time
        0.0,     // electric current
        0.0,     // thermodynamic temperature
        0.0,     // amount of substance
        0.0];    // luminous intensity
    kind: dyn (crate::si::marker::ConstituentConcentrationKind);
    units {
        @per_square_meter_second: prefix!(none); "m⁻² · s⁻¹", "per square meter second",
            "per square meter second";
        @per_square_centimeter_second: prefix!(none) / prefix!(centi) / prefix!(centi);
            "cm⁻² · s⁻¹", "per square centimeter second", "per square centimeter second";

        @per_acre_second: prefix!(none) / 4.046_873_E3; "ac⁻¹ · s⁻¹", "per acre second",
            "per acre second";
        @per_are_second: prefix!(none) / 1.0_E2; "a⁻¹ · s⁻¹", "per are second", "per are second";
        @per_barn_second: prefix!(none) / 1.0_E-28; "b⁻¹ · s⁻¹", "per barn second",
            "per barn second";
        @per_circular_mil_second: prefix!(none) / 5.067_075_E-10; "cmil⁻¹ · s⁻¹",
            "per circular mil second", "per circular mil second";
        @per_hectare_second: prefix!(none) / 1.0_E4; "ha⁻¹ · s⁻¹", "per hectare second",
            "per hectare second";
        @per_square_foot_second: prefix!(none) / 9.290_304_E-2; "ft⁻² · s⁻¹",
            "per square foot second", "per square foot second";
        @per_square_inch_second: prefix!(none) / 6.451_6_E-4; "in⁻² · s⁻¹",
            "per square inch second", "per square inch second";
        @per_square_mile_second: prefix!(none) / 2.589_988_E6; "mi⁻² · s⁻¹",
            "per square mile second", "per square mile second";
        @per_square_yard_second: prefix!(none) / 8.361_274_E-1; "yd⁻² · s⁻¹",
            "per square yard second", "per square yard second";
    }
}

#[cfg(test)]
mod test {
    use crate::traits::Unit;
    use crate::{unit_definitions::{areal_number_rate::ArealNumberRateUnit, time::TimeUnit}, units::{AreaUnit, LengthUnit}, units_base::UnitDefinition};

   

    #[test]
    fn check_dimension() {
        assert_eq!(ArealNumberRateUnit::base(),  LengthUnit::base().powi(-2) / TimeUnit::base());
    }

    #[test]
    fn check_units() {
        test_unit(ArealNumberRateUnit::per_square_meter_second, AreaUnit::square_meter, TimeUnit::second);
        test_unit(ArealNumberRateUnit::per_square_centimeter_second, AreaUnit::square_centimeter, TimeUnit::second);

        test_unit(ArealNumberRateUnit::per_acre_second, AreaUnit::acre, TimeUnit::second);
        test_unit(ArealNumberRateUnit::per_are_second, AreaUnit::are, TimeUnit::second);
        test_unit(ArealNumberRateUnit::per_barn_second, AreaUnit::barn, TimeUnit::second);
        test_unit(ArealNumberRateUnit::per_circular_mil_second, AreaUnit::circular_mil, TimeUnit::second);
        test_unit(ArealNumberRateUnit::per_hectare_second, AreaUnit::hectare, TimeUnit::second);
        test_unit(ArealNumberRateUnit::per_square_foot_second, AreaUnit::square_foot, TimeUnit::second);
        test_unit(ArealNumberRateUnit::per_square_inch_second, AreaUnit::square_inch, TimeUnit::second);
        test_unit(ArealNumberRateUnit::per_square_mile_second, AreaUnit::square_mile, TimeUnit::second);
        test_unit(ArealNumberRateUnit::per_square_yard_second, AreaUnit::square_yard, TimeUnit::second);

        fn test_unit(value: ArealNumberRateUnit, area: AreaUnit, time: TimeUnit) {
            assert!(Into::<UnitDefinition>::into(value).approx_eq(Into::<UnitDefinition>::into(area).powi(-1) / Into::<UnitDefinition>::into(time) , 1e-12));
        }
    }
}
