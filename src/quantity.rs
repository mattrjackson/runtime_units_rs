use core::fmt::{Display, Debug};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::errors::RuntimeUnitError;
use crate::traits::ArbitraryQuantity;
use crate::units_base::UnitDefinition;

impl crate::traits::ArbitraryQuantity for Quantity
{
    fn unit(&self) -> UnitDefinition {
        self.unit
    }

    fn convert(&self, unit: UnitDefinition)  -> Quantity
    {
        Quantity { value: self.value() * self.unit.convert_unchecked(unit), unit }
    }
    
    fn try_convert_mut(&mut self, unit: UnitDefinition) -> Result<(), RuntimeUnitError> {
        self.value *= self.unit.try_convert(unit)?;
        Ok(())
    }
    
    fn convert_mut(&mut self, unit: UnitDefinition) {
        self.value *= self.unit.convert_unchecked(unit);
    }
    
    fn try_convert(&self, unit: UnitDefinition) -> Result<Quantity, RuntimeUnitError> {
        Ok(Quantity { value : self.value * self.unit.try_convert(unit)?, unit })
    }
    
    fn unit_mut(&mut self) -> &mut UnitDefinition {
        &mut self.unit
    }
}

#[doc = "A quantity of a unit, supports converting from one unit to another." ]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone)]
pub struct Quantity
{
    pub(crate) value: f64,
    pub(crate) unit: UnitDefinition
}
impl Quantity
{
    ///
    /// Create a new instance of `Quantity` with a given `value` and `unit` 
    ///
    pub fn new(value: f64, unit: UnitDefinition) -> Self
    {        
        Self { value, unit }
    }

    ///
    /// Amount of unit stored in quantity
    /// 
    #[inline]    
    pub fn value(&self) -> f64
    {
        self.value
    }

    ///
    /// Get mutable reference to the value for this quantity.
    /// 
    #[inline]    
    pub fn value_mut(&mut self) -> &mut f64
    {
        &mut self.value
    }

    #[inline] 
    /// Convert from one unit to another (no check is made to ensure destination unit is valid).
    pub(crate) fn convert_unchecked(&self, unit: UnitDefinition) -> f64
    {
        if self.unit == unit
        {
            self.value
        }
        else
        {
            self.value * self.unit.multiplier / unit.multiplier()
        }
    }

    #[inline]
    pub fn powf(&self, value: f64) -> Quantity
    {
        let mut r = *self;
        r.unit = r.unit.powf(value);
        r.value = r.value.powf(value);
        r
    }

    #[inline]
    pub fn powi(&self, value: i8) -> Quantity
    {
        let mut r = *self;
        r.unit = r.unit.powi(value);
        r.value = r.value.powi(value as i32);
        r
    }

    #[inline]
    pub fn inv(&self) -> Quantity
    {
        Quantity {
            value: 1.0 / self.value,
            unit: self.unit.inv(),
        }
    }


}
impl Debug for Quantity
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result 
    {
        write!(f, "{} {:?}", self.value(), self.unit)
    }
}
impl Mul<f64> for Quantity
{
    type Output = Quantity;

    fn mul(self, rhs: f64) -> Self::Output {
        Self{ value: self.value*rhs, unit: self.unit }
    }
}

impl Div<f64> for Quantity
{
    type Output = Quantity;

    fn div(self, rhs: f64) -> Self::Output {
        Self{ value: self.value/rhs, unit: self.unit }
    }
}
impl Mul<Quantity> for Quantity
{
    type Output = Quantity;

    fn mul(self, rhs: Quantity) -> Self::Output {
        Self{ value: self.value*rhs.value, unit: self.unit*rhs.unit }
    }
}
impl Div<Quantity> for Quantity
{
    type Output = Quantity;

    fn div(self, rhs: Quantity) -> Self::Output {
        Self{ value: self.value/rhs.value, unit: self.unit/rhs.unit }
    }
}
impl Add<Quantity> for Quantity
{
    type Output=Quantity;

    fn add(self, rhs: Quantity) -> Self::Output {
        let rhs = rhs.try_convert(self.unit).unwrap_or_else(|_| panic!("Addition failed due to incompatible units `{}` and `{}`", self.unit, rhs.unit));
        Self { value: self.value + rhs.value, unit: self.unit }
    }
}
impl Sub<Quantity> for Quantity
{
    type Output=Quantity;

    fn sub(self, rhs: Quantity) -> Self::Output {
        let rhs = rhs.try_convert(self.unit).unwrap_or_else(|_| panic!("Subtraction failed due to incompatible units `{}` and `{}`", self.unit, rhs.unit));
        Self { value: self.value - rhs.value, unit: self.unit }
    }
}

impl AddAssign for Quantity
{
    fn add_assign(&mut self, rhs: Self) {
        let rhs = rhs.try_convert(self.unit).unwrap_or_else(|_| panic!("Addition failed due to incompatible units `{}` and `{}`", self.unit, rhs.unit));
        self.value += rhs.value;
    }
}

impl SubAssign for Quantity
{
    fn sub_assign(&mut self, rhs: Self) {
        let rhs = rhs.try_convert(self.unit).unwrap_or_else(|_| panic!("Subtraction failed due to incompatible units `{}` and `{}`", self.unit, rhs.unit));
        self.value -= rhs.value;
    }
}

impl MulAssign for Quantity
{
    fn mul_assign(&mut self, rhs: Self) {
        self.value *= rhs.value;
        self.unit *= rhs.unit;
    }
}

impl MulAssign<f64> for Quantity
{
    fn mul_assign(&mut self, rhs: f64) {
        self.value *= rhs;
    }
}


impl DivAssign for Quantity
{
    fn div_assign(&mut self, rhs: Self) {
        self.value /= rhs.value;
        self.unit /= rhs.unit;
    }
}

impl DivAssign<f64> for Quantity
{
    fn div_assign(&mut self, rhs: f64) {
        self.value /= rhs;
    }
}
///
/// This only compares magnitudes...
/// 
impl PartialOrd<Quantity> for Quantity
{
    fn partial_cmp(&self, other: &Quantity) -> Option<core::cmp::Ordering> {
        self.convert_unchecked(other.unit).partial_cmp(&other.value)
    }
}

impl PartialEq for Quantity
{
    fn eq(&self, other: &Self) -> bool {
        self.unit.base == other.unit.base && self.convert_unchecked(other.unit) == other.value
    }
}

impl Display for Quantity
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}