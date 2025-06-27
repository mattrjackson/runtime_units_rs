//! Electric quadrupole moment (base UnitDefinition coulomb square meter, m² · s · A).
use crate::{prefix, quantity};
quantity! {
    /// Electric quadrupole moment (base UnitDefinition coulomb square meter, m² · s · A).
    quantity: ElectricQuadrupoleMoment; "electric quadrupole moment";
    /// Dimension of electric quadrupole moment, LTI (base UnitDefinition coulomb square meter, m² · s · A).
    dimension: ISQ[
        2.0,     // length
        0.0,     // mass
        1.0,     // time
        1.0,     // electric current
        0.0,     // thermodynamic temperature
        0.0,     // amount of substance
        0.0];    // luminous intensity
    units {
        @coulomb_square_meter: prefix!(none); "C · m²", "coulomb square meter",
            "coulomb square meters";
        @elementary_charge_barn: 1.602_176_634_E-19 * 1.0_E-28; "e · b", "elementary charge barn",
            "elementary charge barns";
        /// Hartree UnitDefinition of electric quadrupole moment e · a₀², where e is elementary charge and a₀
        /// is Bohr radius.
        @atomic_unit_of_electric_quadrupole_moment: 4.486_551_524_613_E-40;  "e · a₀²",
            "atomic UnitDefinition of electric quadrupole moment",
            "atomic units of electric quadrupole moment";
    }
}

#[cfg(test)]
mod test {
    use crate::traits::Unit;
    use crate::{unit_definitions::{area::AreaUnit, electric_charge::ElectricChargeUnit, electric_quadrupole_moment::ElectricQuadrupoleMomentUnit}, units_base::UnitDefinition};


    #[test]
    fn check_dimension() {
        assert_eq!(ElectricQuadrupoleMomentUnit::base(), ElectricChargeUnit::base() * AreaUnit::base());
    }

    #[test]
    fn check_units() {
        test(ElectricChargeUnit::coulomb, AreaUnit::square_meter, ElectricQuadrupoleMomentUnit::coulomb_square_meter);
        test(ElectricChargeUnit::elementary_charge, AreaUnit::barn, ElectricQuadrupoleMomentUnit::elementary_charge_barn);

        fn test(charge: ElectricChargeUnit, area: AreaUnit, value: ElectricQuadrupoleMomentUnit) {
            assert_eq!(Into::<UnitDefinition>::into(value), Into::<UnitDefinition>::into(charge) * Into::<UnitDefinition>::into(area));
        }
    }
}
