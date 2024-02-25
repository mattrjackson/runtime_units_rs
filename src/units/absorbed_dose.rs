//! Absorbed Dose (m^2/s^2).

use crate::{prefix, quantity};
quantity! {
    /// Absorbed Dose (Gray) J/kg => m^2/s^2
    quantity: AbsorbedDose; "absorbed_dose";
    /// Dimension of absorbed dose.
    dimension: ISQ<
        P2,     // length
        Z0,     // mass
        N2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::AbsorbedDoseKind);
    units {
        /// SI derived unit of absorbed dose.
        @gray: 1.0_E0; "Gy", "gray", "gray";
        @centigray: prefix!(centi); "cGy", "centigray", "centigray";
        @rad: prefix!(centi); "rad", "rad", "rad";
    }
}

// #[cfg(test)]
// mod tests {
//     storage_types! {
//         use crate::lib::f64::consts::PI;
//         use crate::num::{FromPrimitive, One};
//         use crate::si::absorbed_dose as a;
//         use crate::si::quantities::*;
//         use crate::tests::Test;
//         use crate::si::f64::AbsorbedDose;
//         #[test]
//         fn check_units() {
//             Test::assert_eq(&AbsorbedDose::new::<a::centigray>(200.0),
//             &"100 cGy".parse::<AbsorbedDose>().unwrap());
//         }
//     }
// }
