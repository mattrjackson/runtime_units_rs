//! Volumetric heat capacity (base UnitDefinition joule per cubic meter kelvin, m⁻¹ · kg · s⁻² · K⁻¹).
use crate::{prefix, quantity};
quantity! {
    /// Volumetric heat capacity (base UnitDefinition joule per cubic meter kelvin, m⁻¹ · kg · s⁻² · K⁻¹).
    quantity: VolumetricHeatCapacity; "volumetric heat capacity";
    /// Dimension of volumetric heat capacity, L⁻¹MT⁻²Th⁻¹(base UnitDefinition joule per cubic meter kelvin,
    /// m⁻¹ · kg · s⁻² · K⁻¹).
    dimension: ISQ[
        -1.0,     // length
        1.0,     // mass
        -2.0,     // time
        0.0,     // electric current
        -1.0,     // thermodynamic temperature
        0.0,     // amount of substance
        0.0];    // luminous intensity
    units {
        @joule_per_cubic_meter_kelvin: prefix!(none); "J/(m³ · K)", "joule per cubic meter kelvin",
            "joules per cubic meter kelvin";
        @calorie_per_cubic_meter_kelvin: 4.184_E0; "cal/(m³ · K)", "calorie per cubic meter kelvin",
            "calories per cubic meter kelvin";
    }
}

// #[cfg(test)]
// mod tests {
// use crate::traits::Unit;
//     storage_types! {
//         use crate::num::One;
//         use crate::si::heat_capacity as hc;
//         use crate::si::volume as vol;
//         use crate::si::quantities::*;
//         use crate::si::volumetric_heat_capacity as vhc;
//         use crate::tests::Test;

//         #[test]
//         fn check_dimension() {
//             let _: VolumetricHeatCapacity<V> = HeatCapacity::new::<hc::joule_per_kelvin>(V::one())
//                 / Volume::new::<vol::cubic_meter>(V::one());
//         }

//         #[test]
//         fn check_heat_capacity_volume_units() {
//             test::<hc::joule_per_kelvin, vol::cubic_meter, vhc::joule_per_cubic_meter_kelvin>();
//             test::<hc::calorie_per_kelvin, vol::cubic_meter, vhc::calorie_per_cubic_meter_kelvin>();

//             fn test<HC: hc::Conversion<V>, VOL: vol::Conversion<V>, VHC: vhc::Conversion<V>>()
//             {
//                 Test::assert_approx_eq(&VolumetricHeatCapacity::new::<VHC>(V::one()),
//                     &(HeatCapacity::new::<HC>(V::one()) / (Volume::new::<VOL>(V::one())))
//                 );
//             }
//         }
//     }
// }
