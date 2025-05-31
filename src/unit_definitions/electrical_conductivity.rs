//! Electrical conductivity (base UnitDefinition siemens per meter, m⁻³ · kg⁻¹ · s³ · A²).
use crate::{prefix, quantity};
quantity! {
    /// Electrical conductivity (base UnitDefinition siemens per meter, m⁻³ · kg⁻¹ · s³ · A²).
    quantity: ElectricalConductivity; "electrical conductivity";
    /// Dimension of electrical conductivity, L⁻³M⁻¹T³I² (base UnitDefinition siemens per meter,
    /// m⁻³ · kg⁻¹ · s³ · A²).
    dimension: ISQ[
        -3.0,     // length
        -1.0,     // mass
        3.0,     // time
        2.0,     // electric current
        0.0,     // thermodynamic temperature
        0.0,     // amount of substance
        0.0];    // luminous intensity
    units {
        @siemens_per_meter: prefix!(none); "S/m", "siemens per meter", "siemens per meter";
        @siemens_per_centimeter: prefix!(none) / prefix!(centi); "S/cm", "siemens per centimeter",
            "siemens per centimeter";
    }
}

// #[cfg(test)]
// mod tests {
// use crate::traits::Unit;
//     storage_types! {
//         use crate::num::One;
//         use crate::si::electrical_conductance as g;
//         use crate::si::length as l;
//         use crate::si::electrical_conductivity as ec;
//         use crate::si::quantities::*;
//         use crate::tests::Test;

//         #[test]
//         fn check_dimension() {
//             let _: ElectricalConductivity<V> = ElectricalConductance::new::<g::siemens>(V::one())
//                 / Length::new::<l::meter>(V::one());
//         }

//         #[test]
//         fn check_units() {
//             test::<ec::siemens_per_meter, g::siemens, l::meter>();
//             test::<ec::siemens_per_centimeter, g::siemens, l::centimeter>();

//             fn test<EC: ec::Conversion<V>, G: g::Conversion<V>, L: l::Conversion<V>>() {
//                 Test::assert_approx_eq(&ElectricalConductivity::new::<EC>(V::one()),
//                     &(ElectricalConductance::new::<G>(V::one()) / Length::new::<L>(V::one())));
//             }
//         }
//     }
// }
