//! Dynamic viscosity (base UnitDefinition pascal second, kg · m⁻¹ · s⁻¹).
use crate::{prefix, quantity};
quantity! {
    /// Dynamic viscosity (base UnitDefinition pascal second, kg · m⁻¹ · s⁻¹).
    quantity: DynamicViscosity; "dynamic viscosity";
    /// Dimension of dynamic viscosity, L⁻¹MT⁻¹ (base UnitDefinition pascal second, kg · m⁻¹ · s⁻¹).
    dimension: ISQ[
        -1.0,     // length
        1.0,     // mass
        -1.0,     // time
        0.0,     // electric current
        0.0,     // thermodynamic temperature
        0.0,     // amount of substance
        0.0];    // luminous intensity
    units {
        @pascal_second: prefix!(none); "Pa · s", "pascal second", "pascal seconds";
        @millipascal_second: prefix!(milli); "mPa · s", "millipascal second", "millipascal seconds";
        @micropascal_second: prefix!(micro); "µPa · s", "micropascal second", "micropascal seconds";
        // poise = 0.1 Pa · s
        @poise: 1.0_E-1; "P", "poise", "poises";
        // centipoise = 1 mPa · s
        @centipoise: prefix!(centi) * 1.0_E-1; "cP", "centipoise", "centipoises";
        @pound_force_second_per_square_foot: 4.448_222_E0 / 3.048_E-1 / 3.048_E-1; "lbf · s/ft²",
            "pound-force second per square foot", "pound-force seconds per square foot";
        @pound_force_second_per_square_inch: 4.448_222_E0 / 2.54_E-2 / 2.54_E-2; "lbf · s/in²",
            "pound-force second per square inch", "pound-force seconds per square inch";
        // Reyn = 1 lbf · s/in²
        @reyn: 4.448_222_E0 / 2.54_E-2 / 2.54_E-2; "reyn", "reyn", "reyns";
        @pound_per_foot_second: 4.535_924_E-1 / 3.048_E-1; "lb/(ft · s)", "pound per foot second",
            "pounds per foot second";
        @pound_per_foot_hour: 4.535_924_E-1 / 3.048_E-1 / 3.6_E3; "lb/(ft · h)",
            "pound per foot hour", "pounds per foot hour";
        @slug_per_foot_second: 1.459_390_E1 / 3.048_E-1; "slug/(ft · s)", "slug per foot second",
            "slugs per foot second";
        @gram_per_centimeter_second: prefix!(milli) / prefix!(centi); "g/(cm · s)",
            "gram per centimeter second", "grams per centimeter second";
    }
}

#[cfg(test)]
mod test {
    use crate::traits::Unit;
    use crate::{unit_definitions::{dynamic_viscosity::DynamicViscosityUnit, time::TimeUnit}, units::{LengthUnit, MassUnit, PressureUnit}, units_base::UnitDefinition};


    #[test]
    fn check_dimension() {
        assert_eq!(DynamicViscosityUnit::base(), PressureUnit::base() * TimeUnit::base());
    }

    #[test]
    fn check_units() {
        test_unit(PressureUnit::pascal, TimeUnit::second, DynamicViscosityUnit::pascal_second);
        test_unit(PressureUnit::millipascal, TimeUnit::second, DynamicViscosityUnit::millipascal_second);
        test_unit(PressureUnit::micropascal, TimeUnit::second, DynamicViscosityUnit::micropascal_second);
        test_unit(PressureUnit::dyne_per_square_centimeter, TimeUnit::second, DynamicViscosityUnit::poise);
        test_unit(PressureUnit::millipascal, TimeUnit::second,DynamicViscosityUnit::centipoise);
        test_unit(PressureUnit::pound_force_per_square_foot, TimeUnit::second,
            DynamicViscosityUnit::pound_force_second_per_square_foot);
        test_unit(PressureUnit::pound_force_per_square_inch, TimeUnit::second,
            DynamicViscosityUnit::pound_force_second_per_square_inch);
        test_unit(PressureUnit::pound_force_per_square_inch, TimeUnit::second, DynamicViscosityUnit::reyn);
    }

    #[test]
    fn check_units_mlt() {
        test_unit_mlt(MassUnit::pound, LengthUnit::foot, TimeUnit::second, DynamicViscosityUnit::pound_per_foot_second);
        test_unit_mlt(MassUnit::pound, LengthUnit::foot, TimeUnit::hour, DynamicViscosityUnit::pound_per_foot_hour);
        test_unit_mlt(MassUnit::gram, LengthUnit::centimeter, TimeUnit::second, DynamicViscosityUnit::gram_per_centimeter_second);
        test_unit_mlt(MassUnit::slug, LengthUnit::foot, TimeUnit::second, DynamicViscosityUnit::slug_per_foot_second);
    }
    fn test_unit(pressure: PressureUnit, time: TimeUnit, value: DynamicViscosityUnit)
    {
        assert!(Into::<UnitDefinition>::into(value).approx_eq(Into::<UnitDefinition>::into(pressure) * Into::<UnitDefinition>::into(time), 1e-12));
    }
    fn test_unit_mlt(mass: MassUnit, length: LengthUnit, time: TimeUnit, value: DynamicViscosityUnit) {
        assert!(Into::<UnitDefinition>::into(value).approx_eq(Into::<UnitDefinition>::into(mass) / Into::<UnitDefinition>::into(length) / Into::<UnitDefinition>::into(time), 1e-12));
    }
}

