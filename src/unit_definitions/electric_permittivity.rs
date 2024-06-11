//! Electric permittivity (base UnitDefinition farad per meter, m⁻³ · kg⁻¹ · s⁴ · A²).
use crate::{prefix, quantity};
quantity! {
    /// Electric permittivity (base UnitDefinition farad per meter, m⁻³ · kg⁻¹ · s⁴ · A²).
    quantity: ElectricPermittivity; "electric permittivity";
    /// Dimension of electric permittivity, L⁻³M⁻¹T⁴I² (base UnitDefinition farad per meter,
    /// m⁻³ · kg⁻¹ · s⁴ · A²).
    dimension: ISQ<
        N3,     // length
        N1,     // mass
        P4,     // time
        P2,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @farad_per_meter: prefix!(none); "F/m", "farad per meter", "farads per meter";
        @vacuum_electric_permittivity: 8.854_187_8128_E-12; "ε₀", "vacuum electric permittivity",
            "vacuum electric permittivity";
    }
}

#[cfg(test)]
mod test {
    use crate::traits::Unit;
    use crate::{unit_definitions::{capacitance::CapacitanceUnit, electric_permittivity::ElectricPermittivityUnit}, units::LengthUnit, units_base::UnitDefinition};


    #[test]
    fn check_dimension() {
        assert_eq!(ElectricPermittivityUnit::base(), CapacitanceUnit::base() / LengthUnit::base());
    }

    #[test]
    fn check_units() {
        test(ElectricPermittivityUnit::farad_per_meter, CapacitanceUnit::farad, LengthUnit::meter);

        fn test(value: ElectricPermittivityUnit, capacitance: CapacitanceUnit, length: LengthUnit) {
            assert_eq!(Into::<UnitDefinition>::into(value), Into::<UnitDefinition>::into(capacitance) / Into::<UnitDefinition>::into(length));
        }
    }
}
