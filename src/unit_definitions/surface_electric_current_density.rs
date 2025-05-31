//! Surface electric current density (base UnitDefinition ampere per meter, ). 
use crate::{prefix, quantity};
quantity! {
    /// Surface electric current density (base UnitDefinition ampere per meter, m⁻¹ · A).
    quantity: SurfaceElectricCurrentDensity; "surface electric current density";
    /// Dimension of surface electric current density, IL⁻¹ (base UnitDefinition ampere per meter, m⁻¹ · A).
    dimension: ISQ[
        -1.0,     // length
        0.0,     // mass
        0.0,     // time
        1.0,     // electric current
        0.0,     // thermodynamic temperature
        0.0,     // amount of substance
        0.0];    // luminous intensity
    kind: dyn (crate::si::marker::ConstituentConcentrationKind);
    units {
        @ampere_per_meter: prefix!(none); "A/m", "ampere per meter", "amperes per meter";
        @ampere_per_centimeter: prefix!(none) / prefix!(centi) ; "A/cm", "ampere per centimeter",
            "amperes per centimeter";
        @ampere_per_millimeter: prefix!(none) / prefix!(milli) ; "A/mm", "ampere per millimeter",
            "amperes per millimeter";
        @ampere_per_micrometer: prefix!(none) / prefix!(micro) ; "A/μm", "ampere per micrometer",
            "amperes per micrometer";
    }
}

// #[cfg(test)]
// mod tests {
// use crate::traits::Unit;
//     storage_types! {
//         use crate::num::One;
//         use crate::si::length as l;
//         use crate::si::electric_current as i;
//         use crate::si::surface_electric_current_density as ecd;
//         use crate::si::quantities::*;
//         use crate::tests::Test;

//         #[test]
//         fn check_dimension() {
//             let _: SurfaceElectricCurrentDensity<V> = (ElectricCurrent::new::<i::ampere>(V::one())
//                 / Length::new::<l::meter>(V::one())).into();
//         }

//         #[test]
//         fn check_units() {
//             test::<i::ampere, l::meter, ecd::ampere_per_meter>();
//             test::<i::ampere, l::centimeter, ecd::ampere_per_centimeter>();
//             test::<i::ampere, l::millimeter, ecd::ampere_per_millimeter>();
//             test::<i::ampere, l::micrometer, ecd::ampere_per_micrometer>();

//             fn test<I: i::Conversion<V>, L: l::Conversion<V>, ECD: ecd::Conversion<V>>() {
//                 Test::assert_approx_eq(&SurfaceElectricCurrentDensity::new::<ECD>(V::one()),
//                     &(ElectricCurrent::new::<I>(V::one()) / Length::new::<L>(V::one())).into());
//             }
//         }
//     }
// }
