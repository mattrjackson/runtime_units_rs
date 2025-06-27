//! Volumetric power density (base UnitDefinition watt per cubic meter, m⁻¹ · kg · s⁻³).
use crate::{prefix, quantity};
quantity! {
    /// Volumetric power density (base UnitDefinition watt per cubic meter, m⁻¹ · kg · s⁻³).
    quantity: VolumetricPowerDensity; "volumetric power density";
    /// Dimension of volumetric power density, L⁻¹MT⁻³ (base UnitDefinition watt per cubic meter,
    /// m⁻¹ · kg · s⁻³).
    dimension: ISQ[
        -1.0,     // length
        1.0,     // mass
        -3.0,     // time
        0.0,     // electric current
        0.0,     // thermodynamic temperature
        0.0,     // amount of substance
        0.0];    // luminous intensity
    units {
        @watt_per_cubic_meter: prefix!(none); "W/m³", "watt per cubic meter",
            "watts per cubic meter";
        @watt_per_cubic_centimeter:
            prefix!(none) / prefix!(centi) / prefix!(centi) / prefix!(centi); "W/cm³",
            "watt per cubic centimeter", "watts per cubic centimeter";
        @watt_per_cubic_millimeter:
            prefix!(none) / prefix!(milli) / prefix!(milli) / prefix!(milli); "W/mm³",
            "watt per cubic millimeter", "watts per cubic millimeter";
    }
}

// #[cfg(test)]
// mod tests {
// use crate::traits::Unit;
//     storage_types! {
//         use crate::num::One;
//         use crate::si::power as p;
//         use crate::si::quantities::*;
//         use crate::si::volumetric_power_density as vpd;
//         use crate::si::volume as v;
//         use crate::tests::Test;

//         #[test]
//         fn check_dimension() {
//             let _: VolumetricPowerDensity<V> =  Power::new::<p::watt>(V::one())
//                 / Volume::new::<v::cubic_meter>(V::one());
//         }

//         #[test]
//         fn check_units() {
//             test::<p::watt, v::cubic_meter, vpd::watt_per_cubic_meter>();
//             test::<p::watt, v::cubic_centimeter, vpd::watt_per_cubic_centimeter>();
//             test::<p::watt, v::cubic_millimeter, vpd::watt_per_cubic_millimeter>();

//             fn test<P: p::Conversion<V>, U: v::Conversion<V>, VPD: vpd::Conversion<V>>() {
//                 Test::assert_approx_eq(&VolumetricPowerDensity::new::<VPD>(V::one()),
//                     &(Power::new::<P>(V::one()) / Volume::new::<U>(V::one())));
//             }
//         }
//     }
// }
