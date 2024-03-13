//! Area (base UnitDefinition square meter, m²).
use crate::{prefix, quantity};
quantity! {
    /// Area (base UnitDefinition square meter, m²).
    quantity: Area; "area";
    /// Dimension of area, L² (base UnitDefinition square meter, m²).
    dimension: ISQ<
        P2,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @square_yottameter: prefix!(yotta) * prefix!(yotta);
            "Ym²", "square yottameter", "square yottameters";
        @square_zettameter: prefix!(zetta) * prefix!(zetta);
            "Zm²", "square zettameter", "square zettameters";
        @square_exameter: prefix!(exa) * prefix!(exa);
            "Em²", "square exameter", "square exameters";
        @square_petameter: prefix!(peta) * prefix!(peta);
            "Pm²", "square petameter", "square petameters";
        @square_terameter: prefix!(tera) * prefix!(tera);
            "Tm²", "square terameter", "square terameters";
        @square_gigameter: prefix!(giga) * prefix!(giga);
            "Gm²", "square gigameter", "square gigameters";
        @square_megameter: prefix!(mega) * prefix!(mega);
            "Mm²", "square megameter", "square megameters";
        @square_kilometer: prefix!(kilo) * prefix!(kilo);
            "km²", "square kilometer", "square kilometers";
        @square_hectometer: prefix!(hecto) * prefix!(hecto);
            "hm²", "square hectometer", "square hectometers";
        @square_decameter: prefix!(deca) * prefix!(deca);
            "dam²", "square decameter", "square decameters";
        @square_meter: prefix!(none);
            "m²", "square meter", "square meters";
        @square_decimeter: prefix!(deci) * prefix!(deci);
            "dm²", "square decimeter", "square decimeters";
        @square_centimeter: prefix!(centi) * prefix!(centi);
            "cm²", "square centimeter", "square centimeters";
        @square_millimeter: prefix!(milli) * prefix!(milli);
            "mm²", "square millimeter", "square millimeters";
        @square_micrometer: prefix!(micro) * prefix!(micro);
            "µm²", "square micrometer", "square micrometers";
        @square_nanometer: prefix!(nano) * prefix!(nano);
            "nm²", "square nanometer", "square nanometers";
        @square_picometer: prefix!(pico) * prefix!(pico);
            "pm²", "square picometer", "square picometers";
        @square_femtometer: prefix!(femto) * prefix!(femto);
            "fm²", "square femtometer", "square femtometers";
        @square_attometer: prefix!(atto) * prefix!(atto);
            "am²", "square attometer", "square attometers";
        @square_zeptometer: prefix!(zepto) * prefix!(zepto);
            "zm²", "square zeptometer", "square zeptometers";
        @square_yoctometer: prefix!(yocto) * prefix!(yocto);
            "ym²", "square yoctometer", "square yoctometers";

        @acre: 4.046_873_E3; "ac", "acre", "acres";
        @are: 1.0_E2; "a", "are", "ares";
        @barn: 1.0_E-28; "b", "barn", "barns";
        @circular_mil: 5.067_075_E-10; "cmil", "circular mil", "circular mils";
        @hectare: 1.0_E4; "ha", "hectare", "hectares";
        @square_foot: 9.290_304_E-2; "ft²", "square foot", "square feet";
        @square_inch: 6.451_6_E-4; "in²", "square inch", "square inches";
        @square_mile: 2.589_988_E6; "mi²", "square mile", "square miles";
        @square_yard: 8.361_274_E-1; "yd²", "square yard", "square yards";
    }
}

#[cfg(test)]
mod tests {
    use crate::{units::{AreaUnit, LengthUnit}, units_base::UnitDefinition};


    #[test]
    fn check_dimension() {
        assert_eq!(AreaUnit::unit_base(),  LengthUnit::unit_base().powi(2));
    }

    #[test]
    fn check_units() {
        test_unit(LengthUnit::yottameter, AreaUnit::square_yottameter);
        test_unit(LengthUnit::zettameter, AreaUnit::square_zettameter);

        test_unit(LengthUnit::exameter, AreaUnit::square_exameter);
        test_unit(LengthUnit::petameter, AreaUnit::square_petameter);
        test_unit(LengthUnit::terameter, AreaUnit::square_terameter);
        test_unit(LengthUnit::gigameter, AreaUnit::square_gigameter);
        test_unit(LengthUnit::megameter, AreaUnit::square_megameter);
        test_unit(LengthUnit::kilometer, AreaUnit::square_kilometer);
        test_unit(LengthUnit::hectometer, AreaUnit::square_hectometer);
        test_unit(LengthUnit::decameter, AreaUnit::square_decameter);
        test_unit(LengthUnit::meter, AreaUnit::square_meter);
        test_unit(LengthUnit::decimeter, AreaUnit::square_decimeter);
        test_unit(LengthUnit::centimeter, AreaUnit::square_centimeter);
        test_unit(LengthUnit::millimeter, AreaUnit::square_millimeter);
        test_unit(LengthUnit::micrometer, AreaUnit::square_micrometer);
        test_unit(LengthUnit::nanometer, AreaUnit::square_nanometer);
        test_unit(LengthUnit::picometer, AreaUnit::square_picometer);
        test_unit(LengthUnit::femtometer, AreaUnit::square_femtometer);
        test_unit(LengthUnit::attometer, AreaUnit::square_attometer);
        test_unit(LengthUnit::zeptometer, AreaUnit::square_zeptometer);
      
    }
    fn test_unit(length: LengthUnit, value: AreaUnit) {
        assert_eq!(Into::<UnitDefinition>::into(value), Into::<UnitDefinition>::into(length).powi(2));
    }
}

