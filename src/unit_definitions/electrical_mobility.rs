//! Electrical mobility (base UnitDefinition square meter per volt second, kg⁻¹ · s² · A).
use crate::{prefix, quantity};
quantity! {
    /// Electrical mobility (base UnitDefinition square meter per volt second, kg⁻¹ · s² · A).
    quantity: ElectricalMobility; "electrical mobility";
    /// Dimension of electrical mobility, M⁻¹T²I⁻¹ (base UnitDefinition square meter per volt second,
    /// kg⁻¹ · s² · A).
    dimension: ISQ[
        0.0,     // length
        -1.0,     // mass
        2.0,     // time
        1.0,     // electric current
        0.0,     // thermodynamic temperature
        0.0,     // amount of substance
        0.0];    // luminous intensity
    units {
        @square_meter_per_volt_second: prefix!(none); "m²/(V · s)", "square meter per volt second",
            "square meters per volt second";
        @square_centimeter_per_volt_second: prefix!(centi) * prefix!(centi); "cm²/(V · s)",
            "square centimeter per volt second", "square centimeters per volt second";
    }
}

// #[cfg(test)]
// mod test {
//     storage_types! {
//         use crate::num::One;
//         use crate::si::electrical_mobility as em;
//         use crate::si::electric_potential as ep;
//         use crate::si::time as t;
//         use crate::si::length as l;
//         use crate::si::quantities::*;
//         use crate::tests::Test;

//         #[test]
//         fn check_dimension() {
//             let _: ElectricalMobility<V> = Length::new::<l::meter>(V::one())
//                 * Length::new::<l::meter>(V::one())
//                 / Time::new::<t::second>(V::one())
//                 / ElectricPotential::new::<ep::volt>(V::one());
//         }

//         #[test]
//         fn check_units() {
//             test::<l::meter, ep::volt, t::second, em::square_meter_per_volt_second>();
//             test::<l::centimeter, ep::volt, t::second,em::square_centimeter_per_volt_second>();

//             fn test<L: l::Conversion<V>, EP: ep::Conversion<V>, T: t::Conversion<V>,
//                 EM: em::Conversion<V>>()
//             {
//                 Test::assert_approx_eq(&ElectricalMobility::new::<EM>(V::one()),
//                     &(Length::new::<L>(V::one())
//                         * Length::new::<L>(V::one())
//                         / ElectricPotential::new::<EP>(V::one())
//                         / Time::new::<T>(V::one())));
//             }
//         }
//     }
// }
