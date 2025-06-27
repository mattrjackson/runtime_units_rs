//! Reciprocal length (base UnitDefinition reciprocal meter, m⁻¹).
use crate::{prefix, quantity};
quantity! {
    /// Reciprocal length (base UnitDefinition reciprocal meter, m⁻¹).
    quantity: ReciprocalLength; "reciprocal length";
    /// Dimension of reciprocal length, L⁻¹ (base UnitDefinition reciprocal meter, m⁻¹).
    dimension: ISQ[
        -1.0,     // length
        0.0,     // mass
        0.0,     // time
        0.0,     // electric current
        0.0,     // thermodynamic temperature
        0.0,     // amount of substance
        0.0];    // luminous intensity
    units {
        @reciprocal_kilometer: prefix!(none) / prefix!(kilo); "km⁻¹", "reciprocal kilometer",
            "reciprocal kilometers";
        @reciprocal_meter: prefix!(none); "m⁻¹", "reciprocal meter", "reciprocal meters";
        @reciprocal_decimeter: prefix!(none) / prefix!(deci); "dm⁻¹", "reciprocal decimeter",
            "reciprocal decimeters";
        @reciprocal_centimeter: prefix!(none) / prefix!(centi); "cm⁻¹", "reciprocal centimeter",
            "reciprocal centimeters";
        @reciprocal_millimeter: prefix!(none) / prefix!(milli); "mm⁻¹", "reciprocal millimeter",
            "reciprocal millimeters";
        @reciprocal_micrometer: prefix!(none) / prefix!(micro); "µm⁻¹", "reciprocal micrometer",
            "reciprocal micrometers";
        @reciprocal_nanometer: prefix!(none) / prefix!(nano); "nm⁻¹", "reciprocal nanometer",
            "reciprocal nanometers";

        @reciprocal_angstrom: prefix!(none) / 1.0_E-10; "Å⁻¹", "reciprocal ångström",
            "reciprocal ångströms";
        @diopter: prefix!(none); "dpt", "diopter", "diopters";
        @rydberg_constant: 10_973_731.568_160; "R∞", "Rydberg constant", "Rydberg constants";
    }
}

// #[cfg(test)]
// mod test {
//     storage_types! {
//         use crate::num::One;
//         use crate::si::length as l;
//         use crate::si::reciprocal_length as rl;
//         use crate::si::quantities::*;
//         use crate::tests::Test;

//         #[test]
//         fn check_dimension() {
//             let _: ReciprocalLength<V> = V::one()
//                 / Length::new::<l::meter>(V::one());
//         }

//         #[test]
//         fn check_units() {
//             test::<l::kilometer, rl::reciprocal_kilometer>();
//             test::<l::meter, rl::reciprocal_meter>();
//             test::<l::decimeter, rl::reciprocal_decimeter>();
//             test::<l::centimeter, rl::reciprocal_centimeter>();
//             test::<l::millimeter, rl::reciprocal_millimeter>();
//             test::<l::micrometer, rl::reciprocal_micrometer>();
//             test::<l::nanometer, rl::reciprocal_nanometer>();
//             test::<l::angstrom, rl::reciprocal_angstrom>();
//             test::<l::meter, rl::diopter>();

//             fn test<L: l::Conversion<V>, RL: rl::Conversion<V>>() {
//                 Test::assert_approx_eq(&ReciprocalLength::new::<RL>(V::one()),
//                     &(V::one() / Length::new::<L>(V::one())));
//             }
//         }
//     }
// }
