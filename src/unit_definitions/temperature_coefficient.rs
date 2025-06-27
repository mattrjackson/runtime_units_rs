//! Temperature coefficient (base UnitDefinition 1 / kelvin, K⁻¹).
use crate::{prefix, quantity};
quantity! {
    /// Temperature coefficient (base UnitDefinition 1 / kelvin, K⁻¹).
    quantity: TemperatureCoefficient; "temperature coefficient";
    /// Dimension of temperature coefficient, Th⁻¹ (base UnitDefinition 1 / kelvin, K⁻¹).
    dimension: ISQ[
        0.0,     // length
        0.0,     // mass
        0.0,     // time
        0.0,     // electric current
        -1.0,     // thermodynamic temperature
        0.0,     // amount of substance
        0.0];    // luminous intensity
    units {
        @per_kelvin: prefix!(none); "K⁻¹", "per kelvin", "per kelvin";
        @ppm_per_kelvin: 1E-6; "ppm/K", "ppm per kelvin", "ppm per kelvin";
        @ppm_per_degree_celsius: 1E-6; "ppm/°C", "ppm per degree Celsius", "ppm per degree Celsius";
    }
}

// #[cfg(test)]
// mod test {
//     storage_types! {
//         use crate::num::One;
//         use crate::si::temperature_interval as ti;
//         use crate::si::temperature_coefficient as tc;
//         use crate::si::quantities::*;
//         use crate::tests::Test;

//         #[test]
//         fn check_dimension() {
//             let _: TemperatureCoefficient<V> = V::one()
//                 / TemperatureInterval::new::<ti::kelvin>(V::one());
//         }

//         #[test]
//         fn check_units() {
//             test::<ti::kelvin, tc::per_kelvin>();
//             test::<ti::megakelvin, tc::ppm_per_kelvin>();
//             test::<ti::megakelvin, tc::ppm_per_degree_celsius>();

//             fn test<TI: ti::Conversion<V>, TC: tc::Conversion<V>>() {
//                 Test::assert_approx_eq(&TemperatureCoefficient::new::<TC>(V::one()),
//                     &(V::one()
//                         / TemperatureInterval::new::<TI>(V::one())));
//             }
//         }
//     }
// }
