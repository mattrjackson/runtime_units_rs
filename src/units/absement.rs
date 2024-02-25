//! Absement (base unit meter second, m · s).
use crate::{prefix, quantity};
quantity! {
    /// Absement (base unit meter second, m · s).
    quantity: Absement; "absement";
    /// Dimension of absement, LT (base unit meter second, m · s).
    dimension: ISQ<
        P1,     // length
        Z0,     // mass
        P1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottameter_second: prefix!(yotta); "Ym · s", "yottameter second",
            "yottameter seconds";
        @zettameter_second: prefix!(zetta); "Zm · s", "zettameter second",
            "zettameter seconds";
        @exameter_second: prefix!(exa); "Em · s", "exameter second",
            "exameter seconds";
        @petameter_second: prefix!(peta); "Pm · s", "petameter second",
            "petameter seconds";
        @terameter_second: prefix!(tera); "Tm · s", "terameter second",
            "terameter seconds";
        @gigameter_second: prefix!(giga); "Gm · s", "gigameter second",
            "gigameter seconds";
        @megameter_second: prefix!(mega); "Mm · s", "megameter second",
            "megameter seconds";
        @kilometer_second: prefix!(kilo); "km · s", "kilometer second",
            "kilometer seconds";
        @hectometer_second: prefix!(hecto); "hm · s", "hectometer second",
            "hectometer seconds";
        @decameter_second: prefix!(deca); "dam · s", "decameter second",
            "decameter seconds";
        @meter_second: prefix!(none); "m · s", "meter second",
            "meter seconds";
        @decimeter_second: prefix!(deci); "dm · s", "decimeter second",
            "decimeter seconds";
        @centimeter_second: prefix!(centi); "cm · s", "centimeter second",
            "centimeter seconds";
        @millimeter_second: prefix!(milli); "mm · s", "millimeter second",
            "millimeter seconds";
        @micrometer_second: prefix!(micro); "µm · s", "micrometer second",
            "micrometer seconds";
        @nanometer_second: prefix!(nano); "nm · s", "nanometer second",
            "nanometer seconds";
        @picometer_second: prefix!(pico); "pm · s", "picometer second",
            "picometer seconds";
        @femtometer_second: prefix!(femto); "fm · s", "femtometer second",
            "femtometer seconds";
        @attometer_second: prefix!(atto); "am · s", "attometer second",
            "attometer seconds";
        @zeptometer_second: prefix!(zepto); "zm · s", "zeptometer second",
            "zeptometer seconds";
        @yoctometer_second: prefix!(yocto); "ym · s", "yoctometer second",
            "yoctometer seconds";

        @foot_second: 3.048_E-1; "ft · s", "foot second", "foot seconds";
        @inch_second: 2.54_E-2; "in · s", "inch second", "inch seconds";
        @kilometer_hour: 3.6_E6; "km · h", "kilometer hour", "kilometer hours";
    }
}

#[cfg(test)]
mod tests {
    use crate::{units::{absement::{self, Absement}, length::{self, Length}, time::Time}, units_base::{Unit, UnitBase}};


    #[test]
    fn check_dimension() {
        println!("{:?}", Absement::base_unit());
        println!("{:?}",UnitBase::new_length()*UnitBase::new_time());
        assert!(Absement::base_unit() == UnitBase::new_length()*UnitBase::new_time());
    }

        #[test]
        fn check_units() {
            test(Length::yottameter, Time::second, Absement::yottameter_second);
            test(Length::zettameter, Time::second, Absement::zettameter_second);
            test(Length::exameter, Time::second, Absement::exameter_second);
            test(Length::petameter, Time::second, Absement::petameter_second);
            test(Length::terameter, Time::second, Absement::terameter_second);
            test(Length::gigameter, Time::second, Absement::gigameter_second);
            test(Length::megameter, Time::second, Absement::megameter_second);
            test(Length::kilometer, Time::second, Absement::kilometer_second);
            test(Length::hectometer, Time::second, Absement::hectometer_second);
            test(Length::decameter, Time::second, Absement::decameter_second);
            test(Length::meter, Time::second, Absement::meter_second);
            test(Length::decimeter, Time::second, Absement::decimeter_second);
            test(Length::centimeter, Time::second, Absement::centimeter_second);
            test(Length::millimeter, Time::second, Absement::millimeter_second);
            test(Length::micrometer, Time::second, Absement::micrometer_second);
            test(Length::nanometer, Time::second, Absement::nanometer_second);
            test(Length::picometer, Time::second, Absement::picometer_second);
            test(Length::femtometer, Time::second, Absement::femtometer_second);
            test(Length::attometer, Time::second, Absement::attometer_second);
            test(Length::zeptometer, Time::second, Absement::zeptometer_second);
            test(Length::yoctometer, Time::second, Absement::yoctometer_second);            
            test(Length::foot, Time::second, Absement::foot_second);
            test(Length::inch, Time::second, Absement::inch_second);
            test(Length::kilometer, Time::hour, Absement::kilometer_hour);
        }
            fn test(length: Length, time: Time, absement: Absement) {
                let unit: Unit = absement.into();
                let length_unit: Unit = length.into();
                let unit2: Unit = length_unit*time.into();               
                assert!(unit == unit2);
            }
        
}
