//! Areal density of states (base unit 1 / square meter joule, kg⁻¹ · m⁻⁴ · s²).
use crate::{prefix, quantity};
quantity! {
    /// Areal density of states (base unit 1 / square meter joule, kg⁻¹ · m⁻⁴ · s²).
    quantity: ArealDensityOfStates; "areal density of states";
    /// Dimension of areal density of states, L⁻⁴M⁻¹T² (base unit 1 / square meter joule, kg⁻¹ · m⁻⁴ · s²).
    dimension: ISQ<
        N4,     // length
        N1,     // mass
        P2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
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
    use crate::{units::{MassUnit, ArealDensityOfStatesUnit, EnergyUnit, TimeUnit, LengthUnit}, units_base::Unit};
    
    #[test]
    fn check_dimension() {
        assert_eq!(ArealDensityOfStatesUnit::unit_base(),  MassUnit::unit_base().powi(-1) * TimeUnit::unit_base().powi(2) / LengthUnit::unit_base().powi(4));
    }

   #[test]
    fn check_units() {
        test_unit(ArealDensityOfStatesUnit::state_per_square_meter_joule, LengthUnit::meter, EnergyUnit::joule);
        test_unit(ArealDensityOfStatesUnit::state_per_square_centimeter_joule, LengthUnit::centimeter, EnergyUnit::joule);
        test_unit(ArealDensityOfStatesUnit::state_per_square_centimeter_electronvolt, LengthUnit::centimeter, EnergyUnit::electronvolt);

    }
    fn test_unit(value: ArealDensityOfStatesUnit, length: LengthUnit, energy: EnergyUnit) {
        assert_eq!(Into::<Unit>::into(value), Into::<Unit>::into(length).powi(-2) / Into::<Unit>::into(energy));
    }
}