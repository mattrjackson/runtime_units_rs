//! Amount of substance (base UnitDefinition mole, mol).
use crate::{prefix, quantity};
quantity! {
    /// Amount of substance (base UnitDefinition mole, mol).
    quantity: AmountOfSubstance; "amount of substance";
    /// Dimension of amount of substance, N (base UnitDefinition mole, mol).
    dimension: ISQ[
        0.0,     // length
        0.0,     // mass
        0.0,     // time
        0.0,     // electric current
        0.0,     // thermodynamic temperature
        1.0,     // amount of substance
        0.0];    // luminous intensity
    units {
        @yottamole: prefix!(yotta); "Ymol", "yottamole", "yottamoles";
        @zettamole: prefix!(zetta); "Zmol", "zettamole", "zettamoles";
        @examole: prefix!(exa); "Emol", "examole", "examoles";
        @petamole: prefix!(peta); "Pmol", "petamole", "petamoles";
        @teramole: prefix!(tera); "Tmol", "teramole", "teramoles";
        @gigamole: prefix!(giga); "Gmol", "gigamole", "gigamoles";
        @megamole: prefix!(mega); "Mmol", "megamole", "megamoles";
        @kilomole: prefix!(kilo); "kmol", "kilomole", "kilomoles";
        @hectomole: prefix!(hecto); "hmol", "hectomole", "hectomoles";
        @decamole: prefix!(deca); "damol", "decamole", "decamoles";
        /// 1. The mole is the SI UnitDefinition of amount of substance. One mole contains exactly
        ///    6.022 140 76 × 10²³ elementary entities. This number is the fixed numerical value of
        ///    the Avogadro constant, *N*<sub>A</sub>, when expressed in the UnitDefinition mol⁻¹ and is
        ///    called the Avogadro number.
        /// 2. The amount of substance, symbol *n*, of a system is a measure of the number of
        ///    specified elementary entities. An elementary entity may be an atom, a molecule, an
        ///    ion, an electron, any other particle or specified group of particles.
        @mole: prefix!(none); "mol", "mole", "moles";
        @decimole: prefix!(deci); "dmol", "decimole", "decimoles";
        @centimole: prefix!(centi); "cmol", "centimole", "centimoles";
        @millimole: prefix!(milli); "mmol", "millimole", "millimoles";
        @micromole: prefix!(micro); "µmol", "micromole", "micromoles";
        @nanomole: prefix!(nano); "nmol", "nanomole", "nanomoles";
        @picomole: prefix!(pico); "pmol", "picomole", "picomoles";
        @femtomole: prefix!(femto); "fmol", "femtomole", "femtomoles";
        @attomole: prefix!(atto); "amol", "attomole", "attomoles";
        @zeptomole: prefix!(zepto); "zmol", "zeptomole", "zeptomoles";
        @yoctomole: prefix!(yocto); "ymol", "yoctomole", "yoctomoles";

        /// One elementary entity may be an atom, a molecule, an ion, an electron, any other
        /// particle or specified group of particles.
        @particle: 1.0_E0 / 6.022_140_76_E23; "particle", "particle", "particles";
        /// Amount of ideal gas contained in a volume of cubic meter at standard temperature (O°C) and pressure (1 bar)
        @standard_cubic_meter: 1_E5 * prefix!(none) / 8.314_462_618 / 273.15; "m³(STP)",
            "standard cubic meter", "standard cubic mols";
        /// Amount of ideal gas contained in a volume of liter at standard temperature (O°C) and pressure (1 bar)
        @standard_liter: 1_E5 * prefix!(milli) / 8.314_462_618 / 273.15; "L(STP)",
            "standard liter", "standard liters";
        /// Amount of ideal gas contained in a volume of cubic centimeter at standard temperature (O°C) and pressure (1 bar)
        @standard_centimeter: 1_E5 * prefix!(micro) / 8.314_462_618 / 273.15; "cm³(STP)",
            "standard cubic centimeter", "standard cubic centimols";
        /// Amount of ideal gas contained in a volume of cubic foot at standard temperature (O°C) and pressure (1 bar)
        @standard_cubic_foot: 1_E5 * 2.831_685_E-2 / 8.314_462_618 / 273.15; "scf",
            "standard cubic foot", "standard cubic feet";
    }
}