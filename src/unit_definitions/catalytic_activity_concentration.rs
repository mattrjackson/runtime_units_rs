 //! Catalytic activity concentration (base UnitDefinition katal per cubic meter, mol · s⁻¹ · m⁻³).
use crate::{prefix, quantity};
quantity! {
    /// Catalytic activity concentration (base UnitDefinition katal per cubic meter, mol · s⁻¹ · m⁻³).
    quantity: CatalyticActivityConcentration; "catalytic activity concentration";
    /// Dimension of catalytic activity concentration, L⁻³T⁻¹N
    /// (base UnitDefinition katal per cubic meter, mol · s⁻¹ · m⁻³).
    dimension: ISQ<
        N3,     // length
        Z0,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        P1,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::ConstituentConcentrationKind);
    units {
        @yottakatal_per_cubic_meter: prefix!(yotta); "Ykat/m³",
            "yottakatal per cubic meter", "yottakatals per cubic meter";
        @zettakatal_per_cubic_meter: prefix!(zetta); "Zkat/m³",
            "zettakatal per cubic meter", "zettakatals per cubic meter";
        @exakatal_per_cubic_meter: prefix!(exa); "Ekat/m³",
            "exakatal per cubic meter", "exakatals per cubic meter";
        @petakatal_per_cubic_meter: prefix!(peta); "Pkat/m³",
            "petakatal per cubic meter", "petakatals per cubic meter";
        @terakatal_per_cubic_meter: prefix!(tera); "Tkat/m³",
            "terakatal per cubic meter", "terakatals per cubic meter";
        @gigakatal_per_cubic_meter: prefix!(giga); "Gkat/m³",
            "gigakatal per cubic meter", "gigakatals per cubic meter";
        @megakatal_per_cubic_meter: prefix!(mega); "Mkat/m³",
            "megakatal per cubic meter", "megakatals per cubic meter";
        @kilokatal_per_cubic_meter: prefix!(kilo); "kkat/m³",
            "kilokatal per cubic meter", "kilokatals per cubic meter";
        @hectokatal_per_cubic_meter: prefix!(hecto); "hkat/m³",
            "hectokatal per cubic meter", "hectokatals per cubic meter";
        @decakatal_per_cubic_meter: prefix!(deca); "dakat/m³",
            "decakatal per cubic meter", "decakatals per cubic meter";
        @katal_per_cubic_meter: prefix!(none); "kat/m³",
            "katal per cubic meter", "katals per cubic meter";
        @decikatal_per_cubic_meter: prefix!(deci); "dkat/m³",
            "decikatal per cubic meter", "decikatals per cubic meter";
        @centikatal_per_cubic_meter: prefix!(centi); "ckat/m³",
            "centikatal per cubic meter", "centikatals per cubic meter";
        @millikatal_per_cubic_meter: prefix!(milli); "mkat/m³",
            "millikatal per cubic meter", "millikatals per cubic meter";
        @microkatal_per_cubic_meter: prefix!(micro); "µkat/m³",
            "microkatal per cubic meter", "microkatals per cubic meter";
        @nanokatal_per_cubic_meter: prefix!(nano); "nkat/m³",
            "nanokatal per cubic meter", "nanokatals per cubic meter";
        @picokatal_per_cubic_meter: prefix!(pico); "pkat/m³",
            "picokatal per cubic meter", "picokatals per cubic meter";
        @femtokatal_per_cubic_meter: prefix!(femto); "fkat/m³",
            "femtokatal per cubic meter", "femtokatals per cubic meter";
        @attokatal_per_cubic_meter: prefix!(atto); "akat/m³",
            "attokatal per cubic meter", "attokatals per cubic meter";
        @zeptokatal_per_cubic_meter: prefix!(zepto); "zkat/m³",
            "zeptokatal per cubic meter", "zeptokatals per cubic meter";
        @yoctokatal_per_cubic_meter: prefix!(yocto); "ykat/m³",
            "yoctokatal per cubic meter", "yoctokatals per cubic meter";

        @kilokatal_per_cubic_decimeter:
            prefix!(kilo) / prefix!(deci) / prefix!(deci) / prefix!(deci);
            "kkat/dm³", "kilokatal per cubic decimeter", "kilokatals per cubic decimeter";
        @kilokatal_per_liter:
            prefix!(kilo) / prefix!(milli);
            "kkat/L", "kilokatal per liter", "kilokatals per liter";
        @katal_per_cubic_decimeter:
            prefix!(none) / prefix!(deci) / prefix!(deci) / prefix!(deci);
            "kat/dm³", "katal per cubic decimeter", "katals per cubic decimeter";
        @katal_per_liter:
            prefix!(none) / prefix!(milli);
            "kat/L", "katal per liter", "katals per liter";
        @millikatal_per_cubic_decimeter:
            prefix!(milli) / prefix!(deci) / prefix!(deci) / prefix!(deci);
            "mkat/dm³", "millikatal per cubic decimeter", "millikatals per cubic decimeter";
        @millikatal_per_liter:
            prefix!(milli) / prefix!(milli);
            "mkat/L", "millikatal per liter", "millikatals per liter";
        @microkatal_per_cubic_decimeter:
            prefix!(micro) / prefix!(deci) / prefix!(deci) / prefix!(deci);
            "μkat/dm³", "microkatal per cubic decimeter", "microkatals per cubic decimeter";
        @microkatal_per_liter:
            prefix!(micro) / prefix!(milli);
            "μkat/L", "microkatal per liter", "microkatals per liter";
        @nanokatal_per_cubic_decimeter:
            prefix!(nano) / prefix!(deci) / prefix!(deci) / prefix!(deci);
            "nkat/dm³", "nanokatal per cubic decimeter", "nanokatals per cubic decimeter";
        @nanokatal_per_liter:
            prefix!(nano) / prefix!(milli);
            "nkat/L", "nanokatal per liter", "nanokatals per liter";
        @picokatal_per_cubic_decimeter:
            prefix!(pico) / prefix!(deci) / prefix!(deci) / prefix!(deci);
            "pkat/dm³", "picokatal per cubic decimeter", "picokatals per cubic decimeter";
        @picokatal_per_liter:
            prefix!(pico) / prefix!(milli);
            "pkat/L", "picokatal per liter", "picokatals per liter";
        @femtokatal_per_cubic_decimeter:
            prefix!(femto) / prefix!(deci) / prefix!(deci) / prefix!(deci);
            "fkat/dm³", "femtokatal per cubic decimeter", "femtokatals per cubic decimeter";
        @femtokatal_per_liter:
            prefix!(femto) / prefix!(milli);
            "fkat/L", "femtokatal per liter", "femtokatals per liter";

        @kilokatal_per_deciliter:
            prefix!(kilo) / prefix!(deci) / prefix!(milli);
            "kkat/dL", "kilokatal per deciliter", "kilokatals per deciliter";
        @katal_per_deciliter:
            prefix!(none) / prefix!(deci) / prefix!(milli);
            "kat/dL", "katal per deciliter", "katals per deciliter";
        @millikatal_per_deciliter:
            prefix!(milli) / prefix!(deci) / prefix!(milli);
            "mkat/dL", "millikatal per deciliter", "millikatals per deciliter";
        @microkatal_per_deciliter:
            prefix!(micro) / prefix!(deci) / prefix!(milli);
            "μkat/dL", "microkatal per deciliter", "microkatals per deciliter";
        @nanokatal_per_deciliter:
            prefix!(nano) / prefix!(deci) / prefix!(milli);
            "nkat/dL", "nanokatal per deciliter", "nanokatals per deciliter";
        @picokatal_per_deciliter:
            prefix!(pico) / prefix!(deci) / prefix!(milli);
            "pkat/dL", "picokatal per deciliter", "picokatals per deciliter";
        @femtokatal_per_deciliter:
            prefix!(femto) / prefix!(deci) / prefix!(milli);
            "fkat/dL", "femtokatal per deciliter", "femtokatals per deciliter";

        @kilokatal_per_milliliter:
            prefix!(kilo) / prefix!(milli) / prefix!(milli);
            "kkat/mL", "kilokatal per milliliter", "kilokatals per milliliter";
        @katal_per_milliliter:
            prefix!(none) / prefix!(milli) / prefix!(milli);
            "kat/mL", "katal per milliliter", "katals per milliliter";
        @millikatal_per_milliliter:
            prefix!(milli) / prefix!(milli) / prefix!(milli);
            "mkat/mL", "millikatal per milliliter", "millikatals per milliliter";
        @microkatal_per_milliliter:
            prefix!(micro) / prefix!(milli) / prefix!(milli);
            "μkat/mL", "microkatal per milliliter", "microkatals per milliliter";
        @nanokatal_per_milliliter:
            prefix!(nano) / prefix!(milli) / prefix!(milli);
            "nkat/mL", "nanokatal per milliliter", "nanokatals per milliliter";
        @picokatal_per_milliliter:
            prefix!(pico) / prefix!(milli) / prefix!(milli);
            "pkat/mL", "picokatal per milliliter", "picokatals per milliliter";
        @femtokatal_per_milliliter:
            prefix!(femto) / prefix!(milli) / prefix!(milli);
            "fkat/mL", "femtokatal per milliliter", "femtokatals per milliliter";

        @yotta_enzyme_unit_per_cubic_meter: prefix!(yotta) * prefix!(micro) / 6.0_E1; "YU/m³",
            "yotta enzyme UnitDefinition per cubic meter", "yotta enzyme units per cubic meter";
        @zetta_enzyme_unit_per_cubic_meter: prefix!(zetta) * prefix!(micro) / 6.0_E1; "ZU/m³",
            "zetta enzyme UnitDefinition per cubic meter", "zetta enzyme units per cubic meter";
        @exa_enzyme_unit_per_cubic_meter: prefix!(exa) * prefix!(micro) / 6.0_E1; "EU/m³",
            "exa enzyme UnitDefinition per cubic meter", "exa enzyme units per cubic meter";
        @peta_enzyme_unit_per_cubic_meter: prefix!(peta) * prefix!(micro) / 6.0_E1; "PU/m³",
            "peta enzyme UnitDefinition per cubic meter", "peta enzyme units per cubic meter";
        @tera_enzyme_unit_per_cubic_meter: prefix!(tera) * prefix!(micro) / 6.0_E1; "TU/m³",
            "tera enzyme UnitDefinition per cubic meter", "tera enzyme units per cubic meter";
        @giga_enzyme_unit_per_cubic_meter: prefix!(giga) * prefix!(micro) / 6.0_E1; "GU/m³",
            "giga enzyme UnitDefinition per cubic meter", "giga enzyme units per cubic meter";
        @mega_enzyme_unit_per_cubic_meter: prefix!(mega) * prefix!(micro) / 6.0_E1; "MU/m³",
            "mega enzyme UnitDefinition per cubic meter", "mega enzyme units per cubic meter";
        @kilo_enzyme_unit_per_cubic_meter: prefix!(kilo) * prefix!(micro) / 6.0_E1; "kU/m³",
            "kilo enzyme UnitDefinition per cubic meter", "kilo enzyme units per cubic meter";
        @hecto_enzyme_unit_per_cubic_meter: prefix!(hecto) * prefix!(micro) / 6.0_E1; "hU/m³",
            "hecto enzyme UnitDefinition per cubic meter", "hecto enzyme units per cubic meter";
        @deca_enzyme_unit_per_cubic_meter: prefix!(deca) * prefix!(micro) / 6.0_E1; "daU/m³",
            "deca enzyme UnitDefinition per cubic meter", "deca enzyme units per cubic meter";
        @enzyme_unit_per_cubic_meter: prefix!(none) * prefix!(micro) / 6.0_E1; "U/m³",
            "enzyme UnitDefinition per cubic meter", "enzyme units per cubic meter";
        @deci_enzyme_unit_per_cubic_meter: prefix!(deci) * prefix!(micro) / 6.0_E1; "dU/m³",
            "deci enzyme UnitDefinition per cubic meter", "deci enzyme units per cubic meter";
        @centi_enzyme_unit_per_cubic_meter: prefix!(centi) * prefix!(micro) / 6.0_E1; "cU/m³",
            "centi enzyme UnitDefinition per cubic meter", "centi enzyme units per cubic meter";
        @milli_enzyme_unit_per_cubic_meter: prefix!(milli) * prefix!(micro) / 6.0_E1; "mU/m³",
            "milli enzyme UnitDefinition per cubic meter", "milli enzyme units per cubic meter";
        @micro_enzyme_unit_per_cubic_meter: prefix!(micro) * prefix!(micro) / 6.0_E1; "µU/m³",
            "micro enzyme UnitDefinition per cubic meter", "micro enzyme units per cubic meter";
        @nano_enzyme_unit_per_cubic_meter: prefix!(nano) * prefix!(micro) / 6.0_E1; "nU/m³",
            "nano enzyme UnitDefinition per cubic meter", "nano enzyme units per cubic meter";
        @pico_enzyme_unit_per_cubic_meter: prefix!(pico) * prefix!(micro) / 6.0_E1; "pU/m³",
            "pico enzyme UnitDefinition per cubic meter", "pico enzyme units per cubic meter";
        @femto_enzyme_unit_per_cubic_meter: prefix!(femto) * prefix!(micro) / 6.0_E1; "fU/m³",
            "femto enzyme UnitDefinition per cubic meter", "femto enzyme units per cubic meter";
        @atto_enzyme_unit_per_cubic_meter: prefix!(atto) * prefix!(micro) / 6.0_E1; "aU/m³",
            "atto enzyme UnitDefinition per cubic meter", "atto enzyme units per cubic meter";
        @zepto_enzyme_unit_per_cubic_meter: prefix!(zepto) * prefix!(micro) / 6.0_E1; "zU/m³",
            "zepto enzyme UnitDefinition per cubic meter", "zepto enzyme units per cubic meter";
        @yocto_enzyme_unit_per_cubic_meter: prefix!(yocto) * prefix!(micro) / 6.0_E1; "yU/m³",
            "yocto enzyme UnitDefinition per cubic meter", "yocto enzyme units per cubic meter";

        @kilo_enzyme_unit_per_cubic_decimeter:
            prefix!(kilo) * prefix!(micro) / 6.0_E1 / prefix!(milli); "kU/dm³",
            "kilo enzyme UnitDefinition per cubic decimeter", "kilo enzyme units per cubic decimeter";
        @kilo_enzyme_unit_per_liter:
            prefix!(kilo) * prefix!(micro) / 6.0_E1 / prefix!(milli); "kU/L",
            "kilo enzyme UnitDefinition per liter", "kilo enzyme units per liter";
        @enzyme_unit_per_cubic_decimeter:
            prefix!(none) * prefix!(micro) / 6.0_E1 / prefix!(milli); "U/dm³",
            "enzyme UnitDefinition per cubic decimeter", "enzyme units per cubic decimeter";
        @enzyme_unit_per_liter:
            prefix!(none) * prefix!(micro) / 6.0_E1 / prefix!(milli); "U/L",
            "enzyme UnitDefinition per liter", "enzyme units per liter";
        @milli_enzyme_unit_per_cubic_decimeter:
            prefix!(milli) * prefix!(micro) / 6.0_E1 / prefix!(milli); "mU/dm³",
            "milli enzyme UnitDefinition per cubic decimeter", "milli enzyme units per cubic decimeter";
        @milli_enzyme_unit_per_liter:
            prefix!(milli) * prefix!(micro) / 6.0_E1 / prefix!(milli); "mU/L",
            "milli enzyme UnitDefinition per liter", "milli enzyme units per liter";
        @micro_enzyme_unit_per_cubic_decimeter:
            prefix!(micro) * prefix!(micro) / 6.0_E1 / prefix!(milli); "μU/dm³",
            "micro enzyme UnitDefinition per cubic decimeter", "micro enzyme units per cubic decimeter";
        @micro_enzyme_unit_per_liter:
            prefix!(micro) * prefix!(micro) / 6.0_E1 / prefix!(milli); "μU/L",
            "micro enzyme UnitDefinition per liter", "micro enzyme units per liter";
        @nano_enzyme_unit_per_cubic_decimeter:
            prefix!(nano) * prefix!(micro) / 6.0_E1 / prefix!(milli); "nU/dm³",
            "nano enzyme UnitDefinition per cubic decimeter", "nano enzyme units per cubic decimeter";
        @nano_enzyme_unit_per_liter:
            prefix!(nano) * prefix!(micro) / 6.0_E1 / prefix!(milli); "nU/L",
            "nano enzyme UnitDefinition per liter", "nano enzyme units per liter";
        @pico_enzyme_unit_per_cubic_decimeter:
            prefix!(pico) * prefix!(micro) / 6.0_E1 / prefix!(milli); "pU/dm³",
            "pico enzyme UnitDefinition per cubic decimeter", "pico enzyme units per cubic decimeter";
        @pico_enzyme_unit_per_liter:
            prefix!(pico) * prefix!(micro) / 6.0_E1 / prefix!(milli); "pU/L",
            "pico enzyme UnitDefinition per liter", "pico enzyme units per liter";
        @femto_enzyme_unit_per_cubic_decimeter:
            prefix!(femto) * prefix!(micro) / 6.0_E1 / prefix!(milli); "fU/dm³",
            "femto enzyme UnitDefinition per cubic decimeter", "femto enzyme units per cubic decimeter";
        @femto_enzyme_unit_per_liter:
            prefix!(femto) * prefix!(micro) / 6.0_E1 / prefix!(milli); "fU/L",
            "femto enzyme UnitDefinition per liter", "femto enzyme units per liter";

        @kilo_enzyme_unit_per_deciliter:
            prefix!(kilo) * prefix!(micro) / 6.0_E1 / prefix!(deci) / prefix!(milli);
            "kU/dL", "kilo enzyme UnitDefinition per deciliter", "kilo enzyme units per deciliter";
        @enzyme_unit_per_deciliter:
            prefix!(none) * prefix!(micro) / 6.0_E1 / prefix!(deci) / prefix!(milli);
            "U/dL", "enzyme UnitDefinition per deciliter", "enzyme units per deciliter";
        @milli_enzyme_unit_per_deciliter:
            prefix!(milli) * prefix!(micro) / 6.0_E1 / prefix!(deci) / prefix!(milli);
            "mU/dL", "milli enzyme UnitDefinition per deciliter", "milli enzyme units per deciliter";
        @micro_enzyme_unit_per_deciliter:
            prefix!(micro) * prefix!(micro) / 6.0_E1 / prefix!(deci) / prefix!(milli);
            "μU/dL", "micro enzyme UnitDefinition per deciliter", "micro enzyme units per deciliter";
        @nano_enzyme_unit_per_deciliter:
            prefix!(nano) * prefix!(micro) / 6.0_E1 / prefix!(deci) / prefix!(milli);
            "nU/dL", "nano enzyme UnitDefinition per deciliter", "nano enzyme units per deciliter";
        @pico_enzyme_unit_per_deciliter:
            prefix!(pico) * prefix!(micro) / 6.0_E1 / prefix!(deci) / prefix!(milli);
            "pU/dL", "pico enzyme UnitDefinition per deciliter", "pico enzyme units per deciliter";
        @femto_enzyme_unit_per_deciliter:
            prefix!(femto) * prefix!(micro) / 6.0_E1 / prefix!(deci) / prefix!(milli);
            "fU/dL", "femto enzyme UnitDefinition per deciliter", "femto enzyme units per deciliter";

        @kilo_enzyme_unit_per_milliliter:
            prefix!(kilo) * prefix!(micro) / 6.0_E1 / prefix!(milli) / prefix!(milli);
            "kU/mL", "kilo enzyme UnitDefinition per milliliter", "kilo enzyme units per milliliter";
        @enzyme_unit_per_milliliter:
            prefix!(none) * prefix!(micro) / 6.0_E1 / prefix!(milli) / prefix!(milli);
            "U/mL", "enzyme UnitDefinition per milliliter", "enzyme units per milliliter";
        @milli_enzyme_unit_per_milliliter:
            prefix!(milli) * prefix!(micro) / 6.0_E1 / prefix!(milli) / prefix!(milli);
            "mU/mL", "milli enzyme UnitDefinition per milliliter", "milli enzyme units per milliliter";
        @micro_enzyme_unit_per_milliliter:
            prefix!(micro) * prefix!(micro) / 6.0_E1 / prefix!(milli) / prefix!(milli);
            "μU/mL", "micro enzyme UnitDefinition per milliliter", "micro enzyme units per milliliter";
        @nano_enzyme_unit_per_milliliter:
            prefix!(nano) * prefix!(micro) / 6.0_E1 / prefix!(milli) / prefix!(milli);
            "nU/mL", "nano enzyme UnitDefinition per milliliter", "nano enzyme units per milliliter";
        @pico_enzyme_unit_per_milliliter:
            prefix!(pico) * prefix!(micro) / 6.0_E1 / prefix!(milli) / prefix!(milli);
            "pU/mL", "pico enzyme UnitDefinition per milliliter", "pico enzyme units per milliliter";
        @femto_enzyme_unit_per_milliliter:
            prefix!(femto) * prefix!(micro) / 6.0_E1 / prefix!(milli) / prefix!(milli);
            "fU/mL", "femto enzyme UnitDefinition per milliliter", "femto enzyme units per milliliter";

        @particle_per_second_cubic_meter: 1.0_E0 / 6.022_140_76_E23; "particle · s⁻¹ · m⁻³",
            "particle per second cubic meter", "particles per second cubic meter";
    }
}

#[cfg(test)]
mod test {
    use crate::traits::Unit;
    use crate::{unit_definitions::time::TimeUnit, units::{AmountOfSubstanceUnit, CatalyticActivityUnit, LengthUnit, VolumeUnit}, units_base::UnitDefinition};

    use super::CatalyticActivityConcentrationUnit;

    #[test]
    fn check_dimension() {
        assert_eq!(CatalyticActivityConcentrationUnit::base(),  AmountOfSubstanceUnit::base() / TimeUnit::base() / LengthUnit::base().powi(3));
    }

    #[test]
    fn check_units() {
        test_unit(CatalyticActivityUnit::yottakatal, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::yottakatal_per_cubic_meter);
        test_unit(CatalyticActivityUnit::zettakatal, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::zettakatal_per_cubic_meter);
        test_unit(CatalyticActivityUnit::exakatal, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::exakatal_per_cubic_meter);
        test_unit(CatalyticActivityUnit::petakatal, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::petakatal_per_cubic_meter);
        test_unit(CatalyticActivityUnit::terakatal, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::terakatal_per_cubic_meter);
        test_unit(CatalyticActivityUnit::gigakatal, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::gigakatal_per_cubic_meter);
        test_unit(CatalyticActivityUnit::megakatal, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::megakatal_per_cubic_meter);
        test_unit(CatalyticActivityUnit::kilokatal, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::kilokatal_per_cubic_meter);
        test_unit(CatalyticActivityUnit::hectokatal, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::hectokatal_per_cubic_meter);
        test_unit(CatalyticActivityUnit::decakatal, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::decakatal_per_cubic_meter);
        test_unit(CatalyticActivityUnit::katal, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::katal_per_cubic_meter);
        test_unit(CatalyticActivityUnit::decikatal, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::decikatal_per_cubic_meter);
        test_unit(CatalyticActivityUnit::centikatal, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::centikatal_per_cubic_meter);
        test_unit(CatalyticActivityUnit::millikatal, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::millikatal_per_cubic_meter);
        test_unit(CatalyticActivityUnit::microkatal, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::microkatal_per_cubic_meter);
        test_unit(CatalyticActivityUnit::nanokatal, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::nanokatal_per_cubic_meter);
        test_unit(CatalyticActivityUnit::picokatal, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::picokatal_per_cubic_meter);
        test_unit(CatalyticActivityUnit::femtokatal, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::femtokatal_per_cubic_meter);
        test_unit(CatalyticActivityUnit::attokatal, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::attokatal_per_cubic_meter);
        test_unit(CatalyticActivityUnit::zeptokatal, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::zeptokatal_per_cubic_meter);
        test_unit(CatalyticActivityUnit::yoctokatal, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::yoctokatal_per_cubic_meter);

        test_unit(CatalyticActivityUnit::kilokatal, VolumeUnit::cubic_decimeter, CatalyticActivityConcentrationUnit::kilokatal_per_cubic_decimeter);
        test_unit(CatalyticActivityUnit::kilokatal, VolumeUnit::liter, CatalyticActivityConcentrationUnit::kilokatal_per_liter);
        test_unit(CatalyticActivityUnit::katal, VolumeUnit::cubic_decimeter, CatalyticActivityConcentrationUnit::katal_per_cubic_decimeter);
        test_unit(CatalyticActivityUnit::katal, VolumeUnit::liter, CatalyticActivityConcentrationUnit::katal_per_liter);
        test_unit(CatalyticActivityUnit::millikatal, VolumeUnit::cubic_decimeter, CatalyticActivityConcentrationUnit::millikatal_per_cubic_decimeter);
        test_unit(CatalyticActivityUnit::millikatal, VolumeUnit::liter, CatalyticActivityConcentrationUnit::millikatal_per_liter);
        test_unit(CatalyticActivityUnit::microkatal, VolumeUnit::cubic_decimeter, CatalyticActivityConcentrationUnit::microkatal_per_cubic_decimeter);
        test_unit(CatalyticActivityUnit::microkatal, VolumeUnit::liter, CatalyticActivityConcentrationUnit::microkatal_per_liter);
        test_unit(CatalyticActivityUnit::nanokatal, VolumeUnit::cubic_decimeter, CatalyticActivityConcentrationUnit::nanokatal_per_cubic_decimeter);
        test_unit(CatalyticActivityUnit::nanokatal, VolumeUnit::liter, CatalyticActivityConcentrationUnit::nanokatal_per_liter);
        test_unit(CatalyticActivityUnit::picokatal, VolumeUnit::cubic_decimeter, CatalyticActivityConcentrationUnit::picokatal_per_cubic_decimeter);
        test_unit(CatalyticActivityUnit::picokatal, VolumeUnit::liter, CatalyticActivityConcentrationUnit::picokatal_per_liter);
        test_unit(CatalyticActivityUnit::femtokatal, VolumeUnit::cubic_decimeter, CatalyticActivityConcentrationUnit::femtokatal_per_cubic_decimeter);
        test_unit(CatalyticActivityUnit::femtokatal, VolumeUnit::liter, CatalyticActivityConcentrationUnit::femtokatal_per_liter);

        test_unit(CatalyticActivityUnit::kilokatal, VolumeUnit::deciliter, CatalyticActivityConcentrationUnit::kilokatal_per_deciliter);
        test_unit(CatalyticActivityUnit::katal, VolumeUnit::deciliter, CatalyticActivityConcentrationUnit::katal_per_deciliter);
        test_unit(CatalyticActivityUnit::millikatal, VolumeUnit::deciliter, CatalyticActivityConcentrationUnit::millikatal_per_deciliter);
        test_unit(CatalyticActivityUnit::microkatal, VolumeUnit::deciliter, CatalyticActivityConcentrationUnit::microkatal_per_deciliter);
        test_unit(CatalyticActivityUnit::nanokatal, VolumeUnit::deciliter, CatalyticActivityConcentrationUnit::nanokatal_per_deciliter);
        test_unit(CatalyticActivityUnit::picokatal, VolumeUnit::deciliter, CatalyticActivityConcentrationUnit::picokatal_per_deciliter);
        test_unit(CatalyticActivityUnit::femtokatal, VolumeUnit::deciliter, CatalyticActivityConcentrationUnit::femtokatal_per_deciliter);

        test_unit(CatalyticActivityUnit::kilokatal, VolumeUnit::milliliter, CatalyticActivityConcentrationUnit::kilokatal_per_milliliter);
        test_unit(CatalyticActivityUnit::katal, VolumeUnit::milliliter, CatalyticActivityConcentrationUnit::katal_per_milliliter);
        test_unit(CatalyticActivityUnit::millikatal, VolumeUnit::milliliter, CatalyticActivityConcentrationUnit::millikatal_per_milliliter);
        test_unit(CatalyticActivityUnit::microkatal, VolumeUnit::milliliter, CatalyticActivityConcentrationUnit::microkatal_per_milliliter);
        test_unit(CatalyticActivityUnit::nanokatal, VolumeUnit::milliliter, CatalyticActivityConcentrationUnit::nanokatal_per_milliliter);
        test_unit(CatalyticActivityUnit::picokatal, VolumeUnit::milliliter, CatalyticActivityConcentrationUnit::picokatal_per_milliliter);
        test_unit(CatalyticActivityUnit::femtokatal, VolumeUnit::milliliter, CatalyticActivityConcentrationUnit::femtokatal_per_milliliter);

        test_unit(CatalyticActivityUnit::yotta_enzyme_unit, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::yotta_enzyme_unit_per_cubic_meter);
        test_unit(CatalyticActivityUnit::zetta_enzyme_unit, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::zetta_enzyme_unit_per_cubic_meter);
        test_unit(CatalyticActivityUnit::exa_enzyme_unit, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::exa_enzyme_unit_per_cubic_meter);
        test_unit(CatalyticActivityUnit::peta_enzyme_unit, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::peta_enzyme_unit_per_cubic_meter);
        test_unit(CatalyticActivityUnit::tera_enzyme_unit, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::tera_enzyme_unit_per_cubic_meter);
        test_unit(CatalyticActivityUnit::giga_enzyme_unit, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::giga_enzyme_unit_per_cubic_meter);
        test_unit(CatalyticActivityUnit::mega_enzyme_unit, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::mega_enzyme_unit_per_cubic_meter);
        test_unit(CatalyticActivityUnit::kilo_enzyme_unit, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::kilo_enzyme_unit_per_cubic_meter);
        test_unit(CatalyticActivityUnit::hecto_enzyme_unit, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::hecto_enzyme_unit_per_cubic_meter);
        test_unit(CatalyticActivityUnit::deca_enzyme_unit, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::deca_enzyme_unit_per_cubic_meter);
        test_unit(CatalyticActivityUnit::enzyme_unit, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::enzyme_unit_per_cubic_meter);
        test_unit(CatalyticActivityUnit::deci_enzyme_unit, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::deci_enzyme_unit_per_cubic_meter);
        test_unit(CatalyticActivityUnit::centi_enzyme_unit, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::centi_enzyme_unit_per_cubic_meter);
        test_unit(CatalyticActivityUnit::milli_enzyme_unit, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::milli_enzyme_unit_per_cubic_meter);
        test_unit(CatalyticActivityUnit::micro_enzyme_unit, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::micro_enzyme_unit_per_cubic_meter);
        test_unit(CatalyticActivityUnit::nano_enzyme_unit, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::nano_enzyme_unit_per_cubic_meter);
        test_unit(CatalyticActivityUnit::pico_enzyme_unit, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::pico_enzyme_unit_per_cubic_meter);
        test_unit(CatalyticActivityUnit::femto_enzyme_unit, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::femto_enzyme_unit_per_cubic_meter);
        test_unit(CatalyticActivityUnit::atto_enzyme_unit, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::atto_enzyme_unit_per_cubic_meter);
        test_unit(CatalyticActivityUnit::zepto_enzyme_unit, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::zepto_enzyme_unit_per_cubic_meter);
        test_unit(CatalyticActivityUnit::yocto_enzyme_unit, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::yocto_enzyme_unit_per_cubic_meter);

        test_unit(CatalyticActivityUnit::kilo_enzyme_unit, VolumeUnit::cubic_decimeter,
            CatalyticActivityConcentrationUnit::kilo_enzyme_unit_per_cubic_decimeter);
        test_unit(CatalyticActivityUnit::kilo_enzyme_unit, VolumeUnit::liter, CatalyticActivityConcentrationUnit::kilo_enzyme_unit_per_liter);
        test_unit(CatalyticActivityUnit::enzyme_unit, VolumeUnit::cubic_decimeter,
            CatalyticActivityConcentrationUnit::enzyme_unit_per_cubic_decimeter);
        test_unit(CatalyticActivityUnit::enzyme_unit, VolumeUnit::liter, CatalyticActivityConcentrationUnit::enzyme_unit_per_liter);
        test_unit(CatalyticActivityUnit::milli_enzyme_unit, VolumeUnit::cubic_decimeter,
            CatalyticActivityConcentrationUnit::milli_enzyme_unit_per_cubic_decimeter);
        test_unit(CatalyticActivityUnit::milli_enzyme_unit, VolumeUnit::liter, CatalyticActivityConcentrationUnit::milli_enzyme_unit_per_liter);
        test_unit(CatalyticActivityUnit::micro_enzyme_unit, VolumeUnit::cubic_decimeter,
            CatalyticActivityConcentrationUnit::micro_enzyme_unit_per_cubic_decimeter);
        test_unit(CatalyticActivityUnit::micro_enzyme_unit, VolumeUnit::liter, CatalyticActivityConcentrationUnit::micro_enzyme_unit_per_liter);
        test_unit(CatalyticActivityUnit::nano_enzyme_unit, VolumeUnit::cubic_decimeter,
            CatalyticActivityConcentrationUnit::nano_enzyme_unit_per_cubic_decimeter);
        test_unit(CatalyticActivityUnit::nano_enzyme_unit, VolumeUnit::liter, CatalyticActivityConcentrationUnit::nano_enzyme_unit_per_liter);
        test_unit(CatalyticActivityUnit::pico_enzyme_unit, VolumeUnit::cubic_decimeter,
            CatalyticActivityConcentrationUnit::pico_enzyme_unit_per_cubic_decimeter);
        test_unit(CatalyticActivityUnit::pico_enzyme_unit, VolumeUnit::liter, CatalyticActivityConcentrationUnit::pico_enzyme_unit_per_liter);
        test_unit(CatalyticActivityUnit::femto_enzyme_unit, VolumeUnit::cubic_decimeter,
            CatalyticActivityConcentrationUnit::femto_enzyme_unit_per_cubic_decimeter);
        test_unit(CatalyticActivityUnit::femto_enzyme_unit, VolumeUnit::liter, CatalyticActivityConcentrationUnit::femto_enzyme_unit_per_liter);

        test_unit(CatalyticActivityUnit::kilo_enzyme_unit, VolumeUnit::deciliter, CatalyticActivityConcentrationUnit::kilo_enzyme_unit_per_deciliter);
        test_unit(CatalyticActivityUnit::enzyme_unit, VolumeUnit::deciliter, CatalyticActivityConcentrationUnit::enzyme_unit_per_deciliter);
        test_unit(CatalyticActivityUnit::milli_enzyme_unit, VolumeUnit::deciliter, CatalyticActivityConcentrationUnit::milli_enzyme_unit_per_deciliter);
        test_unit(CatalyticActivityUnit::micro_enzyme_unit, VolumeUnit::deciliter, CatalyticActivityConcentrationUnit::micro_enzyme_unit_per_deciliter);
        test_unit(CatalyticActivityUnit::nano_enzyme_unit, VolumeUnit::deciliter, CatalyticActivityConcentrationUnit::nano_enzyme_unit_per_deciliter);
        test_unit(CatalyticActivityUnit::pico_enzyme_unit, VolumeUnit::deciliter, CatalyticActivityConcentrationUnit::pico_enzyme_unit_per_deciliter);
        test_unit(CatalyticActivityUnit::femto_enzyme_unit, VolumeUnit::deciliter, CatalyticActivityConcentrationUnit::femto_enzyme_unit_per_deciliter);

        test_unit(CatalyticActivityUnit::kilo_enzyme_unit, VolumeUnit::milliliter, CatalyticActivityConcentrationUnit::kilo_enzyme_unit_per_milliliter);
        test_unit(CatalyticActivityUnit::enzyme_unit, VolumeUnit::milliliter, CatalyticActivityConcentrationUnit::enzyme_unit_per_milliliter);
        test_unit(CatalyticActivityUnit::milli_enzyme_unit, VolumeUnit::milliliter, CatalyticActivityConcentrationUnit::milli_enzyme_unit_per_milliliter);
        test_unit(CatalyticActivityUnit::micro_enzyme_unit, VolumeUnit::milliliter, CatalyticActivityConcentrationUnit::micro_enzyme_unit_per_milliliter);
        test_unit(CatalyticActivityUnit::nano_enzyme_unit, VolumeUnit::milliliter, CatalyticActivityConcentrationUnit::nano_enzyme_unit_per_milliliter);
        test_unit(CatalyticActivityUnit::pico_enzyme_unit, VolumeUnit::milliliter, CatalyticActivityConcentrationUnit::pico_enzyme_unit_per_milliliter);
        test_unit(CatalyticActivityUnit::femto_enzyme_unit, VolumeUnit::milliliter, CatalyticActivityConcentrationUnit::femto_enzyme_unit_per_milliliter);

        test_unit(CatalyticActivityUnit::particle_per_second, VolumeUnit::cubic_meter, CatalyticActivityConcentrationUnit::particle_per_second_cubic_meter);
    }
    fn test_unit(activity: CatalyticActivityUnit, volume: VolumeUnit, value: CatalyticActivityConcentrationUnit) {
        assert!(Into::<UnitDefinition>::into(value).approx_eq(Into::<UnitDefinition>::into(activity) / Into::<UnitDefinition>::into(volume), 1e-12));
    }
}
