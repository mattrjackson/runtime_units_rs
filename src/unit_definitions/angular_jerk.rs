//! Angular jerk (base UnitDefinition radian per second cubed, s⁻³).
use crate::quantity;
quantity! {
    /// Angular jerk (base UnitDefinition radian per second cubed, s⁻³).
    quantity: AngularJerk; "angular jerk";
    /// Dimension of angular jerk, T⁻³ (base UnitDefinition radian per second cubed, s⁻³).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        N3,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::AngleKind);
    units {
        /// Derived UnitDefinition of angular jerk.
        @radian_per_second_cubed: 1.0; "rad/s³", "radian per second cubed",
            "radians per second cubed";
        @degree_per_second_cubed: 1.745_329_251_994_329_5_E-2; "°/s³",
            "degree per second cubed", "degrees per second cubed";
    }
}
#[cfg(test)]
mod tests {
    use crate::{units::{AngleUnit, AngularJerkUnit, TimeUnit}, units_base::UnitDefinition};
    
    #[test]
    fn check_dimension() {
        assert_eq!(AngularJerkUnit::unit_base(), AngleUnit::unit_base() / TimeUnit::unit_base().powi(3));
    }

    #[test]
    fn check_units() {
        test_unit(AngularJerkUnit::radian_per_second_cubed, TimeUnit::second, AngleUnit::radian);
        test_unit(AngularJerkUnit::degree_per_second_cubed, TimeUnit::second, AngleUnit::degree);

    }
    fn test_unit(value: AngularJerkUnit, time: TimeUnit, angle: AngleUnit) {
        assert_eq!(Into::<UnitDefinition>::into(value), Into::<UnitDefinition>::into(angle) / Into::<UnitDefinition>::into(time).powi(3));
    }
}
