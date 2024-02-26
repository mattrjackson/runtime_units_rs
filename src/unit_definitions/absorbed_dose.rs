//! Absorbed Dose (m^2/s^2).

use crate::{prefix, quantity};
quantity! {
    /// Absorbed Dose (Gray) J/kg => m^2/s^2
    quantity: AbsorbedDose; "absorbed_dose";
    /// Dimension of absorbed dose.
    dimension: ISQ<
        P2,     // length
        Z0,     // mass
        N2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::AbsorbedDoseKind);
    units {        
        /// Radiation unit (defined in CGS units in 1953)
        @rad: prefix!(centi); "rad", "rad", "rad";

        @yottagray: prefix!(yotta); "YGy", "yottagray", "yottagrays";
        @zettagray: prefix!(zetta); "ZGy", "zettagray", "zettagrays";
        @exagray: prefix!(exa); "EGy", "exagray", "exagrays";
        @petagray: prefix!(peta); "PGy", "petagray", "petagrays";
        @teragray: prefix!(tera); "TGy", "teragray", "teragrays";
        @gigagray: prefix!(giga); "GGy", "gigagray", "gigagrays";
        @megagray: prefix!(mega); "MGy", "megagray", "megagrays";
        @kilogray: prefix!(kilo); "kGy", "kilogray", "kilograys";
        @hectogray: prefix!(hecto); "hGy", "hectogray", "hectograys";
        @decagray: prefix!(deca); "daGy", "decagray", "decagrays";
        /// SI derived unit of absorbed dose.
        @gray: prefix!(none); "Gy", "gray", "grays";
        @decigray: prefix!(deci); "dGy", "decigray", "decigrays";
        @centigray: prefix!(centi); "cGy", "centigray", "centigrays";
        @milligray: prefix!(milli); "mGy", "milligray", "milligrays";
        @microgray: prefix!(micro); "µGy", "microgray", "micrograys";
        @nanogray: prefix!(nano); "nGy", "nanogray", "nanograys";
        @picogray: prefix!(pico); "pGy", "picogray", "picograys";
        @femtogray: prefix!(femto); "fGy", "femtogray", "femtograys";
        @attogray: prefix!(atto); "aGy", "attogray", "attograys";
        @zeptogray: prefix!(zepto); "zN", "zeptogray", "zeptograys";
        @yoctogray: prefix!(yocto); "yN", "yoctogray", "yoctograys";
    }
}

#[cfg(test)]
#[cfg(feature="AbsorbedDose")]
mod tests {
    
    #[test]
    fn check_units() {
        use crate::AbsorbedDose;
        assert_eq!(AbsorbedDose::centigray(100.0).to_quantity(), AbsorbedDose::gray(1.0).to_quantity());
        assert_ne!(AbsorbedDose::centigray(10.0).to_quantity(), AbsorbedDose::gray(1.0).to_quantity());
        
        assert_eq!(AbsorbedDose::centigray(10.0).to_quantity(), AbsorbedDose::rad(10.0).to_quantity());
        assert_eq!(AbsorbedDose::gray(0.1).to_quantity(), AbsorbedDose::rad(10.0).to_quantity());
        assert_ne!(AbsorbedDose::centigray(10.0).to_quantity(), AbsorbedDose::rad(100.0).to_quantity());
    }    
}
