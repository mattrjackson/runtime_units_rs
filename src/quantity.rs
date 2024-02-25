use std::fmt::Debug;
use std::ops::{Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::errors::RuntimeUnitError;
use crate::units_base::Unit;
use crate::Units;

#[derive(Copy, Clone, PartialEq)]
 
#[cfg_attr(feature="utoipa", derive(utoipa::ToSchema))]
pub struct Quantity
{
    pub value: f64,
    pub unit: Unit
}
impl Quantity
{
    pub fn new(value: f64, unit: Units) -> Self
    {        
        Self { value, unit: unit.into() }
    }

    ///
    /// Amount of unit stored in quantity
    /// 
    pub fn value(&self) -> f64
    {
        self.value
    }

    ///
    /// Retrieve a unit with a corresponding multiplier
    /// 
    pub fn unit(&self) -> Unit
    {
        self.unit
    }

    ///
    /// Convert a quantity from one unit to another     
    ///
    #[inline]
    pub fn convert(&self, unit: Units) -> Result<Quantity, RuntimeUnitError>
    {
        if self.unit == unit.into()
        {
            Ok(*self)
        }
        else
        {
            if self.unit.is_convertible(unit.into())
            {
                Ok(Self { value: self.value * self.unit.multiplier / unit.multiplier(), unit: unit.into() })
            }
            else
            {
                Err(RuntimeUnitError::IncompatibleUnitConversion)
            }
        }
    }    

    #[inline] 
    pub fn convert_unit(&self, unit: Unit) -> f64
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


}
impl Debug for Quantity
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "{} {:?}", self.value(), self.unit())
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
        Self { value: self.value + rhs.value, unit: self.unit }
    }
}
impl Sub<Quantity> for Quantity
{
    type Output=Quantity;

    fn sub(self, rhs: Quantity) -> Self::Output {
        Self { value: self.value - rhs.value, unit: self.unit }
    }
}

impl AddAssign for Quantity
{
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl SubAssign for Quantity
{
    fn sub_assign(&mut self, rhs: Self) {
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


impl DivAssign for Quantity
{
    fn div_assign(&mut self, rhs: Self) {
        self.value /= rhs.value;
        self.unit /= rhs.unit;
    }
}

///
/// This only compares magnitudes...
/// 
impl PartialOrd<Quantity> for Quantity
{
    fn partial_cmp(&self, other: &Quantity) -> Option<std::cmp::Ordering> {
        self.convert_unit(other.unit).partial_cmp(&other.value)
    }
}