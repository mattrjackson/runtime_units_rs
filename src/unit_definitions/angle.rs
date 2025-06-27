//! Angle (dimensionless quantity).

use crate::quantity;

#[cfg(feature="Angle")]
quantity! {
    /// Angle (dimensionless quantity).
    quantity: Angle; "angle";
    /// Dimension of angle, 1 (dimensionless).
    dimension: ISQ[
        0.0,     // length
        0.0,     // mass
        0.0,     // time
        0.0,     // electric current
        0.0,     // thermodynamic temperature
        0.0,     // amount of substance
        0.0];    // luminous intensity
    kind: dyn (crate::si::marker::AngleKind);
    units {
        /// SI derived UnitDefinition of angle. It is the angle subtended at the center of a circle by an
        /// arc that is equal in length to the radius of the circle.
        @radian: 1.0_E0; "rad", "radian", "radians";
        @revolution: 6.283_185_307_179_586_E0; "r", "revolution", "revolutions";
        @degree: 1.745_329_251_994_329_5_E-2; "°", "degree", "degrees";
        @gon: 1.570_796_326_794_896_7_E-2; "gon", "gon", "gons";
        @mil: 9.817_477_E-4; "mil", "mil", "mils";
        @minute: 2.908_882_086_657_216_E-4; "′", "minute", "minutes";
        @second: 4.848_136_811_095_36_E-6; "″", "second", "seconds";
    }
}

#[cfg(feature="Angle")]
impl Angle
{
    /// Computes the value of the cosine of the angle.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline(always)]
    pub fn cos(self) -> f64 {
        self.convert_unchecked(AngleUnit::get_radian()).cos()
    }

    /// Computes the value of the hyperbolic cosine of the angle.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline(always)]
    pub fn cosh(self) -> f64 {
        self.convert_unchecked(AngleUnit::get_radian()).cosh()  
    }

    /// Computes the value of the sine of the angle.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline(always)]
    pub fn sin(self) -> f64 {
        self.convert_unchecked(AngleUnit::get_radian()).sin() 
    }

    /// Computes the value of the hyperbolic sine of the angle.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline(always)]
    pub fn sinh(self) -> f64 {
        self.convert_unchecked(AngleUnit::get_radian()).sinh() 
    }

    /// Computes the value of both the sine and cosine of the angle.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline(always)]
    pub fn sin_cos(self) -> (f64, f64) {
        let (sin, cos) = self.convert_unchecked(AngleUnit::get_radian()).sin_cos();
        (sin, cos)
    }

    /// Computes the value of the tangent of the angle.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline(always)]
    pub fn tan(self) -> f64 {
        self.convert_unchecked(AngleUnit::get_radian()).tan()
    }

    /// Computes the value of the hyperbolic tangent of the angle.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline(always)]
    pub fn tanh(self) -> f64 {
        self.convert_unchecked(AngleUnit::get_radian()).tanh()
    }
}
