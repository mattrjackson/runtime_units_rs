//! Time (base UnitDefinition second, s).

use crate::{prefix, quantity};

quantity! {
    /// Time (base UnitDefinition second, s).
    quantity: Time; "time";
    /// Dimension of time, T (base UnitDefinition second, s).
    dimension: ISQ[
        0.0,     // length
        0.0,     // mass
        1.0,     // time
        0.0,     // electric current
        0.0,     // thermodynamic temperature
        0.0,     // amount of substance
        0.0];    // luminous intensity
    units {
        @yottasecond: prefix!(yotta); "Ys", "yottasecond", "yottaseconds";
        @zettasecond: prefix!(zetta); "Zs", "zettasecond", "zettaseconds";
        @exasecond: prefix!(exa); "Es", "exasecond", "exaseconds";
        @petasecond: prefix!(peta); "Ps", "petasecond", "petaseconds";
        @terasecond: prefix!(tera); "Ts", "terasecond", "teraseconds";
        @gigasecond: prefix!(giga); "Gs", "gigasecond", "gigaseconds";
        @megasecond: prefix!(mega); "Ms", "megasecond", "megaseconds";
        @kilosecond: prefix!(kilo); "ks", "kilosecond", "kiloseconds";
        @hectosecond: prefix!(hecto); "hs", "hectosecond", "hectoseconds";
        @decasecond: prefix!(deca); "das", "decasecond", "decaseconds";
        /// The second is the SI UnitDefinition of time. It is defined by taking the fixed numerical value of
        /// the caesium frequency ∆*ν*<sub>Cs</sub>, the unperturbed ground-state hyperfine
        /// transition frequency of the caesium 133 atom, to be 9 192 631 770 when expressed in the
        /// UnitDefinition Hz, which is equal to s⁻¹.
        @second: prefix!(none); "s", "second", "seconds";
        @decisecond: prefix!(deci); "ds", "decisecond", "deciseconds";
        @centisecond: prefix!(centi); "cs", "centisecond", "centiseconds";
        @millisecond: prefix!(milli); "ms", "millisecond", "milliseconds";
        @microsecond: prefix!(micro); "µs", "microsecond", "microseconds";
        @nanosecond: prefix!(nano); "ns", "nanosecond", "nanoseconds";
        @picosecond: prefix!(pico); "ps", "picosecond", "picoseconds";
        @femtosecond: prefix!(femto); "fs", "femtosecond", "femtoseconds";
        @attosecond: prefix!(atto); "as", "attosecond", "attoseconds";
        @zeptosecond: prefix!(zepto); "zs", "zeptosecond", "zeptoseconds";
        @yoctosecond: prefix!(yocto); "ys", "yoctosecond", "yoctoseconds";

        @second_sidereal: 9.972_696_E-1; "s (sidereal)", "second (sidereal)", "seconds (sidereal)";
        @day: 8.64_E4; "d", "day", "days";
        @day_sidereal: 8.616_409_E4; "d (sidereal)", "day (sidereal)", "days (sidereal)";
        @hour: 3.6_E3; "h", "hour", "hours";
        @hour_sidereal: 3.590_170_E3; "h (sidereal)", "hour (sidereal)", "hours (sidereal)";
        @minute: 6.0_E1; "min", "minute", "minutes";
        @shake: 1.0_E-8; "10.0 ns", "shake", "shakes";
        @year: 3.1536_E7; "a", "year", "years";
        @year_sidereal: 3.155_815_E7; "a (sidereal)", "year (sidereal)", "years (sidereal)";
        @year_tropical: 3.155_693_E7; "a (tropical)", "year (tropical)", "years (tropical)";
    }
}
