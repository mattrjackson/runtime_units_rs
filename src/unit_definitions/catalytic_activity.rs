//! Catalytic activity (base unit katal, mol · s⁻¹).
use crate::{prefix, quantity};
quantity! {
    /// Catalytic activity (base unit katal, mol · s⁻¹).
    quantity: CatalyticActivity; "catalytic activity";
    /// Dimension of catalytic activity, T⁻¹N (base unit katal, mol · s⁻¹).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        P1,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottakatal: prefix!(yotta); "Ykat", "yottakatal", "yottakatals";
        @zettakatal: prefix!(zetta); "Zkat", "zettakatal", "zettakatals";
        @exakatal: prefix!(exa); "Ekat", "exakatal", "exakatals";
        @petakatal: prefix!(peta); "Pkat", "petakatal", "petakatals";
        @terakatal: prefix!(tera); "Tkat", "terakatal", "terakatals";
        @gigakatal: prefix!(giga); "Gkat", "gigakatal", "gigakatals";
        @megakatal: prefix!(mega); "Mkat", "megakatal", "megakatals";
        @kilokatal: prefix!(kilo); "kkat", "kilokatal", "kilokatals";
        @hectokatal: prefix!(hecto); "hkat", "hectokatal", "hectokatals";
        @decakatal: prefix!(deca); "dakat", "decakatal", "decakatals";
        @katal: prefix!(none); "kat", "katal", "katals";
        @decikatal: prefix!(deci); "dkat", "decikatal", "decikatals";
        @centikatal: prefix!(centi); "ckat", "centikatal", "centikatals";
        @millikatal: prefix!(milli); "mkat", "millikatal", "millikatals";
        @microkatal: prefix!(micro); "µkat", "microkatal", "microkatals";
        @nanokatal: prefix!(nano); "nkat", "nanokatal", "nanokatals";
        @picokatal: prefix!(pico); "pkat", "picokatal", "picokatals";
        @femtokatal: prefix!(femto); "fkat", "femtokatal", "femtokatals";
        @attokatal: prefix!(atto); "akat", "attokatal", "attokatals";
        @zeptokatal: prefix!(zepto); "zkat", "zeptokatal", "zeptokatals";
        @yoctokatal: prefix!(yocto); "ykat", "yoctokatal", "yoctokatals";

        @yotta_enzyme_unit: prefix!(yotta) * prefix!(micro) / 6.0_E1; "YU", "yotta enzyme unit",
            "yotta enzyme units";
        @zetta_enzyme_unit: prefix!(zetta) * prefix!(micro) / 6.0_E1; "ZU", "zetta enzyme unit",
            "zetta enzyme units";
        @exa_enzyme_unit: prefix!(exa) * prefix!(micro) / 6.0_E1; "EU", "exa enzyme unit",
            "exa enzyme units";
        @peta_enzyme_unit: prefix!(peta) * prefix!(micro) / 6.0_E1; "PU", "peta enzyme unit",
            "peta enzyme units";
        @tera_enzyme_unit: prefix!(tera) * prefix!(micro) / 6.0_E1; "TU", "tera enzyme unit",
            "tera enzyme units";
        @giga_enzyme_unit: prefix!(giga) * prefix!(micro) / 6.0_E1; "GU", "giga enzyme unit",
            "giga enzyme units";
        @mega_enzyme_unit: prefix!(mega) * prefix!(micro) / 6.0_E1; "MU", "mega enzyme unit",
            "mega enzyme units";
        @kilo_enzyme_unit: prefix!(kilo) * prefix!(micro) / 6.0_E1; "kU", "kilo enzyme unit",
            "kilo enzyme units";
        @hecto_enzyme_unit: prefix!(hecto) * prefix!(micro) / 6.0_E1; "hU", "hecto enzyme unit",
            "hecto enzyme units";
        @deca_enzyme_unit: prefix!(deca) * prefix!(micro) / 6.0_E1; "daU", "deca enzyme unit",
            "deca enzyme units";
        @enzyme_unit: prefix!(none) * prefix!(micro) / 6.0_E1; "U", "enzyme unit", "enzyme units";
        @deci_enzyme_unit: prefix!(deci) * prefix!(micro) / 6.0_E1; "dU", "deci enzyme unit",
            "deci enzyme units";
        @centi_enzyme_unit: prefix!(centi) * prefix!(micro) / 6.0_E1; "cU", "centi enzyme unit",
            "centi enzyme units";
        @milli_enzyme_unit: prefix!(milli) * prefix!(micro) / 6.0_E1; "mU", "milli enzyme unit",
            "milli enzyme units";
        @micro_enzyme_unit: prefix!(micro) * prefix!(micro) / 6.0_E1; "μU", "micro enzyme unit",
            "micro enzyme units";
        @nano_enzyme_unit: prefix!(nano) * prefix!(micro) / 6.0_E1; "nU", "nano enzyme unit",
            "nano enzyme units";
        @pico_enzyme_unit: prefix!(pico) * prefix!(micro) / 6.0_E1; "pU", "pico enzyme unit",
            "pico enzyme units";
        @femto_enzyme_unit: prefix!(femto) * prefix!(micro) / 6.0_E1; "fU", "femto enzyme unit",
            "femto enzyme units";
        @atto_enzyme_unit: prefix!(atto) * prefix!(micro) / 6.0_E1; "aU", "atto enzyme unit",
            "atto enzyme units";
        @zepto_enzyme_unit: prefix!(zepto) * prefix!(micro) / 6.0_E1; "zU", "zepto enzyme unit",
            "zepto enzyme units";
        @yocto_enzyme_unit: prefix!(yocto) * prefix!(micro) / 6.0_E1; "yU", "yocto enzyme unit",
            "yocto enzyme units";

        @particle_per_second: 1.0_E0 / 6.022_140_76_E23; "particle/s", "particle per second",
            "particles per second";

        @mole_per_second: prefix!(none); "mol/s", "mole per second", "moles per second";
        @standard_centimeter_per_minute: 1_E5 * prefix!(micro) / 8.314_462_618 / 273.15 / 60.0;
            "sccm", "standard centimeter per minute", "standard centimeters per minute";
        @standard_liter_per_minute: 1_E5 * prefix!(milli) / 8.314_462_618 / 273.15 / 60.0; "slm",
            "standard liter per minute", "standard liters per minute";
        @standard_cubic_meter_per_minute: 1_E5 * prefix!(none) / 8.314_462_618 / 273.15 / 60.0;
            "m³(STP)/min", "standard cubic meter per minute", "standard cubic meters per minute";
        @standard_cubic_foot_per_minute: 1_E5 * 2.831_685_E-2 / 8.314_462_618 / 273.15 / 60.0;
            "scfm", "standard cubic foot per hour", "standard cubic feet per hour";
    }
}

#[cfg(test)]
mod tests {
    use crate::{unit_definitions::time::TimeUnit, units::{AmountOfSubstanceUnit, CatalyticActivityUnit}, units_base::Unit};

    #[test]
    fn check_dimension() {
        assert_eq!(CatalyticActivityUnit::unit_base(),  AmountOfSubstanceUnit::unit_base() / TimeUnit::unit_base());
    }


    #[test]
    fn check_units() {
        test_unit(AmountOfSubstanceUnit::yottamole, TimeUnit::second, CatalyticActivityUnit::yottakatal);
        test_unit(AmountOfSubstanceUnit::zettamole, TimeUnit::second, CatalyticActivityUnit::zettakatal);
        test_unit(AmountOfSubstanceUnit::examole, TimeUnit::second, CatalyticActivityUnit::exakatal);
        test_unit(AmountOfSubstanceUnit::petamole, TimeUnit::second, CatalyticActivityUnit::petakatal);
        test_unit(AmountOfSubstanceUnit::teramole, TimeUnit::second, CatalyticActivityUnit::terakatal);
        test_unit(AmountOfSubstanceUnit::gigamole, TimeUnit::second, CatalyticActivityUnit::gigakatal);
        test_unit(AmountOfSubstanceUnit::megamole, TimeUnit::second, CatalyticActivityUnit::megakatal);
        test_unit(AmountOfSubstanceUnit::kilomole, TimeUnit::second, CatalyticActivityUnit::kilokatal);
        test_unit(AmountOfSubstanceUnit::hectomole, TimeUnit::second, CatalyticActivityUnit::hectokatal);
        test_unit(AmountOfSubstanceUnit::decamole, TimeUnit::second, CatalyticActivityUnit::decakatal);
        test_unit(AmountOfSubstanceUnit::mole, TimeUnit::second, CatalyticActivityUnit::katal);
        test_unit(AmountOfSubstanceUnit::decimole, TimeUnit::second, CatalyticActivityUnit::decikatal);
        test_unit(AmountOfSubstanceUnit::centimole, TimeUnit::second, CatalyticActivityUnit::centikatal);
        test_unit(AmountOfSubstanceUnit::millimole, TimeUnit::second, CatalyticActivityUnit::millikatal);
        test_unit(AmountOfSubstanceUnit::micromole, TimeUnit::second, CatalyticActivityUnit::microkatal);
        test_unit(AmountOfSubstanceUnit::nanomole, TimeUnit::second, CatalyticActivityUnit::nanokatal);
        test_unit(AmountOfSubstanceUnit::picomole, TimeUnit::second, CatalyticActivityUnit::picokatal);
        test_unit(AmountOfSubstanceUnit::femtomole, TimeUnit::second, CatalyticActivityUnit::femtokatal);
        test_unit(AmountOfSubstanceUnit::attomole, TimeUnit::second, CatalyticActivityUnit::attokatal);
        test_unit(AmountOfSubstanceUnit::zeptomole, TimeUnit::second, CatalyticActivityUnit::zeptokatal);
        test_unit(AmountOfSubstanceUnit::yoctomole, TimeUnit::second, CatalyticActivityUnit::yoctokatal);

        test_unit(AmountOfSubstanceUnit::examole, TimeUnit::minute, CatalyticActivityUnit::yotta_enzyme_unit);
        test_unit(AmountOfSubstanceUnit::petamole, TimeUnit::minute, CatalyticActivityUnit::zetta_enzyme_unit);
        test_unit(AmountOfSubstanceUnit::teramole, TimeUnit::minute, CatalyticActivityUnit::exa_enzyme_unit);
        test_unit(AmountOfSubstanceUnit::gigamole, TimeUnit::minute, CatalyticActivityUnit::peta_enzyme_unit);
        test_unit(AmountOfSubstanceUnit::megamole, TimeUnit::minute, CatalyticActivityUnit::tera_enzyme_unit);
        test_unit(AmountOfSubstanceUnit::kilomole, TimeUnit::minute, CatalyticActivityUnit::giga_enzyme_unit);
        test_unit(AmountOfSubstanceUnit::mole, TimeUnit::minute, CatalyticActivityUnit::mega_enzyme_unit);
        test_unit(AmountOfSubstanceUnit::millimole, TimeUnit::minute, CatalyticActivityUnit::kilo_enzyme_unit);
        test_unit(AmountOfSubstanceUnit::micromole, TimeUnit::minute, CatalyticActivityUnit::enzyme_unit);
        test_unit(AmountOfSubstanceUnit::nanomole, TimeUnit::minute, CatalyticActivityUnit::milli_enzyme_unit);
        test_unit(AmountOfSubstanceUnit::picomole, TimeUnit::minute, CatalyticActivityUnit::micro_enzyme_unit);
        test_unit(AmountOfSubstanceUnit::femtomole, TimeUnit::minute, CatalyticActivityUnit::nano_enzyme_unit);
        test_unit(AmountOfSubstanceUnit::attomole, TimeUnit::minute, CatalyticActivityUnit::pico_enzyme_unit);
        test_unit(AmountOfSubstanceUnit::zeptomole, TimeUnit::minute, CatalyticActivityUnit::femto_enzyme_unit);
        test_unit(AmountOfSubstanceUnit::yoctomole, TimeUnit::minute, CatalyticActivityUnit::atto_enzyme_unit);

        test_unit(AmountOfSubstanceUnit::particle, TimeUnit::second, CatalyticActivityUnit::particle_per_second);
        test_unit(AmountOfSubstanceUnit::mole, TimeUnit::second, CatalyticActivityUnit::mole_per_second);
        test_unit(AmountOfSubstanceUnit::standard_centimeter, TimeUnit::minute, CatalyticActivityUnit::standard_centimeter_per_minute);
        test_unit(AmountOfSubstanceUnit::standard_liter, TimeUnit::minute, CatalyticActivityUnit::standard_liter_per_minute);
        test_unit(AmountOfSubstanceUnit::standard_cubic_foot, TimeUnit::minute, CatalyticActivityUnit::standard_cubic_foot_per_minute);
        test_unit(AmountOfSubstanceUnit::standard_cubic_meter, TimeUnit::minute, CatalyticActivityUnit::standard_cubic_meter_per_minute);

        fn test_unit(amount: AmountOfSubstanceUnit, time: TimeUnit, value: CatalyticActivityUnit) {
            assert!(Into::<Unit>::into(value).approx_eq(Into::<Unit>::into(amount) / Into::<Unit>::into(time), 1e-12));
        }
    }    
}
