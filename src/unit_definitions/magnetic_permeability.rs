//! Magnetic permeability (base UnitDefinition henry per meter, m · kg · s⁻² · A⁻²).
use crate::{prefix, quantity};
quantity! {
    /// Magnetic permeability (base UnitDefinition henry per meter, m · kg · s⁻² · A⁻²).
    quantity: MagneticPermeability; "magnetic permeability";
    /// Dimension of magnetic permeability, LMT⁻²I⁻² (base UnitDefinition henry per meter,
    /// m · kg · s⁻² · A⁻²).
    dimension: ISQ[
        1.0,     // length
        1.0,     // mass
        -2.0,     // time
        -2.0,     // electric current
        0.0,     // thermodynamic temperature
        0.0,     // amount of substance
        0.0];    // luminous intensity
    units {
        @henry_per_meter: prefix!(none); "H/m", "henry per meter", "henrys per meter";
        @vacuum_magnetic_permeability: 1.256_637_062_12_E-6; "µ₀", "vacuum magnetic permeability",
            "vacuum magnetic permeability";
    }
}

// #[cfg(test)]
// mod test {
//     storage_types! {
//         use crate::num::One;
//         use crate::si::inductance as ind;
//         use crate::si::magnetic_permeability as mp;
//         use crate::si::length as l;
//         use crate::si::quantities::*;
//         use crate::tests::Test;

//         #[test]
//         fn check_dimension() {
//             let _: MagneticPermeability<V> = Inductance::new::<ind::henry>(V::one())
//                 / Length::new::<l::meter>(V::one());
//         }

//         #[test]
//         fn check_units() {
//             test::<mp::henry_per_meter, ind::henry, l::meter>();

//             fn test<MP: mp::Conversion<V>, IND: ind::Conversion<V>, L: l::Conversion<V>>() {
//                 Test::assert_approx_eq(&MagneticPermeability::new::<MP>(V::one()),
//                     &(Inductance::new::<IND>(V::one())
//                         / Length::new::<L>(V::one())));
//             }
//         }
//     }
// }
