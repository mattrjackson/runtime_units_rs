//! Action (base unit joule second, kg ⋅ m² ⋅ s⁻¹).
use crate::{prefix, quantity};
quantity! {
    /// Action (base unit joule second, kg ⋅ m² ⋅ s⁻¹).
    quantity: Action; "action";
    /// Dimension of action, L²MT⁻¹ (base unit joule second, kg ⋅ m² ⋅ s⁻¹).
    dimension: ISQ<
        P2,     // length
        P1,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @joule_second: prefix!(none); "J · s", "joule second", "joule seconds";

        /// Reduced Planck constant ħ.
        @atomic_unit_of_action: 1.054_571_817_E-34; "ħ", "atomic unit of action",
            "atomic units of action";
        @reduced_planck_constant: 1.054_571_817_E-34; "ħ", "reduced planck constant",
            "reduced planck constants";
        @planck_constant: 6.626_070_15_E-34; "h", "planck constant", "planck constants";
        @erg_second: 1.0_E-7; "erg · s", "erg second", "erg seconds";
        @electronvolt_second: 1.602_176_634_E-19; "eV · s", "electronvolt second",
            "electronvolt seconds";
    }
}

#[cfg(test)]
#[cfg(feature="Action")]
mod test {
   
    use crate::{quantities, units::{ActionUnit, EnergyUnit, TimeUnit}, units_base::{Unit, UnitBase}};

    #[test]
    fn check_dimension() {
        assert_eq!(ActionUnit::unit(), EnergyUnit::unit()*TimeUnit::unit());
    }

    #[test]
    fn check_units() {
        test_unit(EnergyUnit::joule, TimeUnit::second, ActionUnit::joule_second);
        test_unit(EnergyUnit::erg, TimeUnit::second, ActionUnit::erg_second);
        test_unit(EnergyUnit::electronvolt, TimeUnit::second, ActionUnit::electronvolt_second);

    }
    fn test_unit(energy: EnergyUnit, time: TimeUnit, unit: ActionUnit) {
        assert_eq!(Into::<Unit>::into(unit), Into::<Unit>::into(energy) * Into::<Unit>::into(time));
    }
}

