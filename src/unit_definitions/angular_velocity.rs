//! Angular velocity (base UnitDefinition radian per second, s⁻¹).
use crate::quantity;
quantity! {
    /// Angular velocity (base UnitDefinition radian per second, s⁻¹).
    quantity: AngularVelocity; "angular velocity";
    /// Dimension of angular velocity, T⁻¹ (base UnitDefinition radian per second, s⁻¹).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::AngleKind);
    units {
        /// Derived UnitDefinition of angular velocity.
        @radian_per_second: 1.0_E0; "rad/s", "radian per second", "radians per second";
        @degree_per_second: 1.745_329_251_994_329_5_E-2; "°/s", "degree per second",
            "degrees per second";
        @revolution_per_second: 6.283_185_307_179_586_E0; "rps", "revolution per second",
            "revolutions per second";
        @revolution_per_minute: 1.047_197_551_196_597_7_E-1; "rpm", "revolution per minute",
            "revolutions per minute";
        @revolution_per_hour: 1.745_329_251_994_329_6_E-3; "rph", "revolution per hour",
            "revolutions per hour";
    }
}
#[cfg(test)]
mod tests {
    use crate::{units::{AngleUnit, AngularVelocityUnit, TimeUnit}, units_base::UnitDefinition};
    
    #[test]
    fn check_dimension() {
        assert_eq!(AngularVelocityUnit::unit_base(), AngleUnit::unit_base() / TimeUnit::unit_base());
    }

    #[test]
    fn check_units() {
        test_unit(AngularVelocityUnit::radian_per_second, TimeUnit::second, AngleUnit::radian);
        test_unit(AngularVelocityUnit::revolution_per_second, TimeUnit::second, AngleUnit::revolution);
        test_unit(AngularVelocityUnit::revolution_per_minute, TimeUnit::minute, AngleUnit::revolution);
        test_unit(AngularVelocityUnit::revolution_per_hour, TimeUnit::hour, AngleUnit::revolution);

    }
    fn test_unit(value: AngularVelocityUnit, time: TimeUnit, angle: AngleUnit) {
        assert_eq!(Into::<UnitDefinition>::into(value), Into::<UnitDefinition>::into(angle) / Into::<UnitDefinition>::into(time));
    }
}