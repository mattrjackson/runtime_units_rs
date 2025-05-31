//! Magnetic field strength (H field) (base UnitDefinition ampere per meter, m⁻¹ · A).
use crate::{prefix, quantity};
quantity! {
    /// Magnetic field strength (H field) (base UnitDefinition ampere per meter, m⁻¹ · A).
    quantity: MagneticFieldStrength; "magnetic field strength (H field)";
    /// Dimension of magnetic field strength (H field), L⁻¹I (base UnitDefinition ampere per meter, m⁻¹ · A).
    dimension: ISQ[
        -1.0,     // length
        0.0,     // mass
        0.0,     // time
        1.0,     // electric current
        0.0,     // thermodynamic temperature
        0.0,     // amount of substance
        0.0];    // luminous intensity
    units {
        @ampere_per_meter: prefix!(none); "A/m", "ampere per meter", "amperes per meter";
        @ampere_per_centimeter: prefix!(none) / prefix!(centi); "A/cm", "ampere per centimeter",
            "amperes per centimeter";
        @ampere_per_millimeter: prefix!(none) / prefix!(milli); "A/mm", "ampere per millimeter",
            "amperes per millimeter";
        @ampere_per_micrometer: prefix!(none) / prefix!(micro); "A/μm", "ampere per micrometer",
            "amperes per micrometer";

        /// 1 oersted = 1000/(4π) A/m
        @oersted: 79.577_471_545_947_67; "Oe", "oersted", "oersteds";
    }
}

// #[cfg(test)]
// mod tests {
// use crate::traits::Unit;
//     storage_types! {
//         use crate::num::One;
//         use crate::si::length as l;
//         use crate::si::electric_current as i;
//         use crate::si::magnetic_field_strength as h;
//         use crate::si::quantities::*;
//         use crate::tests::Test;

//         #[test]
//         fn check_dimension() {
//             let _: MagneticFieldStrength<V> = ElectricCurrent::new::<i::ampere>(V::one())
//                 / Length::new::<l::meter>(V::one());
//         }

//         #[test]
//         fn check_units() {
//             test::<i::ampere, l::meter, h::ampere_per_meter>();
//             test::<i::ampere, l::centimeter, h::ampere_per_centimeter>();
//             test::<i::ampere, l::millimeter, h::ampere_per_millimeter>();
//             test::<i::ampere, l::micrometer, h::ampere_per_micrometer>();

//             fn test<I: i::Conversion<V>, L: l::Conversion<V>, H: h::Conversion<V>>() {
//                 Test::assert_approx_eq(&MagneticFieldStrength::new::<H>(V::one()),
//                     &(ElectricCurrent::new::<I>(V::one()) / Length::new::<L>(V::one())));
//             }
//         }
//     }
// }
