//! Action (base UnitDefinition joule second, kg ⋅ m² ⋅ s⁻¹).
use crate::{prefix, quantity};
quantity! {
    /// Action (base UnitDefinition joule second, kg ⋅ m² ⋅ s⁻¹).
    quantity: Action; "action";
    /// Dimension of action, L²MT⁻¹ (base UnitDefinition joule second, kg ⋅ m² ⋅ s⁻¹).
    dimension: ISQ[
        2.0,     // length
        1.0,     // mass
        -1.0,     // time
        0.0,     // electric current
        0.0,     // thermodynamic temperature
        0.0,     // amount of substance
        0.0];    // luminous intensity
    units {
        @joule_second: prefix!(none); "J · s", "joule second", "joule seconds";

        /// Reduced Planck constant ħ.
        @atomic_unit_of_action: 1.054_571_817_E-34; "ħ", "atomic UnitDefinition of action",
            "atomic units of action";
        @reduced_planck_constant: 1.054_571_817_E-34; "ħ_reduced", "reduced planck constant",
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
    use crate::traits::Unit;
    use crate::{units::{ActionUnit, EnergyUnit, TimeUnit}, units_base::UnitDefinition};

    #[test]
    fn check_dimension() {
        assert_eq!(ActionUnit::base(), EnergyUnit::base()*TimeUnit::base());
    }

    #[test]
    fn check_units() {
        test_unit(EnergyUnit::joule, TimeUnit::second, ActionUnit::joule_second);
        test_unit(EnergyUnit::erg, TimeUnit::second, ActionUnit::erg_second);
        test_unit(EnergyUnit::electronvolt, TimeUnit::second, ActionUnit::electronvolt_second);

    }
    fn test_unit(energy: EnergyUnit, time: TimeUnit, definition: ActionUnit) {
        assert_eq!(Into::<UnitDefinition>::into(definition), Into::<UnitDefinition>::into(energy) * Into::<UnitDefinition>::into(time));
    }
}

