//! Electric current density (base UnitDefinition ampere per square meter, m⁻² · A).
use crate::{prefix, quantity};
quantity! {
    /// Electric current density (base UnitDefinition ampere per square meter, m⁻² · A).
    quantity: ElectricCurrentDensity; "electric current density";
    /// Dimension of electric current density, L⁻²I (base UnitDefinition ampere per square meter, A · m⁻²).
    dimension: ISQ<
        N2,     // length
        Z0,     // mass
        Z0,     // time
        P1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
        kind: dyn (crate::si::marker::ConstituentConcentrationKind);
    units {
        @ampere_per_square_meter: prefix!(none); "A/m²", "ampere per square meter",
            "amperes per square meter";
        @ampere_per_square_centimeter: prefix!(none) / prefix!(centi) / prefix!(centi); "A/cm²",
            "ampere per square centimeter", "amperes per square centimeter";
        @ampere_per_square_millimeter: prefix!(none) / prefix!(milli) / prefix!(milli); "A/mm²",
            "ampere per square millimeter", "amperes per square millimeter";
    }
}

#[cfg(test)]
mod tests {
use crate::traits::Unit;
    use crate::{unit_definitions::{electric_current::ElectricCurrentUnit, electric_current_density::ElectricCurrentDensityUnit}, units::AreaUnit, units_base::UnitDefinition};

    
    #[test]
    fn check_dimension() {
        assert_eq!(ElectricCurrentDensityUnit::base(), ElectricCurrentUnit::base() / AreaUnit::base());
    }

    #[test]
    fn check_units() {
        test_unit(ElectricCurrentDensityUnit::ampere_per_square_meter, ElectricCurrentUnit::ampere, AreaUnit::square_meter);
        test_unit(ElectricCurrentDensityUnit::ampere_per_square_centimeter, ElectricCurrentUnit::ampere, AreaUnit::square_centimeter);
        test_unit(ElectricCurrentDensityUnit::ampere_per_square_millimeter, ElectricCurrentUnit::ampere, AreaUnit::square_millimeter);
    }
    fn test_unit(value: ElectricCurrentDensityUnit, current: ElectricCurrentUnit, area: AreaUnit)
    {
        assert_eq!(Into::<UnitDefinition>::into(value), (Into::<UnitDefinition>::into(current) / Into::<UnitDefinition>::into(area)));
    }
}
