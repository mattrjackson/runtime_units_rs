use crate::{errors::RuntimeUnitError, units_base::UnitDefinition};
use core::ops::{AddAssign, Div, DivAssign, Mul, MulAssign, SubAssign };
use std::ops::{Deref, DerefMut};


#[derive(Clone)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[doc ="Data structure to hold a unit and array of data"]
pub struct VecQuantity
{
    pub unit: UnitDefinition,
    pub values: Vec<f64>
}

impl Deref for VecQuantity
{
    type Target = Vec<f64>;
    fn deref(&self) -> &Vec<f64> {
        &self.values
    }
}
impl DerefMut for VecQuantity
{
    fn deref_mut(&mut self) -> &mut Vec<f64> {
        &mut self.values
    }
}

// Defining ArbitraryQuantity for our VecQuantity
impl crate::traits::ArbitraryQuantity for VecQuantity
{
    fn unit(&self) -> UnitDefinition {
        self.unit
    }

    fn convert(&self, unit: UnitDefinition) -> Self 
    {
        let mut result = self.clone();
        result.convert_mut(unit);
        return result;
    }

    fn convert_mut(&mut self, unit: UnitDefinition) {
        let factor = self.unit.convert_unchecked(unit);
        for val in self.values.as_mut_slice().iter_mut()
        {
            *val *= factor;
        }            
    }
    
    fn try_convert(&self, unit: UnitDefinition) -> Result<Self, crate::errors::RuntimeUnitError> {
        if self.unit.base != unit.base
        {
            Err(RuntimeUnitError::IncompatibleUnitConversion(format!("Could not convert from base units of {} to {}", self.unit.unit_string(), unit.unit_string())))
        }
        else
        {
            Ok(self.convert(unit))
        }

    }
    
    fn try_convert_mut(&mut self, unit: UnitDefinition) -> Result<(), crate::errors::RuntimeUnitError> {
        if self.unit.base != unit.base
        {
            Err(RuntimeUnitError::IncompatibleUnitConversion(format!("Could not convert from base units of {} to {}", self.unit.unit_string(), unit.unit_string())))
        }
        else
        {
            Ok(self.convert_mut(unit))
        }
    }
    
    fn unit_mut(&mut self) -> &mut UnitDefinition {
        &mut self.unit
    }
}




impl Div<f64> for VecQuantity
{
    type Output = VecQuantity;

    fn div(self, rhs: f64) -> Self::Output {
        let mut result = self.clone();
        for val in result.values.as_mut_slice()
        {  
           *val /= rhs;
        }
        result
    }
}
impl Div<VecQuantity> for VecQuantity
{
    type Output = VecQuantity;

    fn div(self, rhs: VecQuantity) -> Self::Output {
        let mut result = self.clone();
        if rhs.values.len() != self.values.len()
        {
            panic!("Slice dimensions do not match: {} != {}", rhs.values.len(), self.values.len());
        }
        for (val, &rhs) in result.values.as_mut_slice().iter_mut().zip(rhs.values.as_slice())
        {  
           *val /= rhs;
        }
        result
    }
}
impl Mul<VecQuantity> for VecQuantity
{
    type Output = VecQuantity;

    fn mul(self, rhs: VecQuantity) -> Self::Output {
        let mut result = self.clone();
        if rhs.values.len() != self.values.len()
        {
            panic!("Slice dimensions do not match: {} != {}", rhs.values.len(), self.values.len());
        }
        for (val, &rhs) in result.values.as_mut_slice().iter_mut().zip(rhs.values.as_slice())
        {  
           *val *= rhs;
        }
        result
    }
}
impl Mul<f64> for VecQuantity
{
    type Output = VecQuantity;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut result = self.clone();
        for val in result.values.as_mut_slice()
        {  
           *val *= rhs;
        }
        result
    }
}
impl DivAssign<f64> for VecQuantity
{

    fn div_assign(&mut self, rhs: f64) {        
        for val in self.values.as_mut_slice()
        {  
           *val /= rhs;
        }
    }
}
impl MulAssign<f64> for VecQuantity
{

    fn mul_assign(&mut self, rhs: f64) {        
        for val in self.values.as_mut_slice()
        {  
           *val *= rhs;
        }
    }
}
impl DivAssign<VecQuantity> for VecQuantity
{

    fn div_assign(&mut self, rhs: VecQuantity) {            
        if rhs.values.len() != self.values.len()
        {
            panic!("Slice dimensions do not match: {} != {}", rhs.values.len(), self.values.len());
        }
        for (val, &rhs) in self.values.as_mut_slice().iter_mut().zip(rhs.values.as_slice())
        {  
           *val /= rhs;
        }
    }
}
impl AddAssign<VecQuantity> for VecQuantity
{

    fn add_assign(&mut self, rhs: VecQuantity) {            
        if rhs.values.len() != self.values.len()
        {
            panic!("Slice dimensions do not match: {} != {}", rhs.values.len(), self.values.len());
        }
        let factor = rhs.unit.try_convert(self.unit).unwrap_or_else(|_| panic!("Addition failed due to incompatible units `{}` and `{}`", self.unit, rhs.unit));
        for (val, &rhs) in self.values.as_mut_slice().iter_mut().zip(rhs.values.as_slice())
        {  
           *val += rhs*factor;
        }
    }
}
impl SubAssign<VecQuantity> for VecQuantity
{

    fn sub_assign(&mut self, rhs: VecQuantity) {            
        if rhs.values.len() != self.values.len()
        {
            panic!("Slice dimensions do not match: {} != {}", rhs.values.len(), self.values.len());
        }
        let factor = rhs.unit.try_convert(self.unit).unwrap_or_else(|_| panic!("Subtraction failed due to incompatible units `{}` and `{}`", self.unit, rhs.unit));
        for (val, &rhs) in self.values.as_mut_slice().iter_mut().zip(rhs.values.as_slice())
        {  
           *val -= rhs*factor;
        }
    }
}


