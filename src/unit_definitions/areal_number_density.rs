//! Areal number density (base unit 1 per square meter, m⁻²).
use crate::{prefix, quantity};
quantity! {
    /// Areal number density (base unit 1 per square meter, m⁻²).
    quantity: ArealNumberDensity; "areal number density";
    /// Dimension of areal number density, L⁻² (base unit 1 per square meter, m⁻²).
    dimension: ISQ<
        N2,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::ConstituentConcentrationKind);
    units {
        @per_square_kilometer: prefix!(none) / prefix!(kilo) / prefix!(kilo); "km⁻²",
            "per square kilometer", "per square kilometer";
        @per_square_meter: prefix!(none); "m⁻²", "per square meter", "per square meter";
        @per_square_decimeter: prefix!(none) / prefix!(deci) / prefix!(deci); "dm⁻²",
            "per square decimeter", "per square decimeter";
        @per_square_centimeter: prefix!(none) / prefix!(centi) / prefix!(centi); "cm⁻²",
            "per square centimeter", "per square centimeter";
        @per_square_millimeter: prefix!(none) / prefix!(milli) / prefix!(milli); "mm⁻²",
            "per square millimeter", "per square millimeter";
        @per_square_micrometer: prefix!(none) / prefix!(micro) / prefix!(micro); "µm⁻²",
            "per square micrometer", "per square micrometer";

        @per_acre: prefix!(none) / 4.046_873_E3; "ac⁻²", "per acre", "per acre";
        @per_are: prefix!(none) / 1.0_E2; "a⁻²", "per are", "per are";
        @per_barn: prefix!(none) / 1.0_E-28; "b⁻²", "per barn", "per barn";
        @per_circular_mil: prefix!(none) / 5.067_075_E-10; "cmil⁻²", "per circular mil",
            "per circular mil";
        @per_hectare: prefix!(none) / 1.0_E4; "ha⁻²", "per hectare", "per hectare";
        @per_square_foot: prefix!(none) / 9.290_304_E-2; "ft⁻²", "per square foot",
            "per square foot";
        @per_square_inch: prefix!(none) / 6.451_6_E-4; "in⁻²", "per square inch", "per square inch";
        @per_square_mile: prefix!(none) / 2.589_988_E6; "mi⁻²", "per square mile",
            "per square mile";
        @per_square_yard: prefix!(none) / 8.361_274_E-1; "yd⁻²", "per square yard",
            "per square yard";
    }
}

#[cfg(test)]
mod test {
    use crate::{unit_definitions::areal_number_density::ArealNumberDensityUnit, units::{AreaUnit, LengthUnit}, units_base::Unit};
   

    #[test]
    fn check_dimension() {
        assert_eq!(ArealNumberDensityUnit::unit(),  LengthUnit::unit().powi(-2));
    }

    #[test]
    fn check_units() {
        test_unit(ArealNumberDensityUnit::per_square_kilometer, AreaUnit::square_kilometer);
        test_unit(ArealNumberDensityUnit::per_square_meter, AreaUnit::square_meter);
        test_unit(ArealNumberDensityUnit::per_square_decimeter, AreaUnit::square_decimeter);
        test_unit(ArealNumberDensityUnit::per_square_centimeter, AreaUnit::square_centimeter);
        test_unit(ArealNumberDensityUnit::per_square_millimeter, AreaUnit::square_millimeter);
        test_unit(ArealNumberDensityUnit::per_square_micrometer, AreaUnit::square_micrometer);

        test_unit(ArealNumberDensityUnit::per_acre, AreaUnit::acre);
        test_unit(ArealNumberDensityUnit::per_are, AreaUnit::are);
        test_unit(ArealNumberDensityUnit::per_barn, AreaUnit::barn);
        test_unit(ArealNumberDensityUnit::per_circular_mil, AreaUnit::circular_mil);
        test_unit(ArealNumberDensityUnit::per_hectare, AreaUnit::hectare);
        test_unit(ArealNumberDensityUnit::per_square_foot, AreaUnit::square_foot);
        test_unit(ArealNumberDensityUnit::per_square_inch, AreaUnit::square_inch);
        test_unit(ArealNumberDensityUnit::per_square_mile, AreaUnit::square_mile);
        test_unit(ArealNumberDensityUnit::per_square_yard, AreaUnit::square_yard);

        fn test_unit(value: ArealNumberDensityUnit, area: AreaUnit) {
            assert!(Into::<Unit>::into(value).approx_eq(Into::<Unit>::into(area).powi(-1), 1e-12));
        }
    }
    
}
