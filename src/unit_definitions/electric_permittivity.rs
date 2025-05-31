//! Electric permittivity (base UnitDefinition farad per meter, m⁻³ · kg⁻¹ · s⁴ · A²).
use crate::{prefix, quantity};
quantity! {
    /// Electric permittivity (base UnitDefinition farad per meter, m⁻³ · kg⁻¹ · s⁴ · A²).
    quantity: ElectricPermittivity; "electric permittivity";
    /// Dimension of electric permittivity, L⁻³M⁻¹T⁴I² (base UnitDefinition farad per meter,
    /// m⁻³ · kg⁻¹ · s⁴ · A²).
    dimension: ISQ[
        -3.0,     // length
        -1.0,     // mass
        4.0,     // time
        2.0,     // electric current
        0.0,     // thermodynamic temperature
        0.0,     // amount of substance
        0.0];    // luminous intensity
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
