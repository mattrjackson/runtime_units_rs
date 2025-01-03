#[macro_export]
macro_rules! impl_quantity_vec_ops {
    ($quantity:ident) =>
    {
        paste::paste!
        {
        use core::ops::{Mul, Div, Add, Sub, AddAssign, SubAssign, MulAssign, DivAssign };
        impl Div<f64> for [<$quantity Vec>]
        {
            type Output = [<$quantity Vec>];

            fn div(self, rhs: f64) -> Self::Output {
                let mut result = self.clone();
                for val in result.values.as_mut_slice()
                {
                *val /= rhs;
                }
                result
            }
        }


        impl Mul<f64> for [<$quantity Vec>]
        {
            type Output = [<$quantity Vec>];

            fn mul(self, rhs: f64) -> Self::Output {
                let mut result = self.clone();
                for val in result.values.as_mut_slice()
                {
                *val *= rhs;
                }
                result
            }
        }
        impl DivAssign<f64> for [<$quantity Vec>]
        {

            fn div_assign(&mut self, rhs: f64) {
                for val in self.values.as_mut_slice()
                {
                *val /= rhs;
                }
            }
        }
        impl MulAssign<f64> for [<$quantity Vec>]
        {

            fn mul_assign(&mut self, rhs: f64) {
                for val in self.values.as_mut_slice()
                {
                *val *= rhs;
                }
            }
        }
        use $crate::traits::Unit;
        impl AddAssign<[<$quantity Vec>]> for [<$quantity Vec>]
        {

            fn add_assign(&mut self, rhs: [<$quantity Vec>]) {
                if rhs.values.len() != self.values.len()
                {
                    panic!("Slice dimensions do not match: {} != {}", rhs.values.len(), self.values.len());
                }
                let factor = rhs.unit.definition().convert_unchecked(self.unit.definition());
                for (val, &rhs) in self.values.as_mut_slice().iter_mut().zip(rhs.values.as_slice())
                {
                *val += rhs*factor;
                }
            }
        }
        impl SubAssign<[<$quantity Vec>]> for [<$quantity Vec>]
        {

            fn sub_assign(&mut self, rhs: [<$quantity Vec>]) {
                if rhs.values.len() != self.values.len()
                {
                    panic!("Slice dimensions do not match: {} != {}", rhs.values.len(), self.values.len());
                }
                let factor = rhs.unit.convert_unchecked(self.unit);
                for (val, &rhs) in self.values.as_mut_slice().iter_mut().zip(rhs.values.as_slice())
                {
                *val -= rhs*factor;
                }
            }
        }
    }
    }
}

#[macro_export]
macro_rules! impl_quantity_array_ops {
    ($quantity:ident) =>
    {
        paste::paste!
        {
        impl<const N: usize> Div<f64> for [<$quantity Array>]<N>
        {
            type Output = [<$quantity Array>]<N>;

            fn div(self, rhs: f64) -> Self::Output {
                let mut result = self.clone();
                for val in result.values.as_mut_slice()
                {
                *val /= rhs;
                }
                result
            }
        }


        impl<const N: usize> Mul<f64> for [<$quantity Array>]<N>
        {
            type Output = [<$quantity Array>]<N>;

            fn mul(self, rhs: f64) -> Self::Output {
                let mut result = self.clone();
                for val in result.values.as_mut_slice()
                {
                *val *= rhs;
                }
                result
            }
        }
        impl<const N: usize> DivAssign<f64> for [<$quantity Array>]<N>
        {

            fn div_assign(&mut self, rhs: f64) {
                for val in self.values.as_mut_slice()
                {
                *val /= rhs;
                }
            }
        }
        impl<const N: usize> MulAssign<f64> for [<$quantity Array>]<N>
        {

            fn mul_assign(&mut self, rhs: f64) {
                for val in self.values.as_mut_slice()
                {
                *val *= rhs;
                }
            }
        }
        impl<const N: usize> AddAssign<[<$quantity Array>]<N>> for [<$quantity Array>]<N>
        {

            fn add_assign(&mut self, rhs: [<$quantity Array>]<N>) {
                if rhs.values.len() != self.values.len()
                {
                    panic!("Slice dimensions do not match: {} != {}", rhs.values.len(), self.values.len());
                }
                let factor = rhs.unit.definition().convert_unchecked(self.unit.definition());
                for (val, &rhs) in self.values.as_mut_slice().iter_mut().zip(rhs.values.as_slice())
                {
                *val += rhs*factor;
                }
            }
        }
        impl<const N: usize> SubAssign<[<$quantity Array>]<N>> for [<$quantity Array>]<N>
        {

            fn sub_assign(&mut self, rhs: [<$quantity Array>]<N>) {
                if rhs.values.len() != self.values.len()
                {
                    panic!("Slice dimensions do not match: {} != {}", rhs.values.len(), self.values.len());
                }
                let factor = rhs.unit.convert_unchecked(self.unit);
                for (val, &rhs) in self.values.as_mut_slice().iter_mut().zip(rhs.values.as_slice())
                {
                *val -= rhs*factor;
                }
            }
        }
    }
    }
}

#[macro_export]
macro_rules! create_multivalue_quantities {
    ($quantity:ident) =>
    {       
        #[cfg(feature="serde")]
        use serde_with::serde_as;      
        use std::ops::{Deref, DerefMut};  
        paste::paste!
        {
        #[cfg_attr(feature="serde", cfg_eval::cfg_eval, serde_as)]
        #[derive(Copy, Clone, Debug, PartialEq)]
        #[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(feature="utoipa", derive(ToSchema))]
        #[doc = "Array storage for a series of values and [`" [<$quantity Unit>]"`]."]
        pub struct [<$quantity Array>]<const N: usize>
        {
            pub(crate) unit: [<$quantity Unit>],
            #[cfg_attr(feature="serde", serde_as(as = "[_; N]"))]
            pub(crate) values: [f64; N],
        }

        impl<const N: usize> Default for [<$quantity Array>]<N>
        {
            fn default() -> Self {
                [<$quantity Array>]{unit: [<$quantity Unit>]::default(), values: [0.0; N]}
            }
        }

        impl<const N: usize> Deref for [<$quantity Array>]<N>
        {
            type Target = [f64; N];
            fn deref(&self) -> &[f64; N] {
                &self.values
            }        
        }

        impl<const N: usize> DerefMut for  [<$quantity Array>]<N>
        {
            fn deref_mut(&mut self) -> &mut [f64; N] {
                &mut self.values
            }
        } 

        #[derive(Default, Clone, Debug, PartialEq)]
        #[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(feature="utoipa", derive(ToSchema))]
        #[doc = "Vector storage for a series of values and [`" [<$quantity Unit>]"`]."]
        pub struct [<$quantity Vec>]
        {
            pub(crate) unit: [<$quantity Unit>],
            pub(crate) values: Vec<f64>
        }

        impl Deref for [<$quantity Vec>]
        {
            type Target = Vec<f64>;
            fn deref(&self) -> &Vec<f64> {
                &self.values
            }        
        }
        impl DerefMut for [<$quantity Vec>]
        {
            fn deref_mut(&mut self) -> &mut Vec<f64> {
                &mut self.values
            }
        }

        impl crate::traits::FixedSliceQuantity<[<$quantity Unit>], f64> for [<$quantity Vec>]
        {
            fn unit(&self) -> [<$quantity Unit>] {
                self.unit
            }

            fn values(&self) -> &[f64] {
                &self.values.as_slice()
            }

            fn values_mut(&mut self) -> &mut [f64] {
                self.values.as_mut_slice()
            }

            fn len(&self) -> usize {
                self.values.len()
            }

            fn convert(&self, unit: [<$quantity Unit>]) -> Self
            {
                let mut result = self.clone();
                result.convert_mut(unit);
                return result;
            }

            #[inline]
            fn convert_mut(&mut self, unit: [<$quantity Unit>]) {
                let factor = self.unit.definition().convert_unchecked(unit.definition());
                for val in self.values.as_mut_slice().iter_mut()
                {
                    *val *= factor;
                }
                self.unit = unit;
            }

            #[inline]
            fn try_convert(&self, unit: $crate::Units) -> Result<Self, RuntimeUnitError> where Self: Sized
            {
                let destination_unit: [<$quantity Unit>] = unit.try_into()?;
                Ok(self.convert(destination_unit))
            }
        }
        impl<const N: usize>  crate::traits::FixedSliceQuantity<[<$quantity Unit>], f64> for [<$quantity Array>]<N>
        {
            fn unit(&self) -> [<$quantity Unit>] {
                self.unit
            }

            fn values(&self) -> &[f64] {
                &self.values.as_slice()
            }

            fn values_mut(&mut self) -> &mut [f64] {
                self.values.as_mut_slice()
            }

            fn len(&self) -> usize {
                self.values.len()
            }

            fn convert(&self, unit: [<$quantity Unit>]) -> Self 
            {
                let mut result = self.clone();
                result.convert_mut(unit);
                return result;
            }

            #[inline]
            fn convert_mut(&mut self, unit: [<$quantity Unit>]) {
                let factor = self.unit.definition().convert_unchecked(unit.definition());
                for val in self.values.as_mut_slice().iter_mut()
                {
                    *val *= factor;
                }            
                self.unit = unit;
            }

            #[inline]
            fn try_convert(&self, unit: $crate::Units) -> Result<Self, RuntimeUnitError> where Self: Sized
            {
                let destination_unit: [<$quantity Unit>] = unit.try_into()?;                   
                Ok(self.convert(destination_unit))      
            }
        }

        }
    }
}

#[macro_export]
macro_rules! create_multivalue_quantities_vec_enum {
    ($($quantity:ident),+) =>
    {
        paste::paste!{
            #[derive(Clone, Debug, PartialEq)]
            #[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
            #[cfg_attr(feature="utoipa", derive(ToSchema))]
            ///
            /// A wrapper to hold all QuantitiesVec supported by this library. It is analogous to `Units``, 
            /// but when combined with the `serde` feature flag, can serve as a way to serialize a quantity, not just the unit. 
            /// 
            pub enum QuantitiesVec
            {
                $(
                    #[cfg(any(feature = "" $quantity, feature="All"))]   
                    #[cfg_attr(feature="utoipa", schema(title = "" $quantity))]
                    $quantity([<$quantity Vec>]),
                )+
            }
            impl QuantitiesVec
            {                
                /// Get the `Units` enumeration associated with a given `QuantitiesVec` enumeration.
                pub fn unit(&self) -> Units
                {
                    use crate::traits::FixedSliceQuantity;
                    match self
                    {
                        $(
                            #[cfg(any(feature = "" $quantity, feature="All"))]                               
                            QuantitiesVec::$quantity(x)=> $crate::Units::from(x.unit()),
                        )+
                    }
                }
                /// Try to convert to the unit specified by a given `Units` enumeration.
                pub fn try_convert(&self, unit: Units) -> Result<QuantitiesVec, RuntimeUnitError>
                {   
                    use crate::traits::FixedSliceQuantity;
                    match self
                    {
                        $(
                            #[cfg(any(feature = "" $quantity, feature="All"))]   
                            QuantitiesVec::$quantity(x)=> {
                                Ok(QuantitiesVec::$quantity(x.try_convert(unit)?))
                            },
                        )+
                    }
                }

                /// Create a new quantity from a given value and unit
                pub fn new(value: Vec<f64>, unit: Units) -> QuantitiesVec
                {
                    match unit
                    {
                        $(
                            #[cfg(any(feature = "" $quantity, feature="All"))]   
                            Units::$quantity(x)=> {                                
                                QuantitiesVec::$quantity([<$quantity Vec>]::new(value, x))
                            },
                        )+
                    }
                }
                /// Get the value associated with quantity.
                pub fn values(&self) -> Vec<f64>
                {
                    use crate::vector_quantity::VecQuantity;
                    VecQuantity::from(self.clone()).values
                }
            }
            /// A means to create a default quantity with a given set of units.
            impl From<Units> for QuantitiesVec
            {
                
                fn from(value: Units) -> Self 
                {    
                    
                    match value
                    {
                        $(
                            
                            #[cfg(any(feature = "" $quantity, feature="All"))]   
                            Units::$quantity(x)=>
                            {
                                QuantitiesVec::new(vec![0.0], x.into())
                            },
                        )+
                    }                
                    
                }
            }
            impl From<QuantitiesVec> for crate::vector_quantity::VecQuantity
            {
                fn from(value: QuantitiesVec) -> Self {
                    match value
                    {
                        $(
                            #[cfg(any(feature = "" $quantity, feature="All"))]   
                            QuantitiesVec::$quantity(x)=>crate::vector_quantity::VecQuantity { values: x.values, unit: UnitDefinition{multiplier: x.unit.multiplier(), base: [<$quantity:snake>]::[<$quantity:upper _UNIT_BASE>]} },
                        )+
                    }
                }
            }
            impl ToString for QuantitiesVec
            {
                fn to_string(&self) -> String 
                {
                    match self
                    {
                        $(
                            #[cfg(any(feature = "" $quantity, feature="All"))]   
                            QuantitiesVec::$quantity(x) => format!("{:?} {}", x.values, x.unit.abbreviation()),
                        )+
                    }                        
                }
            }
        }
    }
}

#[macro_export]
macro_rules! create_multivalue_quantities_array_enum {
    ($($quantity:ident),+) =>
    {
        paste::paste!{
            #[derive(Copy, Clone, Debug, PartialEq)]
            #[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
            ///
            /// A wrapper to hold all QuantitiesArray supported by this library. It is analogous to `Units``, 
            /// but when combined with the `serde` feature flag, can serve as a way to serialize a quantity, not just the unit. 
            /// 
            pub enum QuantitiesArray<const N: usize>
            {
                $(
                    #[cfg(any(feature = "" $quantity, feature="All"))]   
                    $quantity([<$quantity Array>]<N>),
                )+
            }
            impl<const N: usize> QuantitiesArray<N>
            {                
                /// Get the `Units` enumeration associated with a given `QuantitiesArray` enumeration.
                pub fn unit(&self) -> Units
                {
                    match self
                    {
                        $(
                            #[cfg(any(feature = "" $quantity, feature="All"))]                               
                            QuantitiesArray::$quantity(x)=> $crate::Units::from(x.unit),
                        )+
                    }
                }
                /// Try to convert to the unit specified by a given `Units` enumeration.
                pub fn try_convert(&self, unit: Units) -> Result<QuantitiesArray<N>, RuntimeUnitError>
                {   
                    use crate::traits::FixedSliceQuantity;
                    match self
                    {
                        $(
                            #[cfg(any(feature = "" $quantity, feature="All"))]   
                            QuantitiesArray::$quantity(x)=> {
                                Ok(QuantitiesArray::$quantity(x.try_convert(unit)?))
                            },
                        )+
                    }
                }

                /// Create a new quantity from a given value and unit
                pub fn new(value: [f64; N], unit: Units) -> QuantitiesArray<N>
                {
                    match unit
                    {
                        $(
                            #[cfg(any(feature = "" $quantity, feature="All"))]   
                            Units::$quantity(x)=> {                                
                                QuantitiesArray::$quantity([<$quantity Array>]{values: value, unit:x})
                            },
                        )+
                    }
                }
                /// Get the value associated with quantity.
                pub fn values(&self) -> [f64; N]
                {
                    use crate::array_quantity::ArrayQuantity;
                    ArrayQuantity::from(*self).values
                }
            }
            /// A means to create a default quantity with a given set of units.
            impl<const N: usize> From<Units> for QuantitiesArray<N>
            {
                fn from(value: Units) -> Self 
                {    
                    match value
                    {
                        $(
                            #[cfg(any(feature = "" $quantity, feature="All"))]   
                            Units::$quantity(x)=> QuantitiesArray::new([0.0; N], x.into()),
                        )+
                    }                
                    
                }
            }
            impl<const N: usize> From<QuantitiesArray<N>> for crate::array_quantity::ArrayQuantity<N>
            {
                fn from(value: QuantitiesArray<N>) -> Self {
                    use crate::array_quantity::ArrayQuantity;
                    match value
                    {
                        $(
                            #[cfg(any(feature = "" $quantity, feature="All"))]   
                            QuantitiesArray::$quantity(x)=>ArrayQuantity { values: x.values, unit: UnitDefinition{multiplier: x.unit.multiplier(), base: [<$quantity:snake>]::[<$quantity:upper _UNIT_BASE>]} },
                        )+
                    }
                }
            }
            impl<const N: usize> ToString for QuantitiesArray<N>
            {
                fn to_string(&self) -> String 
                {
                    match self
                    {
                        $(
                            #[cfg(any(feature = "" $quantity, feature="All"))]   
                            QuantitiesArray::$quantity(x) => format!("{:?} {}", x.values, x.unit.abbreviation()),
                        )+
                    }                        
                }
            }
        }
    }
}