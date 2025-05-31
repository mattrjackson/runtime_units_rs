//! Areal density of states (base UnitDefinition 1 / square meter joule, kg⁻¹ · m⁻⁴ · s²).
use crate::{prefix, quantity};
quantity! {
    /// Areal density of states (base UnitDefinition 1 / square meter joule, kg⁻¹ · m⁻⁴ · s²).
    quantity: ArealDensityOfStates; "areal density of states";
    /// Dimension of areal density of states, L⁻⁴M⁻¹T² (base UnitDefinition 1 / square meter joule, kg⁻¹ · m⁻⁴ · s²).
    dimension: ISQ[
        -4.0,     // length
        -1.0,     // mass
        2.0,     // time
        0.0,     // electric current
        0.0,     // thermodynamic temperature
        0.0,     // amount of substance
        0.0];    // luminous intensity
    kind: dyn (crate::si::marker::ConstituentConcentrationKind);
    units {
        @state_per_square_meter_joule: prefix!(none); "1/(m² · J)", "state per square meter joule",
            "states per square meter joule";
        @state_per_square_centimeter_joule: prefix!(none) / prefix!(centi) / prefix!(centi);
            "1/(cm² · J)", "state per square centimeter joule",
            "states per square centimeter joule";
        @state_per_square_centimeter_electronvolt:
            prefix!(none) / prefix!(centi) / prefix!(centi) / 1.602_176_634_E-19; "1/(cm² · eV)",
            "state per square centimeter electronvolt", "states per square centimeter electronvolt";
    }
}

#[cfg(test)]
mod tests {
use crate::traits::Unit;
    use crate::{units::{MassUnit, ArealDensityOfStatesUnit, EnergyUnit, TimeUnit, LengthUnit}, units_base::UnitDefinition};
    
    #[test]
    fn check_dimension() {
        assert_eq!(ArealDensityOfStatesUnit::base(),  MassUnit::base().powi(-1) * TimeUnit::base().powi(2) / LengthUnit::base().powi(4));
    }

   #[test]
    fn check_units() {
        test_unit(ArealDensityOfStatesUnit::state_per_square_meter_joule, LengthUnit::meter, EnergyUnit::joule);
        test_unit(ArealDensityOfStatesUnit::state_per_square_centimeter_joule, LengthUnit::centimeter, EnergyUnit::joule);
        test_unit(ArealDensityOfStatesUnit::state_per_square_centimeter_electronvolt, LengthUnit::centimeter, EnergyUnit::electronvolt);

    }
    fn test_unit(value: ArealDensityOfStatesUnit, length: LengthUnit, energy: EnergyUnit) {
        assert_eq!(Into::<UnitDefinition>::into(value), Into::<UnitDefinition>::into(length).powi(-2) / Into::<UnitDefinition>::into(energy));
    }
}