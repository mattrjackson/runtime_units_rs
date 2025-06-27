//! Molar energy (base UnitDefinition joule per mole, kg · m² · s⁻² · mol⁻¹).
use crate::{prefix, quantity};
quantity! {
    /// Molar energy (base UnitDefinition joule per mole, kg · m² · s⁻² · mol⁻¹).
    quantity: MolarEnergy; "molar energy";
    /// Dimension of molar energy, L²MT⁻²N⁻¹ (base UnitDefinition joule per mole, kg · m² · s⁻² · mol⁻¹).
    dimension: ISQ[
        2.0,     // length
        1.0,     // mass
        -2.0,     // time
        0.0,     // electric current
        0.0,     // thermodynamic temperature
        -1.0,     // amount of substance
        0.0];    // luminous intensity
    units {
        /// Molar energy expressed in base units. Equivalent to J/mol.
        @kilogram_square_meter_per_second_squared_mole: prefix!(kilo) / prefix!(kilo);
            "kg · m²/(s² · mol)", "kilogram square meter per second squared mole",
            "kilograms square meter per second squared mole";

        // Molar energy is commonly expressed in terms of derived UnitDefinition joule
        @yottajoule_per_mole: prefix!(yotta); "YJ/mol", "yottajoule per mole",
            "yottajoules per mole";
        @zettajoule_per_mole: prefix!(zetta); "ZJ/mol", "zettajoule per mole",
            "zettajoules per mole";
        @exajoule_per_mole: prefix!(exa); "EJ/mol", "exajoule per mole", "exajoules per mole";
        @petajoule_per_mole: prefix!(peta); "PJ/mol", "petajoule per mole", "petajoules per mole";
        @terajoule_per_mole: prefix!(tera); "TJ/mol", "terajoule per mole", "terajoules per mole";
        @gigajoule_per_mole: prefix!(giga); "GJ/mol", "gigajoule per mole", "gigajoules per mole";
        @megajoule_per_mole: prefix!(mega); "MJ/mol", "megajoule per mole", "megajoules per mole";
        @kilojoule_per_mole: prefix!(kilo); "kJ/mol", "kilojoule per mole", "kilojoules per mole";
        @hectojoule_per_mole: prefix!(hecto); "hJ/mol", "hectojoule per mole",
            "hectojoules per mole";
        @decajoule_per_mole: prefix!(deca); "daJ/mol", "decajoule per mole", "decajoules per mole";
        /// The derived UnitDefinition of molar energy. Equivalent to kg · m²/(s² · mol).
        @joule_per_mole: prefix!(none); "J/mol", "joule per mole", "joules per mole";
        @decijoule_per_mole: prefix!(deci); "dJ/mol", "decijoule per mole", "decijoules per mole";
        @centijoule_per_mole: prefix!(centi); "cJ/mol", "centijoule per mole",
            "centijoules per mole";
        @millijoule_per_mole: prefix!(milli); "mJ/mol", "millijoule per mole",
            "millijoules per mole";
        @microjoule_per_mole: prefix!(micro); "µJ/mol", "microjoule per mole",
            "microjoules per mole";
        @nanojoule_per_mole: prefix!(nano); "nJ/mol", "nanojoule per mole", "nanojoules per mole";
        @picojoule_per_mole: prefix!(pico); "pJ/mol", "picojoule per mole", "picojoules per mole";
        @femtojoule_per_mole: prefix!(femto); "fJ/mol", "femtojoule per mole",
            "femtojoules per mole";
        @attojoule_per_mole: prefix!(atto); "aJ/mol", "attojoule per mole", "attojoules per mole";
        @zeptojoule_per_mole: prefix!(zepto); "zJ/mol", "zeptojoule per mole",
            "zeptojoules per mole";
        @yoctojoule_per_mole: prefix!(yocto); "yJ/mol", "yoctojoule per mole",
            "yoctojoules per mole";

        @petawatt_hour_per_mole: 3.6_E18; "PW · h/mol", "petawatt hour per mole",
            "petawatt hours per mole";
        @terawatt_hour_per_mole: 3.6_E15; "TW · h/mol", "terawatt hour per mole",
            "terawatt hours per mole";
        @gigawatt_hour_per_mole: 3.6_E12; "GW · h/mol", "gigawatt hour per mole",
            "gigawatt hours per mole";
        @megawatt_hour_per_mole: 3.6_E9; "MW · h/mol", "megawatt hour per mole",
            "megawatt hours per mole";
        @kilowatt_hour_per_mole: 3.6_E6; "kW · h/mol", "kilowatt hour per mole",
            "kilowatt hours per mole";
        @hectowatt_hour_per_mole: 3.6_E5; "hW · h/mol", "hectowatt hour per mole",
            "hectowatt hours per mole";
        @decawatt_hour_per_mole: 3.6_E4; "daW · h/mol", "decawatt hour per mole",
            "decawatt hours per mole";
        @watt_hour_per_mole: 3.6_E3; "W · h/mol", "watt hour per mole", "watt hours per mole";
        @milliwatt_hour_per_mole: 3.6_E0; "mW · h/mol", "milliwatt hour per mole",
            "milliwatt hours per mole";
        @microwatt_hour_per_mole: 3.6_E-3; "µW · h/mol", "microwatt hour per mole",
            "microwatt hours per mole";

        @btu_it_per_mole: 1.055_056_E3; "Btu (IT)/mol", "British thermal UnitDefinition (IT) per mole",
            "British thermal units (IT) per mole";
        @btu_per_mole: 1.054_350_E3; "Btu/mol", "British thermal UnitDefinition per mole",
            "British thermal units per mole";
        @btu_39_per_mole: 1.059_67_E3; "Btu₃₉/mol", "British thermal UnitDefinition (39 °F) per mole",
            "British thermal units (39 °F) per mole";
        @btu_59_per_mole: 1.054_80_E3; "Btu₅₉/mol", "British thermal UnitDefinition (59 °F) per mole",
            "British thermal units (59 °F) per mole";
        @btu_60_per_mole: 1.054_68_E3; "Btu₆₀/mol", "British thermal UnitDefinition (60 °F) per mole",
            "British thermal units (60 °F) per mole";
        @calorie_it_per_mole: 4.186_8_E0; "cal (IT)/mol", "calorie (IT) per mole",
            "calories (IT) per mole";
        @calorie_per_mole: 4.184_E0; "cal/mol", "calorie per mole", "calories per mole";
        @calorie_15_per_mole: 4.185_80_E0; "cal₁₅/mol", "calorie (15 °C) per mole",
            "calories (15 °C) per mole";
        @calorie_20_per_mole: 4.181_90_E0; "cal₂₀/mol", "calorie (20 °C) per mole",
            "calories (20 °C) per mole";
        @calorie_it_nutrition_per_mole: 4.186_8_E3; "Cal (IT)/mol", "Calorie (IT) per mole",
            "Calories (IT) per mole";
        @calorie_nutrition_per_mole: 4.184_E3; "Cal/mol", "Calorie per mole", "Calories per mole";
        @electronvolt_per_mole: 1.602_176_634_E-19; "eV/mol", "electronvolt per mole",
            "electronvolts per mole";
        @erg_per_mole: 1.0_E-7; "erg/mol", "erg per mole", "ergs per mole";
        @foot_poundal_per_mole: 4.214_011_E-2; "ft · pdl/mol", "foot poundal per mole",
            "foot poundals per mole";
        @foot_pound_force_per_mole: 1.355_818_E0; "ft · lbf/mol", "foot pound-force per mole",
            "foot pounds-force per mole";
        @kilocalorie_it_per_mole: 4.186_8_E3; "kcal (IT)/mol", "kilocalorie (IT) per mole",
            "kilocalories (IT) per mole";
        @kilocalorie_per_mole: 4.184_E3; "kcal/mol", "kilocalorie per mole",
            "kilocalories per mole";
        @quad_per_mole: 1.055_056_E18; "10¹⁵ Btu (IT)/mol", "quad per mole", "quads per mole";
        @therm_ec_per_mole: 1.055_06_E8; "thm (EC)/mol", "therm (EC) per mole",
            "therms (EC) per mole";
        @therm_us_per_mole: 1.054_804_E8; "thm/mol", "therm per mole", "therms per mole";
        @ton_tnt_per_mole: 4.184_E9; "t of TNT/mol", "ton of TNT per mole", "tons of TNT per mole";
        @watt_second_per_mole: 1.0_E0; "W · s/mol", "watt second per mole", "watt seconds per mole";

        @joule_per_particle: 6.022_140_76_E23; "J/particle", "joule per particle",
            "joules per particle";
        @electronvolt_per_particle: 1.602_176_634_E-19 * 6.022_140_76_E23; "eV/particle",
            "electronvolt per particle", "electronvolts per particle";
    }
}

// #[cfg(test)]
// mod tests {
// use crate::traits::Unit;
//     storage_types! {
//         use crate::num::One;
//         use crate::si::amount_of_substance as aos;
//         use crate::si::energy as e;
//         use crate::si::length as l;
//         use crate::si::mass as m;
//         use crate::si::molar_energy as me;
//         use crate::si::quantities::*;
//         use crate::si::time as t;
//         use crate::tests::Test;

//         #[test]
//         fn check_dimension() {
//             let _base: MolarEnergy<V> = Mass::new::<m::kilogram>(V::one())
//                 * Length::new::<l::meter>(V::one())
//                 * Length::new::<l::meter>(V::one())
//                 / (Time::new::<t::second>(V::one())
//                     * Time::new::<t::second>(V::one())
//                     * AmountOfSubstance::new::<aos::mole>(V::one()));

//             let _derived: MolarEnergy<V> = Energy::new::<e::joule>(V::one())
//                 / AmountOfSubstance::new::<aos::mole>(V::one());
//         }

//         #[test]
//         fn check_base_units() {
//             test::<m::kilogram, aos::mole, me::kilogram_square_meter_per_second_squared_mole>();

//             fn test<M: m::Conversion<V>, A: aos::Conversion<V>, ME: me::Conversion<V>>() {
//                 Test::assert_approx_eq(&MolarEnergy::new::<ME>(V::one()),
//                     &(Mass::new::<M>(V::one())
//                         * Length::new::<l::meter>(V::one())
//                         * Length::new::<l::meter>(V::one())
//                         / (Time::new::<t::second>(V::one())
//                             * Time::new::<t::second>(V::one())
//                             * AmountOfSubstance::new::<A>(V::one()))));
//             }
//         }

//         #[test]
//         fn check_derived_units() {
//             test::<e::yottajoule, aos::mole, me::yottajoule_per_mole>();
//             test::<e::zettajoule, aos::mole, me::zettajoule_per_mole>();
//             test::<e::exajoule, aos::mole, me::exajoule_per_mole>();
//             test::<e::petajoule, aos::mole, me::petajoule_per_mole>();
//             test::<e::terajoule, aos::mole, me::terajoule_per_mole>();
//             test::<e::gigajoule, aos::mole, me::gigajoule_per_mole>();
//             test::<e::megajoule, aos::mole, me::megajoule_per_mole>();
//             test::<e::kilojoule, aos::mole, me::kilojoule_per_mole>();
//             test::<e::hectojoule, aos::mole, me::hectojoule_per_mole>();
//             test::<e::decajoule, aos::mole, me::decajoule_per_mole>();
//             test::<e::joule, aos::mole, me::joule_per_mole>();
//             test::<e::decijoule, aos::mole, me::decijoule_per_mole>();
//             test::<e::centijoule, aos::mole, me::centijoule_per_mole>();
//             test::<e::millijoule, aos::mole, me::millijoule_per_mole>();
//             test::<e::microjoule, aos::mole, me::microjoule_per_mole>();
//             test::<e::nanojoule, aos::mole, me::nanojoule_per_mole>();
//             test::<e::picojoule, aos::mole, me::picojoule_per_mole>();
//             test::<e::femtojoule, aos::mole, me::femtojoule_per_mole>();
//             test::<e::attojoule, aos::mole, me::attojoule_per_mole>();
//             test::<e::zeptojoule, aos::mole, me::zeptojoule_per_mole>();
//             test::<e::yoctojoule, aos::mole, me::yoctojoule_per_mole>();

//             test::<e::petawatt_hour, aos::mole, me::petawatt_hour_per_mole>();
//             test::<e::terawatt_hour, aos::mole, me::terawatt_hour_per_mole>();
//             test::<e::gigawatt_hour, aos::mole, me::gigawatt_hour_per_mole>();
//             test::<e::megawatt_hour, aos::mole, me::megawatt_hour_per_mole>();
//             test::<e::kilowatt_hour, aos::mole, me::kilowatt_hour_per_mole>();
//             test::<e::hectowatt_hour, aos::mole, me::hectowatt_hour_per_mole>();
//             test::<e::decawatt_hour, aos::mole, me::decawatt_hour_per_mole>();
//             test::<e::watt_hour, aos::mole, me::watt_hour_per_mole>();
//             test::<e::milliwatt_hour, aos::mole, me::milliwatt_hour_per_mole>();
//             test::<e::microwatt_hour, aos::mole, me::microwatt_hour_per_mole>();

//             test::<e::btu_it, aos::mole, me::btu_it_per_mole>();
//             test::<e::btu, aos::mole, me::btu_per_mole>();
//             test::<e::btu_39, aos::mole, me::btu_39_per_mole>();
//             test::<e::btu_59, aos::mole, me::btu_59_per_mole>();
//             test::<e::btu_60, aos::mole, me::btu_60_per_mole>();
//             test::<e::calorie_it, aos::mole, me::calorie_it_per_mole>();
//             test::<e::calorie, aos::mole, me::calorie_per_mole>();
//             test::<e::calorie_15, aos::mole, me::calorie_15_per_mole>();
//             test::<e::calorie_20, aos::mole, me::calorie_20_per_mole>();
//             test::<e::calorie_it_nutrition, aos::mole, me::calorie_it_nutrition_per_mole>();
//             test::<e::calorie_nutrition, aos::mole, me::calorie_nutrition_per_mole>();
//             test::<e::electronvolt, aos::mole, me::electronvolt_per_mole>();
//             test::<e::erg, aos::mole, me::erg_per_mole>();
//             test::<e::foot_poundal, aos::mole, me::foot_poundal_per_mole>();
//             test::<e::foot_pound, aos::mole, me::foot_pound_force_per_mole>();
//             test::<e::kilocalorie_it, aos::mole, me::kilocalorie_it_per_mole>();
//             test::<e::kilocalorie, aos::mole, me::kilocalorie_per_mole>();
//             test::<e::quad, aos::mole, me::quad_per_mole>();
//             test::<e::therm_ec, aos::mole, me::therm_ec_per_mole>();
//             test::<e::therm_us, aos::mole, me::therm_us_per_mole>();
//             test::<e::ton_tnt, aos::mole, me::ton_tnt_per_mole>();
//             test::<e::watt_second, aos::mole, me::watt_second_per_mole>();

//             test::<e::joule, aos::particle, me::joule_per_particle>();
//             test::<e::electronvolt, aos::particle, me::electronvolt_per_particle>();

//             fn test<E: e::Conversion<V>, AOS: aos::Conversion<V>, ME: me::Conversion<V>>() {
//                 Test::assert_approx_eq(&MolarEnergy::new::<ME>(V::one()),
//                     &(Energy::new::<E>(V::one())
//                         / AmountOfSubstance::new::<AOS>(V::one())));
//             }
//         }
//     }
// }
