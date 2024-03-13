use core::fmt::Debug;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::errors::RuntimeUnitError;
use crate::units_base::UnitDefinition;
use crate::Units;

pub(crate) trait IsQuantity
{
    fn value(&self) -> f64;
    fn definition(&self) -> UnitDefinition;
}

#[doc = "A quantity of a unit, supports converting from one unit to another." ]
#[derive(Copy, Clone)]
pub struct QuantityBase
{
    pub(crate) value: f64,
    pub(crate) unit: UnitDefinition
}
impl QuantityBase
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
    /// Retrieve a unit with a corresponding multiplier
    /// 
    #[inline]
    pub fn definition(&self) -> UnitDefinition
    {
        self.unit
    }

    ///
    /// Convert a quantity from one unit to another
    ///
    #[inline]
    pub fn convert(&self, unit: Units) -> Result<QuantityBase, RuntimeUnitError>
    {
        self.convert_unit(unit.into())
    }    
    #[inline]
    pub fn convert_unit(&self, unit: UnitDefinition) -> Result<QuantityBase, RuntimeUnitError>
    {
        if self.unit == unit
        {
            Ok(*self)
        }
        else
        {
            if self.unit.is_convertible(unit)
            {
                Ok(Self { value: self.value * self.unit.multiplier / unit.multiplier(), unit })
            }
            else
            {
                Err(RuntimeUnitError::IncompatibleUnitConversion(format!("Could not convert from base units of {} to {}", self.unit.unit_string(), unit.unit_string())))
            }
        }
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

}
impl Debug for QuantityBase
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result 
    {
        write!(f, "{} {:?}", self.value(), self.definition())
    }
}
impl Mul<f64> for QuantityBase
{
    type Output = QuantityBase;

    fn mul(self, rhs: f64) -> Self::Output {
        Self{ value: self.value*rhs, unit: self.unit }
    }
}

impl Div<f64> for QuantityBase
{
    type Output = QuantityBase;

    fn div(self, rhs: f64) -> Self::Output {
        Self{ value: self.value/rhs, unit: self.unit }
    }
}
impl Mul<QuantityBase> for QuantityBase
{
    type Output = QuantityBase;

    fn mul(self, rhs: QuantityBase) -> Self::Output {
        Self{ value: self.value*rhs.value, unit: self.unit*rhs.unit }
    }
}
impl Div<QuantityBase> for QuantityBase
{
    type Output = QuantityBase;

    fn div(self, rhs: QuantityBase) -> Self::Output {
        Self{ value: self.value/rhs.value, unit: self.unit/rhs.unit }
    }
}
impl Add<QuantityBase> for QuantityBase
{
    type Output=QuantityBase;

    fn add(self, rhs: QuantityBase) -> Self::Output {
        Self { value: self.value + rhs.value, unit: self.unit }
    }
}
impl Sub<QuantityBase> for QuantityBase
{
    type Output=QuantityBase;

    fn sub(self, rhs: QuantityBase) -> Self::Output {
        Self { value: self.value - rhs.value, unit: self.unit }
    }
}

impl AddAssign for QuantityBase
{
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl SubAssign for QuantityBase
{
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

impl MulAssign for QuantityBase
{
    fn mul_assign(&mut self, rhs: Self) {
        self.value *= rhs.value;
        self.unit *= rhs.unit;
    }
}


impl DivAssign for QuantityBase
{
    fn div_assign(&mut self, rhs: Self) {
        self.value /= rhs.value;
        self.unit /= rhs.unit;
    }
}

impl<T:IsQuantity> Mul<T> for QuantityBase
{
    type Output = QuantityBase;
    fn mul(self, rhs: T) -> QuantityBase {
        QuantityBase{ value: self.value*rhs.value(), unit: self.definition()*rhs.definition() }
    }
}
impl<T:IsQuantity> Div<T> for QuantityBase
{
    type Output = QuantityBase;

    fn div(self, rhs: T) -> QuantityBase {
        QuantityBase{ value: self.value/rhs.value(), unit: self.definition()/rhs.definition() }
    }
}
///
/// This only compares magnitudes...
/// 
impl PartialOrd<QuantityBase> for QuantityBase
{
    fn partial_cmp(&self, other: &QuantityBase) -> Option<core::cmp::Ordering> {
        self.convert_unchecked(other.unit).partial_cmp(&other.value)
    }
}

impl PartialEq for QuantityBase
{
    fn eq(&self, other: &Self) -> bool {
        self.unit.base == other.unit.base && self.convert_unchecked(other.unit) == other.value
    }
}