//! Electric field (base UnitDefinition volt per meter, m ⋅ kg ⋅ s⁻³ ⋅ A⁻¹).
use crate::{prefix, quantity};
quantity! {
    /// Electric field (base UnitDefinition volt per meter, m ⋅ kg ⋅ s⁻³ ⋅ A⁻¹).
    quantity: ElectricField; "electric field";
    /// Dimension of electric field, LMT⁻³I⁻¹ (base UnitDefinition volt per meter, m ⋅ kg ⋅ s⁻³ ⋅ A⁻¹).
    dimension: ISQ<
        P1,     // length
        P1,     // mass
        N3,     // time
        N1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @volt_per_meter: prefix!(none); "V/m", "volt per meter", "volts per meter";
        @volt_per_centimeter: prefix!(none) / prefix!(centi); "V/cm", "volt per centimeter",
            "volts per centimeter";
        @volt_per_millimeter: prefix!(none) / prefix!(milli); "V/mm", "volt per millimeter",
            "volts per millimeter";
        @volt_per_micrometer: prefix!(none) / prefix!(micro); "V/μm", "volt per micrometer",
            "volts per micrometer";
        @kilovolt_per_millimeter: prefix!(kilo) / prefix!(milli); "kV/mm",
            "kilovolt per millimeter", "kilovolts per millimeter";
        @megavolt_per_meter: prefix!(mega); "MV/m", "megavolt per meter", "megavolts per meter";
        @megavolt_per_centimeter: prefix!(mega) / prefix!(centi); "MV/cm",
            "megavolt per centimeter", "megavolts per centimeter";
        @volt_per_mil: prefix!(none) / 2.54_E-5; "V/mil", "volt per mil", "volts per mil";

        /// Hartree atomic UnitDefinition of electric field Eₕ / (e ⋅ a₀), where Eₕ is Hartree energy, e is
        /// elementary charge, and a₀ is Bohr radius.
        @atomic_unit_of_electric_field: 5.142_206_747_632_595_E11; "a.u. of electric field",
            "atomic UnitDefinition of electric field", "atomic units of electric field";
    }
}

#[cfg(test)]
mod test {
    use crate::{unit_definitions::{electric_charge::ElectricChargeUnit, electric_field::ElectricFieldUnit, electric_potential::ElectricPotentialUnit, energy::EnergyUnit}, units::LengthUnit, units_base::UnitDefinition};

    #[test]
    fn check_dimension() {
        assert_eq!(ElectricFieldUnit::unit_base(), ElectricPotentialUnit::unit_base() / LengthUnit::unit_base());
    }

    #[test]
    fn check_units() {
        test_unit(ElectricPotentialUnit::volt, LengthUnit::meter, ElectricFieldUnit::volt_per_meter);
        test_unit(ElectricPotentialUnit::volt, LengthUnit::centimeter, ElectricFieldUnit::volt_per_centimeter);
        test_unit(ElectricPotentialUnit::volt, LengthUnit::millimeter, ElectricFieldUnit::volt_per_millimeter);
        test_unit(ElectricPotentialUnit::volt, LengthUnit::micrometer, ElectricFieldUnit::volt_per_micrometer);
        test_unit(ElectricPotentialUnit::kilovolt, LengthUnit::millimeter, ElectricFieldUnit::kilovolt_per_millimeter);
        test_unit(ElectricPotentialUnit::megavolt, LengthUnit::centimeter, ElectricFieldUnit::megavolt_per_centimeter);
        test_unit(ElectricPotentialUnit::megavolt, LengthUnit::meter, ElectricFieldUnit::megavolt_per_meter);
        test_unit(ElectricPotentialUnit::volt, LengthUnit::mil, ElectricFieldUnit::volt_per_mil);        
    }
    fn test_unit(potential: ElectricPotentialUnit, length: LengthUnit, value: ElectricFieldUnit)
    {
         assert_eq!(Into::<UnitDefinition>::into(value), (Into::<UnitDefinition>::into(potential) / Into::<UnitDefinition>::into(length)));
    }

    #[test]
    fn check_units_eql() {
        test(EnergyUnit::joule, ElectricChargeUnit::coulomb, LengthUnit::meter, ElectricFieldUnit::volt_per_meter);
        test(EnergyUnit::hartree, ElectricChargeUnit::elementary_charge, LengthUnit::bohr_radius, 
        ElectricFieldUnit::atomic_unit_of_electric_field);

        fn test(energy: EnergyUnit, charge: ElectricChargeUnit, length: LengthUnit, efield: ElectricFieldUnit)
        {
            assert_eq!(Into::<UnitDefinition>::into(efield), Into::<UnitDefinition>::into(energy) / Into::<UnitDefinition>::into(charge) / Into::<UnitDefinition>::into(length));
        }
    }
}
