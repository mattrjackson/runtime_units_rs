use core::fmt::Debug;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::errors::RuntimeUnitError;
use crate::units_base::Unit;
use crate::Units;

pub(crate) trait IsQuantity
{
    fn value(&self) -> f64;
    fn unit(&self) -> Unit;
}

#[doc = "A quantity of a unit, supports converting from one unit to another." ]
#[derive(Copy, Clone)]
pub struct Quantity
{
    pub value: f64,
    pub unit: Unit
}
impl Quantity
{
    ///
    /// Create a new instance of `Quantity` with a given `value` and `unit` 
    ///
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
                Err(RuntimeUnitError::IncompatibleUnitConversion(format!("Could not convert from base units of {} to {}", self.unit.unit_string(), <crate::unit_definitions::Units as Into<Unit>>::into(unit).unit_string())))
            }
        }
    }    

    #[inline] 
    /// Convert from one unit to another (no check is made to ensure destination unit is valid).
    pub(crate) fn convert_unit(&self, unit: Unit) -> f64
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
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result 
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
    fn partial_cmp(&self, other: &Quantity) -> Option<core::cmp::Ordering> {
        self.convert_unit(other.unit).partial_cmp(&other.value)
    }
}

impl PartialEq for Quantity
{
    fn eq(&self, other: &Self) -> bool {
        self.unit.base == other.unit.base && self.convert_unit(other.unit) == other.value
    }
}