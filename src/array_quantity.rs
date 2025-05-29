use crate::{errors::RuntimeUnitError, units_base::UnitDefinition};
use core::ops::{AddAssign, Div, DivAssign, Mul, MulAssign, SubAssign };
use std::ops::{Deref, DerefMut};
#[cfg(feature="serde")]
use serde_with::serde_as;
#[cfg_attr(feature="serde", cfg_eval::cfg_eval, serde_as)]
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[doc ="Data structure to hold a unit and array of data"]
pub struct ArrayQuantity<const N: usize>
{
    pub unit: UnitDefinition,
    #[cfg_attr(feature="serde", serde_as(as = "[_; N]"))]
    pub values: [f64; N]
}

impl<const N: usize> Deref for ArrayQuantity<N>
{
    type Target = [f64; N];
    fn deref(&self) -> &[f64; N] {
        &self.values
    }        
}

impl<const N: usize> DerefMut for ArrayQuantity<N>
{
    fn deref_mut(&mut self) -> &mut [f64; N] {
        &mut self.values
    }
}   
// Defining ArbitraryQuantity for our ArrayQuantity
impl<const N: usize> crate::traits::ArbitraryQuantity for ArrayQuantity<N>
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




impl<const N: usize> Div<f64> for ArrayQuantity<N>
{
    type Output = ArrayQuantity<N>;

    fn div(self, rhs: f64) -> Self::Output {
        let mut result = self.clone();
        for val in result.values.as_mut_slice()
        {  
           *val /= rhs;
        }
        result
    }
}
impl<const N: usize> Div<ArrayQuantity<N>> for ArrayQuantity<N>
{
    type Output = ArrayQuantity<N>;

    fn div(self, rhs: ArrayQuantity<N>) -> Self::Output {
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
impl<const N: usize> Mul<ArrayQuantity<N>> for ArrayQuantity<N>
{
    type Output = ArrayQuantity<N>;

    fn mul(self, rhs: ArrayQuantity<N>) -> Self::Output {
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
impl<const N: usize> Mul<f64> for ArrayQuantity<N>
{
    type Output = ArrayQuantity<N>;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut result = self.clone();
        for val in result.values.as_mut_slice()
        {  
           *val *= rhs;
        }
        result
    }
}
impl<const N: usize> DivAssign<f64> for ArrayQuantity<N>
{

    fn div_assign(&mut self, rhs: f64) {        
        for val in self.values.as_mut_slice()
        {  
           *val /= rhs;
        }
    }
}
impl<const N: usize> MulAssign<f64> for ArrayQuantity<N>
{

    fn mul_assign(&mut self, rhs: f64) {        
        for val in self.values.as_mut_slice()
        {  
           *val *= rhs;
        }
    }
}
impl<const N: usize> DivAssign<ArrayQuantity<N>> for ArrayQuantity<N>
{

    fn div_assign(&mut self, rhs: ArrayQuantity<N>) {            
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
impl<const N: usize> AddAssign<ArrayQuantity<N>> for ArrayQuantity<N>
{

    fn add_assign(&mut self, rhs: ArrayQuantity<N>) {            
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
impl<const N: usize> SubAssign<ArrayQuantity<N>> for ArrayQuantity<N>
{

    fn sub_assign(&mut self, rhs: ArrayQuantity<N>) {            
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


