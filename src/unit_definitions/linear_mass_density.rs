//! Linear mass density (base UnitDefinition kilogram per meter, m⁻¹ · kg).
use crate::{prefix, quantity};
quantity! {
    /// Linear mass density (base UnitDefinition kilogram per meter, m⁻¹ · kg).
    quantity: LinearMassDensity; "linear mass density";
    /// Dimension of linear mass density, L⁻¹M (base UnitDefinition kilogram per meter, m⁻¹ · kg).
    dimension: ISQ[
        -1.0,     // length
        1.0,     // mass
        0.0,     // time
        0.0,     // electric current
        0.0,     // thermodynamic temperature
        0.0,     // amount of substance
        0.0];    // luminous intensity
    units {
        @kilogram_per_meter: prefix!(none); "kg/m", "kilogram per meter", "kilograms per meter";
        @gram_per_kilometer: prefix!(milli) / prefix!(kilo); "g/km", "gram per kilometer",
            "grams per kilometer";
        @gram_per_centimeter: prefix!(milli) / prefix!(centi); "g/cm", "gram per centimeter",
            "grams per centimeter";

        @ounce_per_foot: 2.834_952_E-2 / 3.048_E-1; "oz/ft", "ounce per foot", "ounces per foot";
        @ounce_per_inch: 2.834_952_E-2 / 2.54_E-2; "oz/in", "ounce per inch", "ounces per inch";
        @pound_per_yard: 4.535_924_E-1 / 9.144_E-1; "lb/yd", "pound per yard", "pounds per yard";
        @pound_per_foot: 4.535_924_E-1 / 3.048_E-1; "lb/ft", "pound per foot", "pounds per foot";
        @pound_per_inch: 4.535_924_E-1 / 2.54_E-2; "lb/in", "pound per inch", "pounds per inch";
    }
}

// #[cfg(test)]
// mod test {
//     storage_types! {
//         use crate::num::One;
//         use crate::si::mass as m;
//         use crate::si::linear_mass_density as d;
//         use crate::si::quantities::*;
//         use crate::si::length as l;
//         use crate::tests::Test;

//         #[test]
//         fn check_dimension() {
//             let _: LinearMassDensity<V> = Mass::new::<m::kilogram>(V::one())
//                 / Length::new::<l::meter>(V::one());
//         }

//         #[test]
//         fn check_units() {
//             test::<m::kilogram, l::meter, d::kilogram_per_meter>();
//             test::<m::gram, l::kilometer, d::gram_per_kilometer>();
//             test::<m::gram, l::centimeter, d::gram_per_centimeter>();

//             test::<m::ounce, l::foot, d::ounce_per_foot>();
//             test::<m::ounce, l::inch, d::ounce_per_inch>();
//             test::<m::pound, l::yard, d::pound_per_yard>();
//             test::<m::pound, l::foot, d::pound_per_foot>();
//             test::<m::pound, l::inch, d::pound_per_inch>();

//             fn test<M: m::Conversion<V>, L: l::Conversion<V>, D: d::Conversion<V>>() {
//                 Test::assert_approx_eq(&LinearMassDensity::new::<D>(V::one()),
//                     &(Mass::new::<M>(V::one()) / Length::new::<L>(V::one())));
//             }
//         }
//     }
// }
