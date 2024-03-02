//! Angular acceleration (base unit radian per second squared, s⁻²).
use crate::quantity;
quantity! {
    /// Angular acceleration (base unit radian per second squared, s⁻²).
    quantity: AngularAcceleration; "angular acceleration";
    /// Dimension of angular acceleration, T⁻² (base unit radian per second squared, s⁻²).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        N2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::AngleKind);
    units {
        /// Derived unit of angular acceleration.
        @radian_per_second_squared: 1.0; "rad/s²", "radian per second squared",
            "radians per second squared";
        @degree_per_second_squared: 1.745_329_251_994_329_5_E-2; "°/s²",
            "degree per second squared", "degrees per second squared";
    }
}

#[cfg(test)]
mod tests {
    use crate::{units::{AngleUnit, AngularAccelerationUnit, TimeUnit}, units_base::Unit};
    
    #[test]
    fn check_dimension() {
        assert_eq!(AngularAccelerationUnit::unit(), AngleUnit::unit() / TimeUnit::unit().powi(2));
    }

    #[test]
    fn check_units() {
        test_unit(AngularAccelerationUnit::radian_per_second_squared, TimeUnit::second, AngleUnit::radian);
        test_unit(AngularAccelerationUnit::degree_per_second_squared, TimeUnit::second, AngleUnit::degree);

    }
    fn test_unit(value: AngularAccelerationUnit, time: TimeUnit, angle: AngleUnit) {
        assert_eq!(Into::<Unit>::into(value), Into::<Unit>::into(angle) / Into::<Unit>::into(time).powi(2));
    }
}
