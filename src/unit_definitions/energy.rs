//! Energy (base unit joule, kg · m² · s⁻²).
use crate::{prefix, quantity};
quantity! {
    /// Energy (base unit joule, kg · m² · s⁻²).
    quantity: Energy; "energy";
    /// Dimension of energy, L²MT⁻² (base unit joule, kg · m² · s⁻²).
    dimension: ISQ<
        P2,     // length
        P1,     // mass
        N2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottajoule: prefix!(yotta); "YJ", "yottajoule", "yottajoules";
        @zettajoule: prefix!(zetta); "ZJ", "zettajoule", "zettajoules";
        @exajoule: prefix!(exa); "EJ", "exajoule", "exajoules";
        @petajoule: prefix!(peta); "PJ", "petajoule", "petajoules";
        @terajoule: prefix!(tera); "TJ", "terajoule", "terajoules";
        @gigajoule: prefix!(giga); "GJ", "gigajoule", "gigajoules";
        @megajoule: prefix!(mega); "MJ", "megajoule", "megajoules";
        @kilojoule: prefix!(kilo); "kJ", "kilojoule", "kilojoules";
        @hectojoule: prefix!(hecto); "hJ", "hectojoule", "hectojoules";
        @decajoule: prefix!(deca); "daJ", "decajoule", "decajoules";
        /// Derived unit of energy.
        @joule: prefix!(none); "J", "joule", "joules";
        @decijoule: prefix!(deci); "dJ", "decijoule", "decijoules";
        @centijoule: prefix!(centi); "cJ", "centijoule", "centijoules";
        @millijoule: prefix!(milli); "mJ", "millijoule", "millijoules";
        @microjoule: prefix!(micro); "µJ", "microjoule", "microjoules";
        @nanojoule: prefix!(nano); "nJ", "nanojoule", "nanojoules";
        @picojoule: prefix!(pico); "pJ", "picojoule", "picojoules";
        @femtojoule: prefix!(femto); "fJ", "femtojoule", "femtojoules";
        @attojoule: prefix!(atto); "aJ", "attojoule", "attojoules";
        @zeptojoule: prefix!(zepto); "zJ", "zeptojoule", "zeptojoules";
        @yoctojoule: prefix!(yocto); "yJ", "yoctojoule", "yoctojoules";

        @petawatt_hour: 3.6_E18; "PW · h", "petawatt hour", "petawatt hours";
        @terawatt_hour: 3.6_E15; "TW · h", "terawatt hour", "terawatt hours";
        @gigawatt_hour: 3.6_E12; "GW · h", "gigawatt hour", "gigawatt hours";
        @megawatt_hour: 3.6_E9; "MW · h", "megawatt hour", "megawatt hours";
        @kilowatt_hour: 3.6_E6; "kW · h", "kilowatt hour", "kilowatt hours";
        @hectowatt_hour: 3.6_E5; "hW · h", "hectowatt hour", "hectowatt hours";
        @decawatt_hour: 3.6_E4; "daW · h", "decawatt hour", "decawatt hours";
        @watt_hour: 3.6_E3; "W · h", "watt hour", "watt hours";
        @milliwatt_hour: 3.6_E0; "mW · h", "milliwatt hour", "milliwatt hours";
        @microwatt_hour: 3.6_E-3; "µW · h", "microwatt hour", "microwatt hours";

        @petaelectronvolt: 1.602_176_634_E-4; "PeV", "petaelectronvolt", "petaelectronvolts";
        @teraelectronvolt: 1.602_176_634_E-7; "TeV", "teraelectronvolt", "teraelectronvolts";
        @gigaelectronvolt: 1.602_176_634_E-10; "GeV", "gigaelectronvolt", "gigaelectronvolts";
        @megaelectronvolt: 1.602_176_634_E-13; "MeV", "megaelectronvolt", "megaelectronvolts";
        @kiloelectronvolt: 1.602_176_634_E-16; "keV", "kiloelectronvolt", "kiloelectronvolts";
        @hectoelectronvolt: 1.602_176_634_E-17; "heV", "hectoelectronvolt", "hectoelectronvolts";
        @decaelectronvolt: 1.602_176_634_E-18; "daeV", "decaelectronvolt", "decaelectronvolts";
        @electronvolt: 1.602_176_634_E-19; "eV", "electronvolt", "electronvolts";

        /// Atomic unit of energy (Hartree energy).
        @hartree: 4.359_744_722_207_1_E-18; "Eₕ", "hartree", "hartrees";

        @btu_it: 1.055_056_E3; "Btu (IT)", "British thermal unit (IT)",
            "British thermal units (IT)";
        @btu: 1.054_350_E3; "Btu", "British thermal unit", "British thermal units";
        @btu_39: 1.059_67_E3; "Btu₃₉", "British thermal unit (39 °F)",
            "British thermal units (39 °F)";
        @btu_59: 1.054_80_E3; "Btu₅₉", "British thermal unit (59 °F)",
            "British thermal units (59 °F)";
        @btu_60: 1.054_68_E3; "Btu₆₀", "British thermal unit (60 °F)",
            "British thermal units (60 °F)";
        @calorie_it: 4.186_8_E0; "cal (IT)", "calorie (IT)", "calories (IT)";
        @calorie: 4.184_E0; "cal", "calorie", "calories";
        @calorie_15: 4.185_80_E0; "cal₁₅", "calorie (15 °C)", "calories (15 °C)";
        @calorie_20: 4.181_90_E0; "cal₂₀", "calorie (20 °C)", "calories (20 °C)";
        @calorie_it_nutrition: 4.186_8_E3; "Cal (IT)", "Calorie (IT)", "Calories (IT)";
        @calorie_nutrition: 4.184_E3; "Cal", "Calorie", "Calories";
        @erg: 1.0_E-7; "erg", "erg", "ergs";
        @foot_poundal: 4.214_011_E-2; "ft · pdl", "foot poundal", "foot poundals";
        @foot_pound: 1.355_818_E0; "ft · lbf", "foot pound-force",
            "foot pounds-force"; // @foot_pound_force
        @kilocalorie_it: 4.186_8_E3; "kcal (IT)", "kilocalorie (IT)", "kilocalories (IT)";
        @kilocalorie: 4.184_E3; "kcal", "kilocalorie", "kilocalories";
        @quad: 1.055_056_E18; "10¹⁵ Btu (IT)", "quad", "quads";
        @therm_ec: 1.055_06_E8; "thm (EC)", "therm (EC)", "therms (EC)";
        @therm_us: 1.054_804_E8; "thm", "therm", "therms";
        @ton_tnt: 4.184_E9; "t of TNT", "ton of TNT", "tons of TNT";
        @kiloton_tnt:  4.184_E12; "kt", "kiloton", "kilotons";
        @watt_second: 1.0_E0; "W · s", "watt second", "watt seconds";
    }
}

#[cfg(test)]
#[cfg(feature="Energy")]
mod tests {
    use crate::{quantities, units::{MassUnit, LengthUnit, EnergyUnit, TimeUnit}, units_base::{Unit, UnitBase}};
    
    #[test]
    fn check_dimension() {
        assert_eq!(EnergyUnit::unit_base(), MassUnit::unit_base()*LengthUnit::unit_base().powi(2) / TimeUnit::unit_base().powi(2));
    }

    #[test]
    fn check_units() {
        test_unit(MassUnit::yottagram, EnergyUnit::zettajoule);
        test_unit(MassUnit::zettagram, EnergyUnit::exajoule);
        test_unit(MassUnit::exagram, EnergyUnit::petajoule);
        test_unit(MassUnit::petagram, EnergyUnit::terajoule);
        test_unit(MassUnit::teragram, EnergyUnit::gigajoule);
        test_unit(MassUnit::gigagram, EnergyUnit::megajoule);
        test_unit(MassUnit::megagram, EnergyUnit::kilojoule);
        test_unit(MassUnit::kilogram, EnergyUnit::joule);
        test_unit(MassUnit::decagram, EnergyUnit::centijoule);
        test_unit(MassUnit::gram, EnergyUnit::millijoule);
        test_unit(MassUnit::milligram, EnergyUnit::microjoule);
        test_unit(MassUnit::microgram, EnergyUnit::nanojoule);
        test_unit(MassUnit::nanogram, EnergyUnit::picojoule);
        test_unit(MassUnit::picogram, EnergyUnit::femtojoule);
        test_unit(MassUnit::femtogram, EnergyUnit::attojoule);
        test_unit(MassUnit::attogram, EnergyUnit::zeptojoule);
        test_unit(MassUnit::zeptogram, EnergyUnit::yoctojoule);
        
    }
    fn test_unit(mass: MassUnit, unit: EnergyUnit) {
        assert!(Into::<Unit>::into(unit).approx_eq(Into::<Unit>::into(mass) * Into::<Unit>::into(LengthUnit::meter).powi(2) / Into::<Unit>::into(TimeUnit::second).powi(2), 1e-12));
    }
    
}
