//! Molality (base UnitDefinition mole per kilogram, kg⁻¹ · mol).
use crate::{prefix, quantity};
quantity! {
    /// Molality (base UnitDefinition mole per kilogram, kg⁻¹ · mol).
    quantity: Molality; "molality";
    /// Dimension of molality, M⁻¹N (base UnitDefinition mole per kilogram, kg⁻¹ · mol).
    dimension: ISQ[
        0.0,     // length
        -1.0,     // mass
        0.0,     // time
        0.0,     // electric current
        0.0,     // thermodynamic temperature
        1.0,     // amount of substance
        0.0];    // luminous intensity
    kind: dyn (crate::si::marker::ConstituentConcentrationKind);
    units {
        @mole_per_kilogram: prefix!(none); "mol/kg", "mole per kilogram", "moles per kilogram";
    }
}

// #[cfg(test)]
// mod tests {
// use crate::traits::Unit;
//     storage_types! {
//         use crate::num::One;
//         use crate::si::amount_of_substance as aos;
//         use crate::si::mass as m;
//         use crate::si::molality as mol;
//         use crate::si::quantities::*;
//         use crate::tests::Test;

//         #[test]
//         fn check_dimension() {
//             let _: Molality<V> = (AmountOfSubstance::new::<aos::mole>(V::one())
//                 / Mass::new::<m::kilogram>(V::one())).into();
//         }

//         #[test]
//         fn check_units() {
//             test::<aos::mole, m::kilogram, mol::mole_per_kilogram>();

//             fn test<AOS: aos::Conversion<V>, M: m::Conversion<V>, MOL: mol::Conversion<V>>() {
//                 Test::assert_approx_eq(&Molality::new::<MOL>(V::one()),
//                     &(AmountOfSubstance::new::<AOS>(V::one()) /
//                     Mass::new::<M>(V::one())).into());
//             }
//         }
//     }
// }
