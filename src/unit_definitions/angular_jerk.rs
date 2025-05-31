//! Angular jerk (base UnitDefinition radian per second cubed, s⁻³).
use crate::quantity;
quantity! {
    /// Angular jerk (base UnitDefinition radian per second cubed, s⁻³).
    quantity: AngularJerk; "angular jerk";
    /// Dimension of angular jerk, T⁻³ (base UnitDefinition radian per second cubed, s⁻³).
    dimension: ISQ[
        0.0,     // length
        0.0,     // mass
        -3.0,     // time
        0.0,     // electric current
        0.0,     // thermodynamic temperature
        0.0,     // amount of substance
        0.0];    // luminous intensity
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
use crate::traits::Unit;
    use crate::{units::{AngleUnit, AngularJerkUnit, TimeUnit}, units_base::UnitDefinition};
    
    #[test]
    fn check_dimension() {
        assert_eq!(AngularJerkUnit::base(), AngleUnit::base() / TimeUnit::base().powi(3));
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
