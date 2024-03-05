//! Diffusion coefficient (base unit square meter per second, m² · s⁻¹).
use crate::{prefix, quantity};
quantity! {
    /// Diffusion coefficient (base unit square meter per second, m² · s⁻¹).
    quantity: DiffusionCoefficient; "diffusion coefficient";
    /// Dimension of diffusion coefficient, L²T⁻¹ (base unit square meter per second, m² · s⁻¹).
    dimension: ISQ<
        P2,     // length
        Z0,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @square_meter_per_second: prefix!(none); "m²/s", "square meter per second",
            "square meters per second";
        @square_centimeter_per_second: prefix!(centi) * prefix!(centi); "cm²/s",
            "square centimeter per second", "square centimeters per second";
        @square_millimeter_per_second: prefix!(milli) * prefix!(milli); "mm²/s",
            "square millimeter per second", "square millimeters per second";
        @square_micrometer_per_second: prefix!(micro) * prefix!(micro); "µm²/s",
            "square micrometer per second", "square micrometers per second";
        @square_nanometer_per_second: prefix!(nano) * prefix!(nano); "nm²/s",
            "square nanometer per second", "square nanometers per second";
        @stokes: prefix!(centi) * prefix!(centi); "St", "Stokes", "Stokes";
        @centistokes: prefix!(centi) * prefix!(centi) * prefix!(centi); "cSt", "centistokes",
            "centistokes";
        }
}

#[cfg(test)]
mod test {
    use crate::{unit_definitions::{diffusion_coefficient::DiffusionCoefficientUnit, time::TimeUnit}, units::AreaUnit, units_base::Unit};



    #[test]
    fn check_dimension() {
        assert_eq!(DiffusionCoefficientUnit::unit_base(), AreaUnit::unit_base() / TimeUnit::unit_base());
    }

    #[test]
    fn check_units() {
        test_unit(AreaUnit::square_meter, TimeUnit::second, DiffusionCoefficientUnit::square_meter_per_second);
        test_unit(AreaUnit::square_centimeter, TimeUnit::second, DiffusionCoefficientUnit::square_centimeter_per_second);
        test_unit(AreaUnit::square_millimeter, TimeUnit::second, DiffusionCoefficientUnit::square_millimeter_per_second);
        test_unit(AreaUnit::square_micrometer, TimeUnit::second, DiffusionCoefficientUnit::square_micrometer_per_second);
        test_unit(AreaUnit::square_nanometer, TimeUnit::second, DiffusionCoefficientUnit::square_nanometer_per_second);
        test_unit(AreaUnit::square_centimeter, TimeUnit::second, DiffusionCoefficientUnit::stokes);
        test_unit(AreaUnit::square_millimeter, TimeUnit::second, DiffusionCoefficientUnit::centistokes);

    }
    fn test_unit(area: AreaUnit, time: TimeUnit, value: DiffusionCoefficientUnit) {
        assert!(Into::<Unit>::into(value).approx_eq(Into::<Unit>::into(area) / Into::<Unit>::into(time), 1e-12));
    }
    
}
