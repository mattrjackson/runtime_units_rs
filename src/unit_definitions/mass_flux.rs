//! Mass flux (base UnitDefinition kilogram per square meter second, m⁻² · kg ·  s⁻¹).
use crate::{prefix, quantity};
quantity! {
    ///  Mass flux (base UnitDefinition kilogram per square meter second, m⁻² · kg ·  s⁻¹).
    quantity: MassFlux; "mass flux";
    /// Dimension of mass flux, L⁻²MT⁻¹ (base UnitDefinition kilogram per square meter second,
    /// m⁻² · kg ·  s⁻¹).
    dimension: ISQ[
        -2.0,     // length
        1.0,     // mass
        -1.0,     // time
        0.0,     // electric current
        0.0,     // thermodynamic temperature
        0.0,     // amount of substance
        0.0];    // luminous intensity
    units {
        @kilogram_per_square_meter_second: prefix!(none); "kg/(m² · s)",
            "kilogram per square meter second", "kilograms per square meter second";
        @gram_per_square_centimeter_second: prefix!(milli) / prefix!(centi) / prefix!(centi);
            "g/(cm² · s)", "gram per square centimeter second",
            "grams per square centimeter second";
    }
}

// #[cfg(test)]
// mod test {
//     storage_types! {
//         use crate::num::One;
//         use crate::si::mass as m;
//         use crate::si::mass_flux as mf;
//         use crate::si::quantities::*;
//         use crate::si::time as t;
//         use crate::si::area as a;
//         use crate::tests::Test;

//         #[test]
//         fn check_dimension() {
//             let _: MassFlux<V> = Mass::new::<m::kilogram>(V::one())
//                 / Time::new::<t::second>(V::one())
//                 / Area::new::<a::square_meter>(V::one());
//         }

//         #[test]
//         fn check_units() {
//             test::<m::kilogram, a::square_meter, t::second, mf::kilogram_per_square_meter_second>();
//             test::<m::gram, a::square_centimeter, t::second,
//                 mf::gram_per_square_centimeter_second>();

//             fn test<M: m::Conversion<V>,
//                 A: a::Conversion<V>,
//                 T: t::Conversion<V>,
//                 MF: mf::Conversion<V>>()
//             {
//                 Test::assert_approx_eq(&MassFlux::new::<MF>(V::one()),
//                     &(Mass::new::<M>(V::one())
//                         / Time::new::<T>(V::one())
//                         / Area::new::<A>(V::one())));
//             }
//         }
//     }
// }
