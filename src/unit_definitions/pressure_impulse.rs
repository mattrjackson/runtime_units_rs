//! Pressure (base UnitDefinition pascal, kg · m⁻¹ · s⁻²).
use crate::{prefix, quantity};
quantity! {
    /// Pressure impulse (base UnitDefinition pascal-sec, kg · m⁻¹ · s⁻¹).
    quantity: PressureImpulse; "pressure_impulse";
    /// Dimension of pressure impulse, L⁻¹MT⁻¹ (base UnitDefinition pascal-sec, kg · m⁻¹ · s⁻¹).
    dimension: ISQ<
        N1,     // length
        P1,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::PressureImpulseKind);
    units {
        @yottapascal_sec : prefix!(yotta); "YPa-sec", "yottapascal-sec", "yottapascals-sec";
        @zettapascal_sec : prefix!(zetta); "ZPa-sec", "zettapascal-sec", "zettapascals-sec";
        @exapascal_sec : prefix!(exa); "EPa-sec", "exapascal-sec", "exapascals-sec";
        @petapascal_sec : prefix!(peta); "PPa-sec", "petapascal-sec", "petapascals-sec";
        @terapascal_sec : prefix!(tera); "TPa-sec", "terapascal-sec", "terapascals-sec";
        @gigapascal_sec : prefix!(giga); "GPa-sec", "gigapascal-sec", "gigapascals-sec";
        @megapascal_sec : prefix!(mega); "MPa-sec", "megapascal-sec", "megapascals-sec";
        @kilopascal_sec : prefix!(kilo); "kPa-sec", "kilopascal-sec", "kilopascals-sec";
        @hectopascal_sec : prefix!(hecto); "hPa-sec", "hectopascal-sec", "hectopascals-sec";
        @decapascal_sec : prefix!(deca); "daPa-sec", "decapascal-sec", "decapascals-sec";
        /// Derived UnitDefinition of pressure.
        @pascal_sec : prefix!(none); "Pa-sec", "pascal-sec", "pascals-sec";
        @decipascal_sec : prefix!(deci); "dPa-sec", "decipascal-sec", "decipascals-sec";
        @centipascal_sec : prefix!(centi); "cPa-sec", "centipascal-sec", "centipascals-sec";
        @millipascal_sec : prefix!(milli); "mPa-sec", "millipascal-sec", "millipascals-sec";
        @micropascal_sec : prefix!(micro); "µPa-sec", "micropascal-sec", "micropascals-sec";
        @nanopascal_sec : prefix!(nano); "nPa-sec", "nanopascal-sec", "nanopascals-sec";
        @picopascal_sec : prefix!(pico); "pPa-sec", "picopascal-sec", "picopascals-sec";
        @femtopascal_sec : prefix!(femto); "fPa-sec", "femtopascal-sec", "femtopascals-sec";
        @attopascal_sec : prefix!(atto); "aPa-sec", "attopascal-sec", "attopascals-sec";
        @zeptopascal_sec : prefix!(zepto); "zPa-sec", "zeptopascal-sec", "zeptopascals-sec";
        @yoctopascal_sec : prefix!(yocto); "yPa-sec", "yoctopascal-sec", "yoctopascals-sec";

        @bar_sec: 1.0_E5; "bar-sec", "bar-sec", "bar-sec";
        @dyne_per_square_centimeter_sec: 1.0_E-1; "dyn/cm²-sec", "dyne second per square centimeter",
            "dyne seconds per square centimeter";        
        @gram_force_per_square_centimeter: 9.806_65_E1; "gf/cm²-sec",
            "gram-force second per square centimeter", "grams-force second per square centimeter";
        @kilogram_force_per_square_centimeter_sec: 9.806_65_E4; "kgf/cm²-sec",
            "kilogram-force second per square centimeter", "kilograms-force second per square centimeter";
        @kilogram_force_per_square_meter_sec: 9.806_65_E0; "kgf/m²-sec",
            "kilogram-force second per square meter", "kilograms-force second per square meter";
        @kilogram_force_per_square_millimeter_sec: 9.806_65_E6; "kgf/mm²-sec",
            "kilogram-force second per square millimeter", "kilograms-force second per square millimeter";
        @pound_force_per_square_foot_sec: 4.788_026_312_163_735_6_E1; "lbf/ft²-sec", "pound-force second per square foot",
            "pounds-force seconds per square foot second";
        @pound_force_per_square_inch_sec: 6.894_757_889_515_779_E3; "lbf/in²", "pound-force second per square inch",
            "pounds-force seconds per square inch";
        @psi_sec: 6.894_757_E3; "psi-sec", "pound-force second per square inch second",
            "pounds-force second per square inch second";
        @torr_sec: 1.333_224_E2; "Torr", "torr", "torr";
    }
    
}

// #[cfg(test)]
// mod tests {
//     storage_types! {
//         use crate::num::One;
//         use crate::si::area as a;
//         use crate::si::force as f;
//         use crate::si::time as t;
//         use crate::si::pressure_impulse as p;
//         use crate::si::quantities::*;
//         use crate::tests::Test;


//         #[test]
//         fn check_units() {
//             Test::assert_eq(&PressureImpulse::new::<p::pascal_sec>(1.0e3),
//                 &PressureImpulse::new::<p::kilopascal_sec>(V::one()));
            
//         }
        
//     }
// }
