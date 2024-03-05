//! [Curvature](https://en.wikipedia.org/wiki/Curvature) (base unit radian per meter, m⁻¹).
use crate::{prefix, quantity};
quantity! {
    /// Curvature (base unit radian per meter, m⁻¹).
    quantity: Curvature; "curvature";
    /// Dimension of curvature, L⁻¹ (base unit radian per meter, m⁻¹).
    dimension: ISQ<
        N1,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::AngleKind);
    units {
        @radian_per_meter: 1.0_E0; "rad/m", "radian per meter", "radians per meter";
        @degree_per_meter: 1.745_329_251_994_329_5_E-2; "°/m", "degree per meter",
            "degrees per meter";

        @radian_per_millimeter: 1000.0; "rad/mm", "radian per millimeter", "radians per millimeter";
        @degree_per_millimeter: 1.745_329_251_994_329_5_E1; "°/mm", "degree per millimeter",
            "degrees per millimeter";
    }
}

#[cfg(test)]
mod tests {
    use crate::{unit_definitions::{angle::AngleUnit, curvature::CurvatureUnit}, units::LengthUnit, units_base::Unit};


    #[test]
    fn check_dimension() {
        assert_eq!(CurvatureUnit::unit_base(), AngleUnit::unit_base() / LengthUnit::unit_base());
    }

    #[test]
    fn check_units() {
        test_unit(AngleUnit::radian, LengthUnit::meter, CurvatureUnit::radian_per_meter);
        test_unit(AngleUnit::degree, LengthUnit::meter, CurvatureUnit::degree_per_meter);
        test_unit(AngleUnit::radian, LengthUnit::millimeter, CurvatureUnit::radian_per_millimeter);
        test_unit(AngleUnit::degree, LengthUnit::millimeter, CurvatureUnit::degree_per_millimeter);
    }
    fn test_unit(angle: AngleUnit, length: LengthUnit, value: CurvatureUnit) {
        assert_eq!(Into::<Unit>::into(value), Into::<Unit>::into(angle) / Into::<Unit>::into(length));
    }

}
