//! Electric charge volumetric density (base UnitDefinition coulomb per cubic meter, m⁻³ · A · s).
use crate::{prefix, quantity};
quantity! {
    ///Electric charge volumetric density (base UnitDefinition coulomb per cubic meter, m⁻³ · A · s).
    quantity: ElectricChargeVolumetricDensity; "electric charge volumetric density";
    /// Dimension of electric charge volumetric density, TIL⁻³ (base UnitDefinition coulomb per cubic meter,
    /// m⁻³ · A · s).
    dimension: ISQ<
        N3,     // length
        Z0,     // mass
        P1,     // time
        P1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
        kind: dyn (crate::si::marker::ConstituentConcentrationKind);
    units {
        @coulomb_per_cubic_meter: prefix!(none); "C/m³", "coulomb per cubic meter",
            "coulombs per cubic meter";
        @coulomb_per_cubic_centimeter:
            prefix!(none) / prefix!(centi) / prefix!(centi) / prefix!(centi); "C/cm³",
            "coulomb per cubic centimeter", "coulombs per cubic centimeter";
    }
}

#[cfg(test)]
mod tests 
{
    use crate::{unit_definitions::{electric_charge::ElectricChargeUnit, electric_charge_volumetric_density::ElectricChargeVolumetricDensityUnit, volume::VolumeUnit}, units_base::UnitDefinition};

    #[test]
    fn check_dimension() {
        assert_eq!(ElectricChargeVolumetricDensityUnit::unit_base(), ElectricChargeUnit::unit_base() / VolumeUnit::unit_base());
    }

    #[test]
    fn check_units() {
        test_unit(ElectricChargeUnit::coulomb, VolumeUnit::cubic_meter, ElectricChargeVolumetricDensityUnit::coulomb_per_cubic_meter);
        test_unit(ElectricChargeUnit::coulomb, VolumeUnit::cubic_centimeter, ElectricChargeVolumetricDensityUnit::coulomb_per_cubic_centimeter);
    }
    fn test_unit(charge: ElectricChargeUnit, volume: VolumeUnit, value: ElectricChargeVolumetricDensityUnit)
    {
        assert!(Into::<UnitDefinition>::into(value).approx_eq(Into::<UnitDefinition>::into(charge) / Into::<UnitDefinition>::into(volume), 1e-12));
    }
}
