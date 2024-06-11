//! Electric dipole moment (base UnitDefinition coulomb meter, m · s · A).
use crate::{prefix, quantity};
quantity! {
    /// Electric dipole moment (base UnitDefinition coulomb meter, m · s · A).
    quantity: ElectricDipoleMoment; "electric dipole moment";
    /// Dimension of electric dipole moment, LTI (base UnitDefinition coulomb meter, m · s · A).
    dimension: ISQ<
        P1,     // length
        Z0,     // mass
        P1,     // time
        P1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @coulomb_meter: prefix!(none); "C · m", "coulomb meter", "coulomb meters";

        @atomic_unit_of_charge_centimeter: 1.602_176_634_E-19 * prefix!(centi);
            "a.u. of charge · cm", "atomic UnitDefinition of charge centimeter",
            "atomic UnitDefinition of charge centimeters";
        @elementary_charge_centimeter: 1.602_176_634_E-19 * prefix!(centi); "e · cm",
            "elementary charge centimeter", "elementary charge centimeters";
        @debye: 1.0 / 299_792_458.0 * 1.0_E-21; "D", "debye", "debyes";
        /// Hartree UnitDefinition of electric dipole moment e·a₀, where e is elementary charge and a₀ is Bohr
        /// radius.
        @atomic_unit_of_electric_dipole_moment: 8.478_353_625_540_766_E-30; "e · a₀",
            "atomic UnitDefinition of electric dipole moment", "atomic units of electric dipole moment";
    }
}

#[cfg(test)]
mod tests {
use crate::traits::Unit;
    use crate::{unit_definitions::{electric_charge::ElectricChargeUnit, electric_dipole_moment::{ElectricDipoleMoment, ElectricDipoleMomentUnit}, length::Length}, units::LengthUnit, units_base::UnitDefinition, ElectricCharge};
    #[test]
    fn check_dimension() {
        assert_eq!(ElectricDipoleMomentUnit::base(), ElectricChargeUnit::base() * LengthUnit::base());
    }

    #[test]
    fn check_units() {
        test_unit(ElectricChargeUnit::coulomb, LengthUnit::meter, ElectricDipoleMomentUnit::coulomb_meter);
        // Manually run this because 1 D = 1e-10 statC*Angstrom 
        assert_eq!(ElectricCharge::statcoulomb(1e-10) * Length::angstrom(1.0), ElectricDipoleMoment::debye(1.0));
        test_unit( ElectricChargeUnit::elementary_charge, LengthUnit::centimeter, ElectricDipoleMomentUnit::elementary_charge_centimeter);
        test_unit(ElectricChargeUnit::elementary_charge, LengthUnit::centimeter, ElectricDipoleMomentUnit::atomic_unit_of_charge_centimeter);
        test_unit(ElectricChargeUnit::elementary_charge, LengthUnit::bohr_radius,
        ElectricDipoleMomentUnit::atomic_unit_of_electric_dipole_moment);
    }
    fn test_unit(charge: ElectricChargeUnit, length: LengthUnit, value: ElectricDipoleMomentUnit)
    {
         assert_eq!(Into::<UnitDefinition>::into(value), (Into::<UnitDefinition>::into(charge) * Into::<UnitDefinition>::into(length)));
    }
}
