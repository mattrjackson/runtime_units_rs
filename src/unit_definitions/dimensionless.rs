//! Dimensionless (dimensionless quantity).
use crate::quantity;
quantity! {
    /// Dimensionless (dimensionless quantity).
    quantity: Dimensionless; "nondimensional";
    /// Dimension of information, 1 (dimensionless).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
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