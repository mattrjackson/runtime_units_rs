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
#[cfg(any(feature = "Absement", feature = "All"))]   
mod tests {
use crate::{units::{AbsementUnit, LengthUnit, TimeUnit}, units_base::{Unit, UnitBase}};


#[test]
fn check_dimension() {
println!("{:?}", AbsementUnit::get_unit_base());
println!("{:?}",UnitBase::new_length()*UnitBase::new_time());
assert!(AbsementUnit::get_unit_base() == UnitBase::new_length()*UnitBase::new_time());
}

#[test]
fn check_units() {
    test(LengthUnit::yottameter, TimeUnit::second, AbsementUnit::yottameter_second);
    test(LengthUnit::zettameter, TimeUnit::second, AbsementUnit::zettameter_second);
    test(LengthUnit::exameter, TimeUnit::second, AbsementUnit::exameter_second);
    test(LengthUnit::petameter, TimeUnit::second, AbsementUnit::petameter_second);
    test(LengthUnit::terameter, TimeUnit::second, AbsementUnit::terameter_second);
    test(LengthUnit::gigameter, TimeUnit::second, AbsementUnit::gigameter_second);
    test(LengthUnit::megameter, TimeUnit::second, AbsementUnit::megameter_second);
    test(LengthUnit::kilometer, TimeUnit::second, AbsementUnit::kilometer_second);
    test(LengthUnit::hectometer, TimeUnit::second, AbsementUnit::hectometer_second);
    test(LengthUnit::decameter, TimeUnit::second, AbsementUnit::decameter_second);
    test(LengthUnit::meter, TimeUnit::second, AbsementUnit::meter_second);
    test(LengthUnit::decimeter, TimeUnit::second, AbsementUnit::decimeter_second);
    test(LengthUnit::centimeter, TimeUnit::second, AbsementUnit::centimeter_second);
    test(LengthUnit::millimeter, TimeUnit::second, AbsementUnit::millimeter_second);
    test(LengthUnit::micrometer, TimeUnit::second, AbsementUnit::micrometer_second);
    test(LengthUnit::nanometer, TimeUnit::second, AbsementUnit::nanometer_second);
    test(LengthUnit::picometer, TimeUnit::second, AbsementUnit::picometer_second);
    test(LengthUnit::femtometer, TimeUnit::second, AbsementUnit::femtometer_second);
    test(LengthUnit::attometer, TimeUnit::second, AbsementUnit::attometer_second);
    test(LengthUnit::zeptometer, TimeUnit::second, AbsementUnit::zeptometer_second);
    test(LengthUnit::yoctometer, TimeUnit::second, AbsementUnit::yoctometer_second);            
    test(LengthUnit::foot, TimeUnit::second, AbsementUnit::foot_second);
    test(LengthUnit::inch, TimeUnit::second, AbsementUnit::inch_second);
    test(LengthUnit::kilometer, TimeUnit::hour, AbsementUnit::kilometer_hour);
}

fn test(length: LengthUnit, time: TimeUnit, absement: AbsementUnit) {
    let unit: Unit = absement.into();
    let length_unit: Unit = length.into();
    let unit2: Unit = length_unit*time.into();               
    assert!(unit == unit2);
}
        
}
