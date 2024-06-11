//! Electric displacement field (base UnitDefinition coulomb per square meter, m⁻² · A · s).
use crate::{prefix, quantity};
quantity! {
    ///Electric displacement field (base UnitDefinition coulomb per square meter, m⁻² · A · s).
    quantity: ElectricDisplacementField; "electric displacement field";
    /// Dimension of electric displacement field, L⁻²TI (base UnitDefinition coulomb per square meter,
    /// m⁻² · A · s).
    dimension: ISQ<
        N2,     // length
        Z0,     // mass
        P1,     // time
        P1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @coulomb_per_square_meter: prefix!(none); "C/m²", "coulomb per square meter",
            "coulombs per square meter";
        @coulomb_per_square_centimeter: prefix!(none) / prefix!(centi) / prefix!(centi); "C/cm²",
            "coulomb per square centimeter", "coulombs per square centimeter";
    }
}

#[cfg(test)]
mod tests {
use crate::traits::Unit;
    use crate::{unit_definitions::{area::AreaUnit, electric_charge::ElectricChargeUnit, electric_displacement_field::ElectricDisplacementFieldUnit}, units_base::UnitDefinition};


    #[test]
    fn check_dimension() {
        assert_eq!(ElectricDisplacementFieldUnit::base(), ElectricChargeUnit::base() / AreaUnit::base());
    }

    #[test]
    fn check_units() {
        test_unit(ElectricChargeUnit::coulomb, AreaUnit::square_meter, ElectricDisplacementFieldUnit::coulomb_per_square_meter);
        test_unit(ElectricChargeUnit::coulomb, AreaUnit::square_centimeter, ElectricDisplacementFieldUnit::coulomb_per_square_centimeter);

    }
    fn test_unit(charge: ElectricChargeUnit, area: AreaUnit, value: ElectricDisplacementFieldUnit)
    {
         assert_eq!(Into::<UnitDefinition>::into(value), (Into::<UnitDefinition>::into(charge) / Into::<UnitDefinition>::into(area)));
    }
}
