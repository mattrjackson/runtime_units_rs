//! Dose Equivalent (m^2/s^2).
use crate::{prefix, quantity};
quantity! {
    /// Dose Equivalent (Sievert) J/kg => m^2/s^2
    quantity: DoseEquivalent; "dose_equivalent";
    /// Dimension of dose equivalent.
    dimension: ISQ<
        P2,     // length
        Z0,     // mass
        N2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::DoseEquivalentKind);
    units {
        /// SI derived unit of dose equivalent. 
        @sievert: 1.0_E0; "Sv", "sievert", "sievert";
        @centisievert: prefix!(centi); "cSv", "centisievert", "centisievert";
        @rem: prefix!(centi); "rem", "rem", "rem";
    }
}


// #[cfg(test)]
// mod tests {
//     storage_types! {
//         use crate::lib::f64::consts::PI;
//         use crate::num::{FromPrimitive, One};
//         use crate::si::angle as a;
//         use crate::si::quantities::*;
//         use crate::tests::Test;

//         #[test]
//         fn check_units() {
//             Test::assert_eq(&Angle::new::<a::radian>(V::from_f64(2.0 * PI).unwrap()),
//                 &Angle::new::<a::revolution>(V::one()));
//             Test::assert_eq(&Angle::new::<a::degree>(V::from_f64(360.0).unwrap()),
//                 &Angle::new::<a::revolution>(V::one()));
//             Test::assert_approx_eq(&Angle::new::<a::gon>(V::from_f64(400.0).unwrap()),
//                 &Angle::new::<a::revolution>(V::one()));
//             Test::assert_eq(&Angle::new::<a::minute>(V::from_f64(60.0).unwrap()),
//                 &Angle::new::<a::degree>(V::one()));
//             Test::assert_eq(&Angle::new::<a::second>(V::from_f64(60.0 * 60.0).unwrap()),
//                 &Angle::new::<a::degree>(V::one()));
//         }
//     }

// }
