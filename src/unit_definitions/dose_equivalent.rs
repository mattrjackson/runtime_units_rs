//! Dose Equivalent (m^2/s^2).
use crate::{prefix, quantity};
quantity! {
    /// Dose Equivalent (Sievert) J/kg => m^2/s^2
    quantity: DoseEquivalent; "dose_equivalent";
    /// Dimension of dose equivalent.
    dimension: ISQ<
        P2,     // length
        Z0,     // mass
        N2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::DoseEquivalentKind);
    units {
      
        /// Roentgen equivalent man
        @rem: prefix!(centi)*prefix!(none); "rem", "rem", "rems";
        @decirem: prefix!(centi)*prefix!(deci); "drem", "decirem", "decirems";
        @centirem: prefix!(centi)*prefix!(centi); "crem", "centirem", "centirems";
        @millirem: prefix!(centi)*prefix!(milli); "mrem", "millirem", "millirems";
        @microrem: prefix!(centi)*prefix!(micro); "µrem", "microrem", "microrems";
        @nanorem: prefix!(centi)*prefix!(nano); "nrem", "nanorem", "nanorems";
        @picorem: prefix!(centi)*prefix!(pico); "prem", "picorem", "picorems";
        @femtorem: prefix!(centi)*prefix!(femto); "frem", "femtorem", "femtorems";
        @attorem: prefix!(centi)*prefix!(atto); "arem", "attorem", "attorems";
        @zeptorem: prefix!(centi)*prefix!(zepto); "zN", "zeptorem", "zeptorems";
        @yoctorem: prefix!(centi)*prefix!(yocto); "yN", "yoctorem", "yoctorems";

        @yottasievert: prefix!(centi)*prefix!(yotta); "YSv", "yottasievert", "yottasieverts";
        @zettasievert: prefix!(centi)*prefix!(zetta); "ZSv", "zettasievert", "zettasieverts";
        @exasievert: prefix!(centi)*prefix!(exa); "ESv", "exasievert", "exasieverts";
        @petasievert: prefix!(centi)*prefix!(peta); "PSv", "petasievert", "petasieverts";
        @terasievert: prefix!(centi)*prefix!(tera); "TSv", "terasievert", "terasieverts";
        @gigasievert: prefix!(centi)*prefix!(giga); "GSv", "gigasievert", "gigasieverts";
        @megasievert: prefix!(centi)*prefix!(mega); "MSv", "megasievert", "megasieverts";
        @kilosievert: prefix!(centi)*prefix!(kilo); "kSv", "kilosievert", "kilosieverts";
        @hectosievert: prefix!(centi)*prefix!(hecto); "hSv", "hectosievert", "hectosieverts";
        @decasievert: prefix!(centi)*prefix!(deca); "daSv", "decasievert", "decasieverts";
        /// SI derived UnitDefinition of dose equivalent. 
        @sievert: prefix!(none); "Sv", "sievert", "sieverts";
        @decisievert: prefix!(deci); "dSv", "decisievert", "decisieverts";
        @centisievert: prefix!(centi); "cSv", "centisievert", "centisieverts";
        @millisievert: prefix!(milli); "mSv", "millisievert", "millisieverts";
        @microsievert: prefix!(micro); "µSv", "microsievert", "microsieverts";
        @nanosievert: prefix!(nano); "nSv", "nanosievert", "nanosieverts";
        @picosievert: prefix!(pico); "pSv", "picosievert", "picosieverts";
        @femtosievert: prefix!(femto); "fSv", "femtosievert", "femtosieverts";
        @attosievert: prefix!(atto); "aSv", "attosievert", "attosieverts";
        @zeptosievert: prefix!(zepto); "zSv", "zeptosievert", "zeptosieverts";
        @yoctosievert: prefix!(yocto); "ySv", "yoctosievert", "yoctosieverts";
    }
}

#[cfg(test)]
#[cfg(feature="DoseEquivalent")]
mod tests {
    use crate::units_base::UnitBase;
    use crate::{DoseEquivalent, units::DoseEquivalentUnit};
    #[test]
    fn check_dimension() {
        assert_eq!(DoseEquivalentUnit::unit_base(), UnitBase::new_length().powi(2) / UnitBase::new_time().powi(2));
    }
    #[test]
    fn check_units() {
        
        assert_eq!(DoseEquivalent::centisievert(100.0), DoseEquivalent::sievert(1.0));
        assert_ne!(DoseEquivalent::centisievert(10.0), DoseEquivalent::sievert(1.0));
        
        assert_eq!(DoseEquivalent::centisievert(10.0), DoseEquivalent::rem(10.0));
        assert_eq!(DoseEquivalent::sievert(0.1), DoseEquivalent::rem(10.0));
        assert_ne!(DoseEquivalent::centisievert(10.0), DoseEquivalent::rem(100.0));
    }    
}
