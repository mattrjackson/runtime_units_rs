//! Electric charge linear density (base UnitDefinition coulomb per meter, m⁻¹ · A · s).
use crate::{prefix, quantity};
quantity! {
    ///Electric charge linear density (base UnitDefinition coulomb per meter, m⁻¹ · A · s).
    quantity: ElectricChargeLinearDensity; "electric charge linear density";
    /// Dimension of electric charge linear density, L⁻¹TI (base UnitDefinition coulomb per meter,
    /// m⁻¹ · A · s).
    dimension: ISQ[
        -1.0,     // length
        0.0,     // mass
        1.0,     // time
        1.0,     // electric current
        0.0,     // thermodynamic temperature
        0.0,     // amount of substance
        0.0];    // luminous intensity
        kind: dyn (crate::si::marker::ConstituentConcentrationKind);
    units {
        @coulomb_per_meter: prefix!(none); "C/m", "coulomb per meter", "coulombs per meter";
        @coulomb_per_centimeter: prefix!(none) / prefix!(centi); "C/cm", "coulomb per centimeter",
            "coulombs per centimeter";
    }
}

#[cfg(test)]
mod tests {
use crate::traits::Unit;
    use crate::{unit_definitions::{electric_charge::ElectricChargeUnit, electric_charge_linear_density::ElectricChargeLinearDensityUnit}, units::LengthUnit, units_base::UnitDefinition};

    #[test]
    fn check_dimension() {
        assert_eq!(ElectricChargeLinearDensityUnit::base(), ElectricChargeUnit::base() / LengthUnit::base());
    }

    #[test]
    fn check_units() {
        test_unit(ElectricChargeUnit::coulomb, LengthUnit::meter, ElectricChargeLinearDensityUnit::coulomb_per_meter);
        test_unit(ElectricChargeUnit::coulomb, LengthUnit::centimeter, ElectricChargeLinearDensityUnit::coulomb_per_centimeter);
    }
    fn test_unit(charge: ElectricChargeUnit, length: LengthUnit, value: ElectricChargeLinearDensityUnit)
    {
        assert_eq!(Into::<UnitDefinition>::into(value), (Into::<UnitDefinition>::into(charge) / Into::<UnitDefinition>::into(length)));
    }
}
