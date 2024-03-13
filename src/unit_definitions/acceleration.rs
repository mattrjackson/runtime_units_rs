//! Acceleration (base UnitDefinition meter per second squared, m · s⁻²).
use crate::{prefix, quantity};
quantity! {
    /// Acceleration (base UnitDefinition meter per second squared, m · s⁻²).
    quantity: Acceleration; "acceleration";
    /// Dimension of acceleration, LT⁻² (base UnitDefinition meter per second squared, m · s⁻²).
    dimension: ISQ<
        P1,     // length
        Z0,     // mass
        N2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottameter_per_second_squared: prefix!(yotta); "Ym/s²", "yottameter per second squared",
            "yottameters per second squared";
        @zettameter_per_second_squared: prefix!(zetta); "Zm/s²", "zettameter per second squared",
            "zettameters per second squared";
        @exameter_per_second_squared: prefix!(exa); "Em/s²", "exameter per second squared",
            "exameters per second squared";
        @petameter_per_second_squared: prefix!(peta); "Pm/s²", "petameter per second squared",
            "petameters per second squared";
        @terameter_per_second_squared: prefix!(tera); "Tm/s²", "terameter per second squared",
            "terameters per second squared";
        @gigameter_per_second_squared: prefix!(giga); "Gm/s²", "gigameter per second squared",
            "gigameters per second squared";
        @megameter_per_second_squared: prefix!(mega); "Mm/s²", "megameter per second squared",
            "megameters per second squared";
        @kilometer_per_second_squared: prefix!(kilo); "km/s²", "kilometer per second squared",
            "kilometers per second squared";
        @hectometer_per_second_squared: prefix!(hecto); "hm/s²", "hectometer per second squared",
            "hectometers per second squared";
        @decameter_per_second_squared: prefix!(deca); "dam/s²", "decameter per second squared",
            "decameters per second squared";
        @meter_per_second_squared: prefix!(none); "m/s²", "meter per second squared",
            "meters per second squared";
        @decimeter_per_second_squared: prefix!(deci); "dm/s²", "decimeter per second squared",
            "decimeters per second squared";
        @centimeter_per_second_squared: prefix!(centi); "cm/s²", "centimeter per second squared",
            "centimeters per second squared";
        @millimeter_per_second_squared: prefix!(milli); "mm/s²", "millimeter per second squared",
            "millimeters per second squared";
        @micrometer_per_second_squared: prefix!(micro); "µm/s²", "micrometer per second squared",
            "micrometers per second squared";
        @nanometer_per_second_squared: prefix!(nano); "nm/s²", "nanometer per second squared",
            "nanometers per second squared";
        @picometer_per_second_squared: prefix!(pico); "pm/s²", "picometer per second squared",
            "picometers per second squared";
        @femtometer_per_second_squared: prefix!(femto); "fm/s²", "femtometer per second squared",
            "femtometers per second squared";
        @attometer_per_second_squared: prefix!(atto); "am/s²", "attometer per second squared",
            "attometers per second squared";
        @zeptometer_per_second_squared: prefix!(zepto); "zm/s²", "zeptometer per second squared",
            "zeptometers per second squared";
        @yoctometer_per_second_squared: prefix!(yocto); "ym/s²", "yoctometer per second squared",
            "yoctometers per second squared";

        @foot_per_second_squared: 3.048_E-1; "ft/s²", "foot per second squared",
            "feet per second squared";
        @galileo: 1.0_E-2; "Gal", "galileo", "galileos";
        @inch_per_second_squared: 2.54_E-2; "in/s²", "inch per second squared",
            "inches per second squared";
        @millimeter_per_minute_squared: 2.777_777_777_777_777_8_E-7; "mm/min²",
            "millimeter per minute squared", "millimeters per minute squared";
        @standard_gravity: 9.806_65_E0; "g₀", "standard acceleration of gravity",
            "standard accelerations of gravity";
    }
}

#[cfg(test)]
#[cfg(feature="Acceleration")]
mod tests {
    use crate::{unit_definitions::time::TimeUnit, units::{AccelerationUnit, LengthUnit}, units_base::{UnitDefinition, UnitBase}};

    #[test]
    fn check_dimension() {
        assert_eq!(AccelerationUnit::unit_base(), UnitBase::new_length()/UnitBase::new_time().powi(2));
    }
    #[test]
    fn check_units() {
        test_unit(LengthUnit::yottameter, TimeUnit::second, AccelerationUnit::yottameter_per_second_squared);
        test_unit(LengthUnit::zettameter, TimeUnit::second, AccelerationUnit::zettameter_per_second_squared);
        test_unit(LengthUnit::exameter, TimeUnit::second, AccelerationUnit::exameter_per_second_squared);
        test_unit(LengthUnit::petameter, TimeUnit::second, AccelerationUnit::petameter_per_second_squared);
        test_unit(LengthUnit::terameter, TimeUnit::second, AccelerationUnit::terameter_per_second_squared);
        test_unit(LengthUnit::gigameter, TimeUnit::second, AccelerationUnit::gigameter_per_second_squared);
        test_unit(LengthUnit::megameter, TimeUnit::second, AccelerationUnit::megameter_per_second_squared);
        test_unit(LengthUnit::kilometer, TimeUnit::second, AccelerationUnit::kilometer_per_second_squared);
        test_unit(LengthUnit::hectometer, TimeUnit::second, AccelerationUnit::hectometer_per_second_squared);
        test_unit(LengthUnit::decameter, TimeUnit::second, AccelerationUnit::decameter_per_second_squared);
        test_unit(LengthUnit::meter, TimeUnit::second, AccelerationUnit::meter_per_second_squared);
        test_unit(LengthUnit::decimeter, TimeUnit::second, AccelerationUnit::decimeter_per_second_squared);
        test_unit(LengthUnit::centimeter, TimeUnit::second, AccelerationUnit::centimeter_per_second_squared);
        test_unit(LengthUnit::millimeter, TimeUnit::second, AccelerationUnit::millimeter_per_second_squared);
        test_unit(LengthUnit::micrometer, TimeUnit::second, AccelerationUnit::micrometer_per_second_squared);
        test_unit(LengthUnit::nanometer, TimeUnit::second, AccelerationUnit::nanometer_per_second_squared);
        test_unit(LengthUnit::picometer, TimeUnit::second, AccelerationUnit::picometer_per_second_squared);
        test_unit(LengthUnit::femtometer, TimeUnit::second, AccelerationUnit::femtometer_per_second_squared);
        test_unit(LengthUnit::attometer, TimeUnit::second, AccelerationUnit::attometer_per_second_squared);
        test_unit(LengthUnit::zeptometer, TimeUnit::second, AccelerationUnit::zeptometer_per_second_squared);
        test_unit(LengthUnit::yoctometer, TimeUnit::second, AccelerationUnit::yoctometer_per_second_squared);

        test_unit(LengthUnit::foot, TimeUnit::second, AccelerationUnit::foot_per_second_squared);
        test_unit(LengthUnit::centimeter, TimeUnit::second, AccelerationUnit::galileo);
        test_unit(LengthUnit::inch, TimeUnit::second, AccelerationUnit::inch_per_second_squared);
        test_unit(LengthUnit::millimeter, TimeUnit::minute, AccelerationUnit::millimeter_per_minute_squared);        
    }
    fn test_unit(length: LengthUnit, time: TimeUnit, definition: AccelerationUnit) {
        assert_eq!(Into::<UnitDefinition>::into(definition), Into::<UnitDefinition>::into(length)/ Into::<UnitDefinition>::into(time).powi(2));
    }
   
}
