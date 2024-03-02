//! Capacitance (base unit farad, m⁻² · kg⁻¹ · s⁴ · A²).
use crate::{prefix, quantity};
quantity! {
    /// Capacitance (base unit farad, m⁻² · kg⁻¹ · s⁴ · A²).
    quantity: Capacitance; "capacitance";
    /// Dimension of capacitance, L⁻²M⁻¹T⁴I² (base unit farad, m⁻² · kg⁻¹ · s⁴ · A²).
    dimension: ISQ<
        N2,     // length
        N1,     // mass
        P4,     // time
        P2,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottafarad: prefix!(yotta); "YF", "yottafarad", "yottafarads";
        @zettafarad: prefix!(zetta); "ZF", "zettafarad", "zettafarads";
        @exafarad: prefix!(exa); "EF", "exafarad", "exafarads";
        @petafarad: prefix!(peta); "PF", "petafarad", "petafarads";
        @terafarad: prefix!(tera); "TF", "terafarad", "terafarads";
        @gigafarad: prefix!(giga); "GF", "gigafarad", "gigafarads";
        @megafarad: prefix!(mega); "MF", "megafarad", "megafarads";
        @kilofarad: prefix!(kilo); "kF", "kilofarad", "kilofarads";
        @hectofarad: prefix!(hecto); "hF", "hectofarad", "hectofarads";
        @decafarad: prefix!(deca); "daF", "decafarad", "decafarads";
        /// Derived unit of capacitance.
        @farad: prefix!(none); "F", "farad", "farads";
        @decifarad: prefix!(deci); "dF", "decifarad", "decifarads";
        @centifarad: prefix!(centi); "cF", "centifarad", "centifarads";
        @millifarad: prefix!(milli); "mF", "millifarad", "millifarads";
        @microfarad: prefix!(micro); "µF", "microfarad", "microfarads";
        @nanofarad: prefix!(nano); "nF", "nanofarad", "nanofarads";
        @picofarad: prefix!(pico); "pF", "picofarad", "picofarads";
        @femtofarad: prefix!(femto); "fF", "femtofarad", "femtofarads";
        @attofarad: prefix!(atto); "aF", "attofarad", "attofarads";
        @zeptofarad: prefix!(zepto); "zF", "zeptofarad", "zeptofarads";
        @yoctofarad: prefix!(yocto); "yF", "yoctofarad", "yoctofarads";

        @abfarad: 1.0_E9; "abF", "abfarad", "abfarads";
        @statfarad: 1.11265005605362E-12; "statF", "statfarad", "statfarads";
    }
}

#[cfg(test)]
mod tests {
    use crate::{unit_definitions::{capacitance::CapacitanceUnit, electric_current::ElectricCurrentUnit, electric_potential::ElectricPotentialUnit, time::TimeUnit}, units_base::Unit, ElectricPotential};


    #[test]
    fn check_dimension() {
        assert_eq!(CapacitanceUnit::unit(),  ElectricCurrentUnit::unit() * TimeUnit::unit() / ElectricPotentialUnit::unit());       
    }

    #[test]
    fn check_units() {
       test_unit(ElectricCurrentUnit::yottaampere, ElectricPotentialUnit::volt, CapacitanceUnit::yottafarad);
       test_unit(ElectricCurrentUnit::zettaampere, ElectricPotentialUnit::volt, CapacitanceUnit::zettafarad);
       test_unit(ElectricCurrentUnit::exaampere, ElectricPotentialUnit::volt, CapacitanceUnit::exafarad);
       test_unit(ElectricCurrentUnit::petaampere, ElectricPotentialUnit::volt, CapacitanceUnit::petafarad);
       test_unit(ElectricCurrentUnit::teraampere, ElectricPotentialUnit::volt, CapacitanceUnit::terafarad);
       test_unit(ElectricCurrentUnit::gigaampere, ElectricPotentialUnit::volt, CapacitanceUnit::gigafarad);
       test_unit(ElectricCurrentUnit::megaampere, ElectricPotentialUnit::volt, CapacitanceUnit::megafarad);
       test_unit(ElectricCurrentUnit::kiloampere, ElectricPotentialUnit::volt, CapacitanceUnit::kilofarad);
       test_unit(ElectricCurrentUnit::hectoampere, ElectricPotentialUnit::volt, CapacitanceUnit::hectofarad);
       test_unit(ElectricCurrentUnit::decaampere, ElectricPotentialUnit::volt, CapacitanceUnit::decafarad);
       test_unit(ElectricCurrentUnit::ampere, ElectricPotentialUnit::volt, CapacitanceUnit::farad);
       test_unit(ElectricCurrentUnit::deciampere, ElectricPotentialUnit::volt, CapacitanceUnit::decifarad);
       test_unit(ElectricCurrentUnit::centiampere, ElectricPotentialUnit::volt, CapacitanceUnit::centifarad);
       test_unit(ElectricCurrentUnit::milliampere, ElectricPotentialUnit::volt, CapacitanceUnit::millifarad);
       test_unit(ElectricCurrentUnit::microampere, ElectricPotentialUnit::volt, CapacitanceUnit::microfarad);
       test_unit(ElectricCurrentUnit::nanoampere, ElectricPotentialUnit::volt, CapacitanceUnit::nanofarad);
       test_unit(ElectricCurrentUnit::picoampere, ElectricPotentialUnit::volt, CapacitanceUnit::picofarad);
       test_unit(ElectricCurrentUnit::femtoampere, ElectricPotentialUnit::volt, CapacitanceUnit::femtofarad);
       test_unit(ElectricCurrentUnit::attoampere, ElectricPotentialUnit::volt, CapacitanceUnit::attofarad);
       test_unit(ElectricCurrentUnit::zeptoampere, ElectricPotentialUnit::volt, CapacitanceUnit::zeptofarad);
       test_unit(ElectricCurrentUnit::yoctoampere, ElectricPotentialUnit::volt, CapacitanceUnit::yoctofarad);

       test_unit(ElectricCurrentUnit::statampere, ElectricPotentialUnit::statvolt, CapacitanceUnit::statfarad);
       test_unit(ElectricCurrentUnit::abampere, ElectricPotentialUnit::abvolt, CapacitanceUnit::abfarad);

        fn test_unit(current: ElectricCurrentUnit, potential: ElectricPotentialUnit, value: CapacitanceUnit) {
            println!("{value}: left: {:?}, right: {:?}", Into::<Unit>::into(value), Into::<Unit>::into(current) / Into::<Unit>::into(potential) * Into::<Unit>::into(TimeUnit::second));
            assert!(Into::<Unit>::into(value).approx_eq(Into::<Unit>::into(current) / Into::<Unit>::into(potential) * Into::<Unit>::into(TimeUnit::second), 1e-12));
        }
    }
}

