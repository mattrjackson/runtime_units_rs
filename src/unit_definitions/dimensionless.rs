//! Dimensionless (dimensionless quantity).
use crate::quantity;
quantity! {
    /// Dimensionless (dimensionless quantity).
    quantity: Dimensionless; "nondimensional";
    /// Dimension of information, 1 (dimensionless).
    dimension: ISQ[
        0.0,     // length
        0.0,     // mass
        0.0,     // time
        0.0,     // electric current
        0.0,     // thermodynamic temperature
        0.0,     // amount of substance
        0.0];    // luminous intensity
    units {        
        @scalar: 1.0; "scalar", "scalar", "scalars";
    }
}


#[cfg(test)]
mod tests {
    use crate::traits::Unit;
    use crate::units::DimensionlessUnit;
    use crate::units_base::UnitDefinition;
    use crate::{Dimensionless, Length};
    
    #[test]
    fn check_dimension() {
        assert_eq!(DimensionlessUnit::base(),  UnitDefinition::new(1.0, 0, 0, 0, 0, 0, 0, 0).base);
    }

   #[test]
    fn check_units() {
       assert_eq!(DimensionlessUnit::scalar.definition(), UnitDefinition::new(1.0, 0, 0, 0, 0, 0, 0, 0));
       assert_eq!(Length::meter(1.0) * Dimensionless::scalar(2.0), Length::meter(2.0));
    }
}