use crate::{errors::RuntimeUnitError, traits::Slice, units_base::UnitDefinition};
use core::ops::{AddAssign, Div, DivAssign, Mul, MulAssign, SubAssign };

// Slice implementations for Vec and arrays.

impl Slice<f64> for Vec<f64>
{
    fn as_mut_slice(&mut self) -> &mut [f64] {
        self.as_mut_slice()
    }

    fn as_slice(&self) -> &[f64] {
        self.as_slice()
    }

    fn len(&self) -> usize {
       Vec::len(&self)
    }
}
impl<const SIZE: usize> Slice<f64> for [f64; SIZE]
{
    fn as_mut_slice(&mut self) -> &mut [f64] {
        self
    }

    fn as_slice(&self) -> &[f64] {
        self
    }

    fn len(&self) -> usize {
        self.as_slice().len()
    }
}


#[derive(Clone)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[doc ="Data structure to hold a unit and array of data"]
pub struct SliceQuantity<T: Clone+Slice<f64>>
{
    pub unit: UnitDefinition,
    pub values: T
}

// Defining ArbitraryQuantity for our SliceQuantity
impl<T: Clone+Slice<f64>> crate::traits::ArbitraryQuantity for SliceQuantity<T>
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
        let factor = self.unit.convert_unchecked(unit).multiplier();
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




impl<T: Clone+Slice<f64>> Div<f64> for SliceQuantity<T>
{
    type Output = SliceQuantity<T>;

    fn div(self, rhs: f64) -> Self::Output {
        let mut result = self.clone();
        for val in result.values.as_mut_slice()
        {  
           *val /= rhs;
        }
        result
    }
}
impl<T: Clone+Slice<f64>> Div<SliceQuantity<T>> for SliceQuantity<T>
{
    type Output = SliceQuantity<T>;

    fn div(self, rhs: SliceQuantity<T>) -> Self::Output {
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
impl<T: Clone+Slice<f64>> Mul<SliceQuantity<T>> for SliceQuantity<T>
{
    type Output = SliceQuantity<T>;

    fn mul(self, rhs: SliceQuantity<T>) -> Self::Output {
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
impl<T: Clone+Slice<f64>> Mul<f64> for SliceQuantity<T>
{
    type Output = SliceQuantity<T>;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut result = self.clone();
        for val in result.values.as_mut_slice()
        {  
           *val *= rhs;
        }
        result
    }
}
impl<T: Clone+Slice<f64>> DivAssign<f64> for SliceQuantity<T>
{

    fn div_assign(&mut self, rhs: f64) {        
        for val in self.values.as_mut_slice()
        {  
           *val /= rhs;
        }
    }
}
impl<T: Clone+Slice<f64>> MulAssign<f64> for SliceQuantity<T>
{

    fn mul_assign(&mut self, rhs: f64) {        
        for val in self.values.as_mut_slice()
        {  
           *val *= rhs;
        }
    }
}
impl<T: Clone+Slice<f64>> DivAssign<SliceQuantity<T>> for SliceQuantity<T>
{

    fn div_assign(&mut self, rhs: SliceQuantity<T>) {            
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
impl<T: Clone+Slice<f64>> AddAssign<SliceQuantity<T>> for SliceQuantity<T>
{

    fn add_assign(&mut self, rhs: SliceQuantity<T>) {            
        if rhs.values.len() != self.values.len()
        {
            panic!("Slice dimensions do not match: {} != {}", rhs.values.len(), self.values.len());
        }
        let factor = rhs.unit.convert_unchecked(self.unit).multiplier;
        for (val, &rhs) in self.values.as_mut_slice().iter_mut().zip(rhs.values.as_slice())
        {  
           *val += rhs*factor;
        }
    }
}
impl<T: Clone+Slice<f64>> SubAssign<SliceQuantity<T>> for SliceQuantity<T>
{

    fn sub_assign(&mut self, rhs: SliceQuantity<T>) {            
        if rhs.values.len() != self.values.len()
        {
            panic!("Slice dimensions do not match: {} != {}", rhs.values.len(), self.values.len());
        }
        let factor = rhs.unit.convert_unchecked(self.unit).multiplier;
        for (val, &rhs) in self.values.as_mut_slice().iter_mut().zip(rhs.values.as_slice())
        {  
           *val -= rhs*factor;
        }
    }
}


