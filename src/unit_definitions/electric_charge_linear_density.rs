//! Electric charge linear density (base unit coulomb per meter, m⁻¹ · A · s).
use crate::{prefix, quantity};
quantity! {
    ///Electric charge linear density (base unit coulomb per meter, m⁻¹ · A · s).
    quantity: ElectricChargeLinearDensity; "electric charge linear density";
    /// Dimension of electric charge linear density, L⁻¹TI (base unit coulomb per meter,
    /// m⁻¹ · A · s).
    dimension: ISQ<
        N1,     // length
        Z0,     // mass
        P1,     // time
        P1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
        kind: dyn (crate::si::marker::ConstituentConcentrationKind);
    units {
        @coulomb_per_meter: prefix!(none); "C/m", "coulomb per meter", "coulombs per meter";
        @coulomb_per_centimeter: prefix!(none) / prefix!(centi); "C/cm", "coulomb per centimeter",
            "coulombs per centimeter";
    }
}

#[cfg(test)]
mod tests {
    use crate::{unit_definitions::{electric_charge::ElectricChargeUnit, electric_charge_linear_density::ElectricChargeLinearDensityUnit}, units::LengthUnit, units_base::Unit};

    #[test]
    fn check_dimension() {
        assert_eq!(ElectricChargeLinearDensityUnit::unit_base(), ElectricChargeUnit::unit_base() / LengthUnit::unit_base());
    }

    #[test]
    fn check_units() {
        test_unit(ElectricChargeUnit::coulomb, LengthUnit::meter, ElectricChargeLinearDensityUnit::coulomb_per_meter);
        test_unit(ElectricChargeUnit::coulomb, LengthUnit::centimeter, ElectricChargeLinearDensityUnit::coulomb_per_centimeter);
    }
    fn test_unit(charge: ElectricChargeUnit, length: LengthUnit, value: ElectricChargeLinearDensityUnit)
    {
        assert_eq!(Into::<Unit>::into(value), (Into::<Unit>::into(charge) / Into::<Unit>::into(length)));
    }
}
