//! Molar heat capacity (base UnitDefinition joule per kelvin mole, m² · kg · s⁻² · K⁻¹ · mol⁻¹).
use crate::{prefix, quantity};
quantity! {
    /// Molar heat capacity (base UnitDefinition joule per kelvin mole, m² · kg · s⁻² · K⁻¹ · mol⁻¹).
    quantity: MolarHeatCapacity; "molar heat capacity";
    /// Dimension of molar heat capacity, L²MT⁻²Th⁻¹N⁻¹ (base UnitDefinition joule per kelvin mole,
    /// m² · kg · s⁻² · K⁻¹ · mol⁻¹).
    dimension: ISQ[
        2.0,     // length
        1.0,     // mass
        -2.0,     // time
        0.0,     // electric current
        -1.0,     // thermodynamic temperature
        -1.0,     // amount of substance
        0.0];    // luminous intensity
    units {
        @yottajoule_per_kelvin_mole: prefix!(yotta); "YJ/(K · mol)", "yottajoule per kelvin mole",
            "yottajoules per kelvin mole";
        @zettajoule_per_kelvin_mole: prefix!(zetta); "ZJ/(K · mol)", "zettajoule per kelvin mole",
            "zettajoules per kelvin mole";
        @exajoule_per_kelvin_mole: prefix!(exa); "EJ/(K · mol)", "exajoule per kelvin mole",
            "exajoules per kelvin mole";
        @petajoule_per_kelvin_mole: prefix!(peta); "PJ/(K · mol)", "petajoule per kelvin mole",
            "petajoules per kelvin mole";
        @terajoule_per_kelvin_mole: prefix!(tera); "TJ/(K · mol)", "terajoule per kelvin mole",
            "terajoules per kelvin mole";
        @gigajoule_per_kelvin_mole: prefix!(giga); "GJ/(K · mol)", "gigajoule per kelvin mole",
            "gigajoules per kelvin mole";
        @megajoule_per_kelvin_mole: prefix!(mega); "MJ/(K · mol)", "megajoule per kelvin mole",
            "megajoules per kelvin mole";
        @kilojoule_per_kelvin_mole: prefix!(kilo); "kJ/(K · mol)", "kilojoule per kelvin mole",
            "kilojoules per kelvin mole";
        @hectojoule_per_kelvin_mole: prefix!(hecto); "hJ/(K · mol)", "hectojoule per kelvin mole",
            "hectojoules per kelvin mole";
        @decajoule_per_kelvin_mole: prefix!(deca); "daJ/(K · mol)", "decajoule per kelvin mole",
            "decajoules per kelvin mole";
        /// Derived UnitDefinition of molar heat capacity.
        @joule_per_kelvin_mole: prefix!(none); "J/(K · mol)", "joule per kelvin mole",
            "joules per kelvin mole";
        @decijoule_per_kelvin_mole: prefix!(deci); "dJ/(K · mol)", "decijoule per kelvin mole",
            "decijoules per kelvin mole";
        @centijoule_per_kelvin_mole: prefix!(centi); "cJ/(K · mol)", "centijoule per kelvin mole",
            "centijoules per kelvin mole";
        @millijoule_per_kelvin_mole: prefix!(milli); "mJ/(K · mol)", "millijoule per kelvin mole",
            "millijoules per kelvin mole";
        @microjoule_per_kelvin_mole: prefix!(micro); "µJ/(K · mol)", "microjoule per kelvin mole",
            "microjoules per kelvin mole";
        @nanojoule_per_kelvin_mole: prefix!(nano); "nJ/(K · mol)", "nanojoule per kelvin mole",
            "nanojoules per kelvin mole";
        @picojoule_per_kelvin_mole: prefix!(pico); "pJ/(K · mol)", "picojoule per kelvin mole",
            "picojoules per kelvin mole";
        @femtojoule_per_kelvin_mole: prefix!(femto); "fJ/(K · mol)", "femtojoule per kelvin mole",
            "femtojoules per kelvin mole";
        @attojoule_per_kelvin_mole: prefix!(atto); "aJ/(K · mol)", "attojoule per kelvin mole",
            "attojoules per kelvin mole";
        @zeptojoule_per_kelvin_mole: prefix!(zepto); "zJ/(K · mol)", "zeptojoule per kelvin mole",
            "zeptojoules per kelvin mole";
        @yoctojoule_per_kelvin_mole: prefix!(yocto); "yJ/(K · mol)", "yoctojoule per kelvin mole",
            "yoctojoules per kelvin mole";

        @btu_it_per_kelvin_mole: 1.055_056_E3; "Btu (IT)/(K · mol)",
            "British thermal UnitDefinition (IT) per kelvin mole",
            "British thermal units (IT) per kelvin mole";
        @btu_per_kelvin_mole: 1.054_350_E3; "Btu/(K · mol)", "British thermal UnitDefinition per kelvin mole",
            "British thermal units per kelvin mole";
        @btu_39_per_kelvin_mole: 1.059_67_E3; "Btu₃₉/(K · mol)",
            "British thermal UnitDefinition (39 °F) per kelvin mole",
            "British thermal units (39 °F) per kelvin mole";
        @btu_59_per_kelvin_mole: 1.054_80_E3; "Btu₅₉/(K · mol)",
            "British thermal UnitDefinition (59 °F) per kelvin mole",
            "British thermal units (59 °F) per kelvin mole";
        @btu_60_per_kelvin_mole: 1.054_68_E3; "Btu₆₀/(K · mol)",
            "British thermal UnitDefinition (60 °F) per kelvin mole",
            "British thermal units (60 °F) per kelvin mole";
        @calorie_it_per_kelvin_mole: 4.186_8_E0; "cal (IT)/(K · mol)",
            "calorie (IT) per kelvin mole", "calories (IT) per kelvin mole";
        @calorie_per_kelvin_mole: 4.184_E0; "cal/(K · mol)", "calorie per kelvin mole",
            "calories per kelvin mole";
        @calorie_15_per_kelvin_mole: 4.185_80_E0; "cal₁₅/(K · mol)",
            "calorie (15 °C) per kelvin mole", "calories (15 °C) per kelvin mole";
        @calorie_20_per_kelvin_mole: 4.181_90_E0; "cal₂₀/(K · mol)",
            "calorie (20 °C) per kelvin mole", "calories (20 °C) per kelvin mole";
        @calorie_it_nutrition_per_kelvin_mole: 4.186_8_E3; "Cal (IT)/(K · mol)",
            "Calorie (IT) per kelvin mole", "Calories (IT) per kelvin mole";
        @calorie_nutrition_per_kelvin_mole: 4.184_E3; "Cal/(K · mol)",
            "Calorie per kelvin mole", "Calories per kelvin mole";
        @kilocalorie_it_per_kelvin_mole: 4.186_8_E3; "kcal (IT)/(K · mol)",
            "kilocalorie (IT) per kelvin mole", "kilocalories (IT) per kelvin mole";
        @kilocalorie_per_kelvin_mole: 4.184_E3; "kcal/(K · mol)", "kilocalorie per kelvin mole",
            "kilocalories per kelvin mole";

        @joule_per_kelvin_particle: 6.022_140_76_E23; "J/(K · particle)",
            "joule per kelvin particle", "joules per kelvin particle";
        @electronvolt_per_kelvin_particle: 6.022_140_76_E23 * 1.602_176_634_E-19;
            "eV/(K · particle)", "electronvolt per kelvin particle",
            "electronvolts per kelvin particle";

        @molar_gas_constant: 8.314_462_618_E0; "R", "molar gas constant", "molar gas constants";
    }
}

// #[cfg(test)]
// mod tests {
// use crate::traits::Unit;
//     storage_types! {
//         use crate::num::One;
//         use crate::si::amount_of_substance as aos;
//         use crate::si::energy as e;
//         use crate::si::molar_heat_capacity as m;
//         use crate::si::quantities::*;
//         use crate::si::temperature_interval as t;
//         use crate::tests::Test;

//         #[test]
//         fn check_dimension() {
//             let _: MolarHeatCapacity<V> = Energy::new::<e::joule>(V::one())
//                 / (TemperatureInterval::new::<t::kelvin>(V::one())
//                     * AmountOfSubstance::new::<aos::mole>(V::one()));
//         }

//         #[test]
//         fn check_units() {
//             test::<e::yottajoule, t::kelvin, aos::mole, m::yottajoule_per_kelvin_mole>();
//             test::<e::zettajoule, t::kelvin, aos::mole, m::zettajoule_per_kelvin_mole>();
//             test::<e::exajoule, t::kelvin, aos::mole, m::exajoule_per_kelvin_mole>();
//             test::<e::petajoule, t::kelvin, aos::mole, m::petajoule_per_kelvin_mole>();
//             test::<e::terajoule, t::kelvin, aos::mole, m::terajoule_per_kelvin_mole>();
//             test::<e::gigajoule, t::kelvin, aos::mole, m::gigajoule_per_kelvin_mole>();
//             test::<e::megajoule, t::kelvin, aos::mole, m::megajoule_per_kelvin_mole>();
//             test::<e::kilojoule, t::kelvin, aos::mole, m::kilojoule_per_kelvin_mole>();
//             test::<e::hectojoule, t::kelvin, aos::mole, m::hectojoule_per_kelvin_mole>();
//             test::<e::decajoule, t::kelvin, aos::mole, m::decajoule_per_kelvin_mole>();
//             test::<e::joule, t::kelvin, aos::mole, m::joule_per_kelvin_mole>();
//             test::<e::decijoule, t::kelvin, aos::mole, m::decijoule_per_kelvin_mole>();
//             test::<e::centijoule, t::kelvin, aos::mole, m::centijoule_per_kelvin_mole>();
//             test::<e::millijoule, t::kelvin, aos::mole, m::millijoule_per_kelvin_mole>();
//             test::<e::microjoule, t::kelvin, aos::mole, m::microjoule_per_kelvin_mole>();
//             test::<e::nanojoule, t::kelvin, aos::mole, m::nanojoule_per_kelvin_mole>();
//             test::<e::picojoule, t::kelvin, aos::mole, m::picojoule_per_kelvin_mole>();
//             test::<e::femtojoule, t::kelvin, aos::mole, m::femtojoule_per_kelvin_mole>();
//             test::<e::attojoule, t::kelvin, aos::mole, m::attojoule_per_kelvin_mole>();
//             test::<e::zeptojoule, t::kelvin, aos::mole, m::zeptojoule_per_kelvin_mole>();
//             test::<e::yoctojoule, t::kelvin, aos::mole, m::yoctojoule_per_kelvin_mole>();

//             test::<e::btu_it, t::kelvin, aos::mole, m::btu_it_per_kelvin_mole>();
//             test::<e::btu, t::kelvin, aos::mole, m::btu_per_kelvin_mole>();
//             test::<e::btu_39, t::kelvin, aos::mole, m::btu_39_per_kelvin_mole>();
//             test::<e::btu_59, t::kelvin, aos::mole, m::btu_59_per_kelvin_mole>();
//             test::<e::btu_60, t::kelvin, aos::mole, m::btu_60_per_kelvin_mole>();
//             test::<e::calorie_it, t::kelvin, aos::mole, m::calorie_it_per_kelvin_mole>();
//             test::<e::calorie, t::kelvin, aos::mole, m::calorie_per_kelvin_mole>();
//             test::<e::calorie_15, t::kelvin, aos::mole, m::calorie_15_per_kelvin_mole>();
//             test::<e::calorie_20, t::kelvin, aos::mole, m::calorie_20_per_kelvin_mole>();
//             test::<e::calorie_it_nutrition, t::kelvin, aos::mole, m::calorie_it_nutrition_per_kelvin_mole>();
//             test::<e::calorie_nutrition, t::kelvin, aos::mole, m::calorie_nutrition_per_kelvin_mole>();
//             test::<e::kilocalorie_it, t::kelvin, aos::mole, m::kilocalorie_it_per_kelvin_mole>();
//             test::<e::kilocalorie, t::kelvin, aos::mole, m::kilocalorie_per_kelvin_mole>();

//             test::<e::joule, t::kelvin, aos::particle, m::joule_per_kelvin_particle>();
//             test::<e::electronvolt, t::kelvin, aos::particle,
//                 m::electronvolt_per_kelvin_particle>();

//             fn test<E: e::Conversion<V>, T: t::Conversion<V>, AOS: aos::Conversion<V>, M: m::Conversion<V>>() {
//                 Test::assert_approx_eq(&MolarHeatCapacity::new::<M>(V::one()),
//                     &(Energy::new::<E>(V::one())
//                         / (TemperatureInterval::new::<T>(V::one())
//                             * AmountOfSubstance::new::<AOS>(V::one()))));
//             }
//         }
//     }
// }
