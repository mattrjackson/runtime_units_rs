//! Electric quadrupole moment (base unit coulomb square meter, m² · s · A).
use crate::{prefix, quantity};
quantity! {
    /// Electric quadrupole moment (base unit coulomb square meter, m² · s · A).
    quantity: ElectricQuadrupoleMoment; "electric quadrupole moment";
    /// Dimension of electric quadrupole moment, LTI (base unit coulomb square meter, m² · s · A).
    dimension: ISQ<
        P2,     // length
        Z0,     // mass
        P1,     // time
        P1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @coulomb_square_meter: prefix!(none); "C · m²", "coulomb square meter",
            "coulomb square meters";
        @elementary_charge_barn: 1.602_176_634_E-19 * 1.0_E-28; "e · b", "elementary charge barn",
            "elementary charge barns";
        /// Hartree unit of electric quadrupole moment e · a₀², where e is elementary charge and a₀
        /// is Bohr radius.
        @atomic_unit_of_electric_quadrupole_moment: 4.486_551_524_613_E-40;  "e · a₀²",
            "atomic unit of electric quadrupole moment",
            "atomic units of electric quadrupole moment";
    }
}

#[cfg(test)]
mod test {
    use crate::{unit_definitions::{area::AreaUnit, electric_charge::ElectricChargeUnit, electric_quadrupole_moment::ElectricQuadrupoleMomentUnit}, units::LengthUnit, units_base::Unit};


    #[test]
    fn check_dimension() {
        assert_eq!(ElectricQuadrupoleMomentUnit::unit_base(), ElectricChargeUnit::unit_base() * AreaUnit::unit_base());
    }

    #[test]
    fn check_units() {
        test(ElectricChargeUnit::coulomb, AreaUnit::square_meter, ElectricQuadrupoleMomentUnit::coulomb_square_meter);
        test(ElectricChargeUnit::elementary_charge, AreaUnit::barn, ElectricQuadrupoleMomentUnit::elementary_charge_barn);

        fn test(charge: ElectricChargeUnit, area: AreaUnit, value: ElectricQuadrupoleMomentUnit) {
            assert_eq!(Into::<Unit>::into(value), Into::<Unit>::into(charge) * Into::<Unit>::into(area));
        }
    }
}
