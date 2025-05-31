//! Electric charge (base UnitDefinition coulomb, A · s).
use crate::{prefix, quantity};
quantity! {
    /// Electric charge (base UnitDefinition coulomb, A · s).
    quantity: ElectricCharge; "electric charge";
    /// Dimension of electric charge, TI (base UnitDefinition coulomb, A · s).
    dimension: ISQ[
        0.0,     // length
        0.0,     // mass
        1.0,     // time
        1.0,     // electric current
        0.0,     // thermodynamic temperature
        0.0,     // amount of substance
        0.0];    // luminous intensity
    units {
        @yottacoulomb: prefix!(yotta); "YC", "yottacoulomb", "yottacoulombs";
        @zettacoulomb: prefix!(zetta); "ZC", "zettacoulomb", "zettacoulombs";
        @exacoulomb: prefix!(exa); "EC", "exacoulomb", "exacoulombs";
        @petacoulomb: prefix!(peta); "PC", "petacoulomb", "petacoulombs";
        @teracoulomb: prefix!(tera); "TC", "teracoulomb", "teracoulombs";
        @gigacoulomb: prefix!(giga); "GC", "gigacoulomb", "gigacoulombs";
        @megacoulomb: prefix!(mega); "MC", "megacoulomb", "megacoulombs";
        @kilocoulomb: prefix!(kilo); "kC", "kilocoulomb", "kilocoulombs";
        @hectocoulomb: prefix!(hecto); "hC", "hectocoulomb", "hectocoulombs";
        @decacoulomb: prefix!(deca); "daC", "decacoulomb", "decacoulombs";
        /// Derived UnitDefinition of electric charge.
        @coulomb: prefix!(none); "C", "coulomb", "coulombs";
        @decicoulomb: prefix!(deci); "dC", "decicoulomb", "decicoulombs";
        @centicoulomb: prefix!(centi); "cC", "centicoulomb", "centicoulombs";
        @millicoulomb: prefix!(milli); "mC", "millcoulomb", "millcoulombs";
        @microcoulomb: prefix!(micro); "µC", "microcoulomb", "microcoulombs";
        @nanocoulomb: prefix!(nano); "nC", "nanocoulomb", "nanocoulombs";
        @picocoulomb: prefix!(pico); "pC", "picocoulomb", "picocoulombs";
        @femtocoulomb: prefix!(femto); "fC", "femtocoulomb", "femtocoulombs";
        @attocoulomb: prefix!(atto); "aC", "attocoulomb", "attocoulombs";
        @zeptocoulomb: prefix!(zepto); "zC", "zeptocoulomb", "zeptocoulombs";
        @yoctocoulomb: prefix!(yocto); "yC", "yoctocoulomb", "yoctocoulombs";

        @petaampere_hour: 3.6_E18; "PA · h", "petaampere hour", "petaampere hours";
        @teraampere_hour: 3.6_E15; "TA · h", "teraampere hour", "teraampere hours";
        @gigaampere_hour: 3.6_E12; "GA · h", "gigaampere hour", "gigaampere hours";
        @megaampere_hour: 3.6_E9; "MA · h", "megaampere hour", "megaampere hours";
        @kiloampere_hour: 3.6_E6; "kA · h", "kiloampere hour", "kiloampere hours";
        @hectoampere_hour: 3.6_E5; "hA · h", "hectoampere hour", "hectoampere hours";
        @decaampere_hour: 3.6_E4; "daA · h", "decaampere hour", "decaampere hours";
        @ampere_hour: 3.6_E3; "A · h", "ampere hour", "ampere hours";
        @milliampere_hour: 3.6_E0; "mA · h", "milliampere hour", "milliampere hours";
        @microampere_hour: 3.6_E-3; "µA · h", "microampere hour", "microampere hours";

        /// Electric charge carried by a single proton.
        @elementary_charge: 1.602_176_634_E-19; "e", "elementary charge", "elementary charges";
        @atomic_unit_of_charge: 1.602_176_634_E-19; "a.u. of charge", "atomic UnitDefinition of charge",
            "atomic units of charge";
        @abcoulomb: 1.0_E1; "abC", "abcoulomb", "abcoulombs";
        @faraday: 9.648_531_E4; "F", "faraday", "faradays";
        @franklin: 3.335_641_E-10; "Fr", "franklin", "franklins";
        @statcoulomb: 3.33564095198152E-10; "statC", "statcoulomb", "statcoulombs";
    }
}

#[cfg(test)]
mod tests {
use crate::traits::Unit;
    use crate::{unit_definitions::{electric_charge::ElectricChargeUnit, electric_current::ElectricCurrentUnit, time::TimeUnit}, units_base::UnitDefinition};
    
    #[test]
    fn check_dimension() {
        assert_eq!(ElectricChargeUnit::base(), ElectricCurrentUnit::base() * TimeUnit::base());
    }

    #[test]
    fn check_units() {
        test_unit(ElectricCurrentUnit::yottaampere, TimeUnit::second, ElectricChargeUnit::yottacoulomb);
        test_unit(ElectricCurrentUnit::zettaampere, TimeUnit::second, ElectricChargeUnit::zettacoulomb);
        test_unit(ElectricCurrentUnit::exaampere, TimeUnit::second, ElectricChargeUnit::exacoulomb);
        test_unit(ElectricCurrentUnit::petaampere, TimeUnit::second, ElectricChargeUnit::petacoulomb);
        test_unit(ElectricCurrentUnit::teraampere, TimeUnit::second, ElectricChargeUnit::teracoulomb);
        test_unit(ElectricCurrentUnit::gigaampere, TimeUnit::second, ElectricChargeUnit::gigacoulomb);
        test_unit(ElectricCurrentUnit::megaampere, TimeUnit::second, ElectricChargeUnit::megacoulomb);
        test_unit(ElectricCurrentUnit::kiloampere, TimeUnit::second, ElectricChargeUnit::kilocoulomb);
        test_unit(ElectricCurrentUnit::hectoampere, TimeUnit::second, ElectricChargeUnit::hectocoulomb);
        test_unit(ElectricCurrentUnit::decaampere, TimeUnit::second, ElectricChargeUnit::decacoulomb);
        test_unit(ElectricCurrentUnit::ampere, TimeUnit::second, ElectricChargeUnit::coulomb);
        test_unit(ElectricCurrentUnit::deciampere, TimeUnit::second, ElectricChargeUnit::decicoulomb);
        test_unit(ElectricCurrentUnit::centiampere, TimeUnit::second, ElectricChargeUnit::centicoulomb);
        test_unit(ElectricCurrentUnit::milliampere, TimeUnit::second, ElectricChargeUnit::millicoulomb);
        test_unit(ElectricCurrentUnit::microampere, TimeUnit::second, ElectricChargeUnit::microcoulomb);
        test_unit(ElectricCurrentUnit::nanoampere, TimeUnit::second, ElectricChargeUnit::nanocoulomb);
        test_unit(ElectricCurrentUnit::picoampere, TimeUnit::second, ElectricChargeUnit::picocoulomb);
        test_unit(ElectricCurrentUnit::femtoampere, TimeUnit::second, ElectricChargeUnit::femtocoulomb);
        test_unit(ElectricCurrentUnit::attoampere, TimeUnit::second, ElectricChargeUnit::attocoulomb);
        test_unit(ElectricCurrentUnit::zeptoampere, TimeUnit::second, ElectricChargeUnit::zeptocoulomb);
        test_unit(ElectricCurrentUnit::yoctoampere, TimeUnit::second, ElectricChargeUnit::yoctocoulomb);

        test_unit(ElectricCurrentUnit::elementary_charge_per_second, TimeUnit::second, ElectricChargeUnit::elementary_charge);
        test_unit(ElectricCurrentUnit::atomic_unit_of_charge_per_second, TimeUnit::second, ElectricChargeUnit::atomic_unit_of_charge);
        test_unit(ElectricCurrentUnit::ampere, TimeUnit::hour, ElectricChargeUnit::ampere_hour);
        test_unit(ElectricCurrentUnit::abampere, TimeUnit::second, ElectricChargeUnit::abcoulomb);
        test_unit(ElectricCurrentUnit::elementary_charge_per_second, TimeUnit::second, ElectricChargeUnit::elementary_charge);
        test_unit(ElectricCurrentUnit::statampere, TimeUnit::second, ElectricChargeUnit::statcoulomb);        
    }
    fn test_unit(current: ElectricCurrentUnit, time: TimeUnit, value: ElectricChargeUnit)
    {
        assert_eq!(Into::<UnitDefinition>::into(value), (Into::<UnitDefinition>::into(current) * Into::<UnitDefinition>::into(time)));
    }
}
