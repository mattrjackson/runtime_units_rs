//! Electric potential (base unit volt, m² · kg · s⁻³ · A⁻¹).
use crate::{prefix, quantity};
quantity! {
    /// Electric potential (base unit volt, m² · kg · s⁻³ · A⁻¹).
    quantity: ElectricPotential; "electric potential";
    /// Dimension of electric potential, L²MT⁻³I⁻¹ (base unit volt, m² · kg · s⁻³ · A⁻¹).
    dimension: ISQ<
        P2,     // length
        P1,     // mass
        N3,     // time
        N1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottavolt: prefix!(yotta); "YV", "yottavolt", "yottavolts";
        @zettavolt: prefix!(zetta); "ZV", "zettavolt", "zettavolts";
        @exavolt: prefix!(exa); "EV", "exavolt", "exavolts";
        @petavolt: prefix!(peta); "PV", "petavolt", "petavolts";
        @teravolt: prefix!(tera); "TV", "teravolt", "teravolts";
        @gigavolt: prefix!(giga); "GV", "gigavolt", "gigavolts";
        @megavolt: prefix!(mega); "MV", "megavolt", "megavolts";
        @kilovolt: prefix!(kilo); "kV", "kilovolt", "kilovolts";
        @hectovolt: prefix!(hecto); "hV", "hectovolt", "hectovolts";
        @decavolt: prefix!(deca); "daV", "decavolt", "decavolts";
        /// Derived unit of electric potential.
        @volt: prefix!(none); "V", "volt", "volts";
        @decivolt: prefix!(deci); "dV", "decivolt", "decivolts";
        @centivolt: prefix!(centi); "cV", "centivolt", "centivolts";
        @millivolt: prefix!(milli); "mV", "millivolt", "millivolts";
        @microvolt: prefix!(micro); "µV", "microvolt", "microvolts";
        @nanovolt: prefix!(nano); "nV", "nanovolt", "nanovolts";
        @picovolt: prefix!(pico); "pV", "picovolt", "picovolts";
        @femtovolt: prefix!(femto); "fV", "femtovolt", "femtovolts";
        @attovolt: prefix!(atto); "aV", "attovolt", "attovolts";
        @zeptovolt: prefix!(zepto); "zV", "zeptovolt", "zeptovolts";
        @yoctovolt: prefix!(yocto); "yV", "yoctovolt", "yoctovolts";

        @abvolt: 1.0_E-8; "abV", "abvolt", "abvolts";
        @statvolt: 2.99792458E+02; "statV", "statvolt", "statvolts";
    }
}

#[cfg(test)]
mod tests {
    use crate::{unit_definitions::{electric_current::ElectricCurrentUnit, electric_potential::ElectricPotentialUnit, time::TimeUnit}, units::{LengthUnit, MassUnit}, units_base::Unit};
    
    #[test]
    fn check_dimension() {
        assert_eq!(ElectricPotentialUnit::unit(),  LengthUnit::unit().powi(2) * MassUnit::unit() / ElectricCurrentUnit::unit() / TimeUnit::unit().powi(3));
    }

    #[test]
    fn check_units() {
        test_unit(ElectricCurrentUnit::yottaampere, ElectricPotentialUnit::yoctovolt);
        test_unit(ElectricCurrentUnit::zettaampere, ElectricPotentialUnit::zeptovolt);
        test_unit(ElectricCurrentUnit::exaampere, ElectricPotentialUnit::attovolt);
        test_unit(ElectricCurrentUnit::petaampere, ElectricPotentialUnit::femtovolt);
        test_unit(ElectricCurrentUnit::teraampere, ElectricPotentialUnit::picovolt);
        test_unit(ElectricCurrentUnit::gigaampere, ElectricPotentialUnit::nanovolt);
        test_unit(ElectricCurrentUnit::megaampere, ElectricPotentialUnit::microvolt);
        test_unit(ElectricCurrentUnit::kiloampere, ElectricPotentialUnit::millivolt);
        test_unit(ElectricCurrentUnit::hectoampere, ElectricPotentialUnit::centivolt);
        test_unit(ElectricCurrentUnit::decaampere, ElectricPotentialUnit::decivolt);
        test_unit(ElectricCurrentUnit::ampere, ElectricPotentialUnit::volt);
        test_unit(ElectricCurrentUnit::deciampere, ElectricPotentialUnit::decavolt);
        test_unit(ElectricCurrentUnit::centiampere, ElectricPotentialUnit::hectovolt);
        test_unit(ElectricCurrentUnit::milliampere, ElectricPotentialUnit::kilovolt);
        test_unit(ElectricCurrentUnit::microampere, ElectricPotentialUnit::megavolt);
        test_unit(ElectricCurrentUnit::nanoampere, ElectricPotentialUnit::gigavolt);
        test_unit(ElectricCurrentUnit::picoampere, ElectricPotentialUnit::teravolt);
        test_unit(ElectricCurrentUnit::femtoampere, ElectricPotentialUnit::petavolt);
        test_unit(ElectricCurrentUnit::attoampere, ElectricPotentialUnit::exavolt);
        test_unit(ElectricCurrentUnit::zeptoampere, ElectricPotentialUnit::zettavolt);
        test_unit(ElectricCurrentUnit::yoctoampere, ElectricPotentialUnit::yottavolt);

        fn test_unit(current: ElectricCurrentUnit, value: ElectricPotentialUnit) {
            assert!(Into::<Unit>::into(value).approx_eq(Into::<Unit>::into(LengthUnit::meter).powi(2) * MassUnit::kilogram.into() / Into::<Unit>::into(TimeUnit::second).powi(3) /  Into::<Unit>::into(current), 1e-12));
        }
    }
    
}
