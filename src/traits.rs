use crate::{errors::RuntimeUnitError, units_base::{UnitBase, UnitDefinition}, Units};
use core::ops::{Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign };


pub(crate) trait IsScalarQuantity
{
    fn value(&self) -> f64;
    fn unit(&self) -> UnitDefinition;
}

///
/// Trait that implements conversion of an arbitrary quantity into another
/// 
pub trait ArbitraryQuantity where Self: Sized + Div<f64> + Mul<f64> + DivAssign<f64> + MulAssign<f64> + Div<Self> + Mul<Self> + AddAssign<Self> + SubAssign<Self> + DivAssign<Self>
{
    /// Return unit associated with this quantity
    fn unit(&self) -> UnitDefinition;
    /// Return unit associated with this quantity (mutable)
    fn unit_mut(&mut self) -> &mut UnitDefinition;
    /// Try to convert from this unit to another (creates a copy)
    fn try_convert(&self, unit: UnitDefinition) -> Result<Self, RuntimeUnitError>;    
    #[inline]
    /// Try to convert from this unit to the given `unit` (creates a copy)
    fn try_convert_unit(&self, unit: Units) -> Result<Self, RuntimeUnitError>
    {
        self.try_convert(unit.into())
    }
    /// Try to convert from this unit to another (modifies current quantity)
    fn try_convert_mut(&mut self, unit: UnitDefinition) -> Result<(), RuntimeUnitError>;    
    /// Convert from this unit to another (creates a copy). No validation of base unit is made.
    fn convert(&self, unit: UnitDefinition) -> Self;
    /// Convert from this unit to another (modifies current quantity). No validation of base unit is made.
    fn convert_mut(&mut self, unit: UnitDefinition);
}

///
/// Trait that implements conversion of a quantity within a given unit type (e.g. m->cm, kg->g)
/// 
pub trait FixedQuantity<UnitType: Unit> where Self: Sized + Div<f64> + Mul<f64> + DivAssign<f64> + MulAssign<f64> + AddAssign<Self> + SubAssign<Self> 
{
    /// Return unit associated with this quantity
    fn unit(&self) -> UnitType;    
    /// Return mutable unit associated with this quantity
    fn unit_mut(&mut self) -> &mut UnitType;    
    /// Convert from this unit to another (creates a copy). No validation of base unit is made.
    fn convert(&self, unit: UnitType) -> Self;
    /// Convert from this unit to another (modifies current quantity). No validation of base unit is made.
    fn convert_mut(&mut self, unit: UnitType);
    /// Try to convert from this unit to another (creates a copy)
    fn try_convert(&self, unit: crate::Units) -> Result<Self, RuntimeUnitError> where Self: Sized;
}

///
/// A trait to define storage that provides iterator over an `Element` of a given type.
/// 
pub trait Slice<Element>
{
    /// Get a mutable slice of the values.
    fn as_mut_slice(&mut self) -> &mut [Element];
    /// Get an immutable slice of the values.
    fn as_slice(&self) -> &[Element];
    /// Get the length of the values.
    fn len(&self) -> usize;
}

///
/// Trait that implements conversion of a slice quantity within a given unit type (e.g. m->cm, kg->g)
/// 
pub trait FixedSliceQuantity<UnitType: Unit, Element, Storage: Slice<Element>>
{
    /// Return unit associated with this quantity
    fn unit(&self) -> UnitType;
    /// Return values in quantity
    fn values(&self) -> &[Element];
    /// Return mutable values in quantity
    fn values_mut(&mut self) -> &mut [Element];
    /// Return number of values in quantity
    fn len(&self) -> usize;
    /// Convert a unit of one `UnitType` to another of the same type. No validation of base unit is made.
    fn convert(&self, unit: UnitType) -> Self;
    /// Mutate current quantity, convering  a unit of one `UnitType` to another of the same type. No validation of base unit is made.
    fn convert_mut(&mut self, unit: UnitType);
    /// Attempt to convert the unit given in `unit` to a `UnitType`. Base unit validation is made here.
    fn try_convert(&self, unit: Units) -> Result<Self, RuntimeUnitError> where Self: Sized;
}


///
/// Trait to define a type of unit (e.g. `LengthUnit`,`MassUnit`)
/// 
pub trait Unit where Self:Sized
{
    /// Return unit definition for this Unit Type
    fn definition(&self) -> UnitDefinition;

    /// Base unit definition
    fn base() -> UnitBase;

    /// Try to compute conversion factor from this unit to another.
    #[inline]
    fn try_convert(&self, unit: UnitDefinition) -> Result<f64, RuntimeUnitError>
    {
        let definition = self.definition();
        if definition.is_convertible(unit)
        {
            Ok(unit.multiplier() / definition.multiplier())
        }
        else
        {
            Err(RuntimeUnitError::IncompatibleUnitConversion(format!("Could not convert from base units of {} to {}", definition.unit_string(), unit.unit_string())))
        }
    }
    /// Compute conversion factor from this unit to another (no check of unit compatibility is made).
    #[inline]
    fn convert_unchecked(&self, unit: Self) -> f64
    {
        let definition = self.definition();
        unit.definition().multiplier() / definition.multiplier()
    }
}