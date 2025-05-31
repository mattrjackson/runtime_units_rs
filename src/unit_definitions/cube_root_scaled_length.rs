//! Length (base UnitDefinition meter, m).

use crate::{prefix, quantity};

quantity! {
    /// Cube root scaled Length (base UnitDefinition meter, m/energy^1/3).
    quantity: CubeRootScaledLength; "cube_root_scaled_length";
    /// Dimension of length, L (base UnitDefinition meter, m).
    dimension: ISQ[
        0.333333333,     // length
        -0.333333333,     // mass
        0.6666666667,     // time
        0.0,     // electric current
        0.0,     // thermodynamic temperature
        0.0,     // amount of substance
        0.0];    // luminous intensity
    units {                        
        @scaled_meter_per_newton: prefix!(none); "m/N^1/3", "meter_per_cube_root_newton", "meter_per_cube_root_newton";    
        @scaled_meter_per_ton_tnt: 6.205871442264474E-04; "m/ton^1/3", "meter_per_cube_root_kiloton_tnt", "meter_per_cube_root_kiloton_tnt";                
        @scaled_kilometer_per_ton_tnt: 6.205871442264474E-01; "km/ton^1/3", "scaled_kilometer", "scaled_kilometers";        
        @scaled_foot_per_ton_tnt: 1.8915496156022117E-04; "ft/ton^1/3", "scaled_foot", "scaled_feet";        
        @scaled_meter_per_kiloton_tnt: 6.205871442264475E-05; "m/kt^1/3", "meter_per_cube_root_kiloton_tnt", "meter_per_cube_root_kiloton_tnt";                
        @scaled_kilometer_per_kiloton_tnt: 6.2058714422644747E-02; "km/kt^1/3", "scaled_kilometer", "scaled_kilometers";        
        @scaled_foot_per_kiloton_tnt: 1.891549615602212E-05; "ft/kt^1/3", "scaled_foot", "scaled_feet";        
    }
}

#[cfg(test)]
mod tests {
    use crate::units_base::UnitDefinition;
    use crate::{units::{CubeRootScaledLengthUnit, LengthUnit, EnergyUnit}};


    #[test]
    fn check_dimension() {
        assert_eq!(CubeRootScaledLengthUnit::base(), LengthUnit::base() / EnergyUnit::base().powf(1.0/3.0));
    }

    #[test]
    fn check_units() {
        test_unit(LengthUnit::meter, EnergyUnit::ton_tnt, CubeRootScaledLengthUnit::scaled_meter_per_ton_tnt);
        test_unit(LengthUnit::foot, EnergyUnit::ton_tnt, CubeRootScaledLengthUnit::scaled_foot_per_ton_tnt);
        test_unit(LengthUnit::kilometer, EnergyUnit::ton_tnt, CubeRootScaledLengthUnit::scaled_kilometer_per_ton_tnt);
        test_unit(LengthUnit::meter, EnergyUnit::kiloton_tnt, CubeRootScaledLengthUnit::scaled_meter_per_kiloton_tnt);
        test_unit(LengthUnit::foot, EnergyUnit::kiloton_tnt, CubeRootScaledLengthUnit::scaled_foot_per_kiloton_tnt);
        test_unit(LengthUnit::kilometer, EnergyUnit::kiloton_tnt, CubeRootScaledLengthUnit::scaled_kilometer_per_kiloton_tnt);
    }
   
    fn test_unit(length: LengthUnit, energy: EnergyUnit, value: CubeRootScaledLengthUnit) {
        assert_eq!(Into::<UnitDefinition>::into(value), Into::<UnitDefinition>::into(length) / Into::<UnitDefinition>::into(energy).powf(1.0/3.0));
    }
}