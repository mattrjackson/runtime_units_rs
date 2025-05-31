//! Molar volume (base UnitDefinition cubic meter per mole, m³ · mol⁻¹).
use crate::{prefix, quantity};
quantity! {
    /// Molar volume (base UnitDefinition cubic meter per mole, m³ · mol⁻¹).
    quantity: MolarVolume; "molar volume";
    /// Dimension of molar volume, L³N⁻¹ (base UnitDefinition cubic meter per mole, m³ · mol⁻¹).
    dimension: ISQ[
        3.0,     // length
        0.0,     // mass
        0.0,     // time
        0.0,     // electric current
        0.0,     // thermodynamic temperature
        -1.0,     // amount of substance
        0.0];    // luminous intensity
    units {
        @cubic_meter_per_mole: prefix!(none); "m³/mol", "cubic meter per mole",
            "cubic meters per mole";
        @cubic_decimeter_per_mole: prefix!(deci) * prefix!(deci) * prefix!(deci); "dm³/mol",
            "cubic decimeter per mole", "cubic decimeters per mole";
        @cubic_centimeter_per_mole: prefix!(centi) * prefix!(centi) * prefix!(centi); "cm³/mol",
            "cubic centimeter per mole", "cubic centimeters per mole";

        @cubic_meter_per_particle: 6.022_140_76_E23; "m³/particle", "cubic meter per particle",
            "cubic meters per particle";
        @cubic_micrometer_per_particle:
            prefix!(micro) * prefix!(micro) * prefix!(micro) * 6.022_140_76_E23; "µm³/particle",
            "cubic micrometer per particle", "cubic micrometers per particle";
        @cubic_nanometer_per_particle:
            prefix!(nano) * prefix!(nano) * prefix!(nano) * 6.022_140_76_E23; "nm³/particle",
            "cubic nanometer per particle", "cubic nanometers per particle";
    }
}

// #[cfg(test)]
// mod test {
//     storage_types! {
//         use crate::num::One;
//         use crate::si::amount_of_substance as aos;
//         use crate::si::molar_volume as mv;
//         use crate::si::quantities::*;
//         use crate::si::volume as v;
//         use crate::tests::Test;

//         #[test]
//         fn check_dimension() {
//             let _: MolarVolume<V> = Volume::new::<v::cubic_meter>(V::one())
//                 / AmountOfSubstance::new::<aos::mole>(V::one());
//         }

//         #[test]
//         fn check_units() {
//             test::<v::cubic_meter, aos::mole, mv::cubic_meter_per_mole>();
//             test::<v::cubic_decimeter, aos::mole, mv::cubic_decimeter_per_mole>();
//             test::<v::cubic_centimeter, aos::mole, mv::cubic_centimeter_per_mole>();
//             test::<v::cubic_meter, aos::particle, mv::cubic_meter_per_particle>();
//             test::<v::cubic_micrometer, aos::particle, mv::cubic_micrometer_per_particle>();
//             test::<v::cubic_nanometer, aos::particle, mv::cubic_nanometer_per_particle>();

//             fn test<U: v::Conversion<V>, AOS: aos::Conversion<V>, MV: mv::Conversion<V>>() {
//                 Test::assert_approx_eq(&MolarVolume::new::<MV>(V::one()),
//                     &(Volume::new::<U>(V::one()) / AmountOfSubstance::new::<AOS>(V::one())));
//             }
//         }
//     }
// }
