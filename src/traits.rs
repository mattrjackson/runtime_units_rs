use crate::{errors::RuntimeUnitError, units_base::{UnitBase, UnitDefinition}};
use core::ops::{Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign };

///
/// Trait that implements conversion of an arbitrary quantity into another
/// 
pub trait Quantity where Self: Sized + Div<f64> + Mul<f64> + DivAssign<f64> + MulAssign<f64> + Div<Self> + Mul<Self> + AddAssign<Self> + SubAssign<Self> + DivAssign<Self>
{
    /// Return unit associated with this quantity
    fn unit(&self) -> UnitDefinition;
    /// Try to convert from this unit to another (creates a copy)
    fn try_convert(&mut self, unit: UnitDefinition) -> Result<Self, RuntimeUnitError>;
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
    /// Convert from this unit to another (creates a copy). No validation of base unit is made.
    fn convert(&self, unit: UnitType) -> Self;
    /// Convert from this unit to another (modifies current quantity). No validation of base unit is made.
    fn convert_mut(&mut self, unit: UnitType);
}

///
/// A trait to define storage that provides iterator over an `Element` of a given type.
/// 
pub trait Slice<Element>
{
    fn as_mut_slice(&mut self) -> &mut [Element];
    fn as_slice(&self) -> &[Element];
    fn len(&self) -> usize;
}


///
/// Trait to define a type of unit (e.g. `LengthUnit`,`MassUnit`)
/// 
pub trait Unit where Self:Sized
{
    /// Return unit definition for this Unit Type
    fn definition(&self) -> UnitDefinition;

    /// Base unit definition
    fn base(&self) -> UnitBase;

    /// Convert from this unit to another. No validation of base unit is made.
    fn try_convert(&self, unit: UnitDefinition) -> Result<UnitDefinition, RuntimeUnitError>
    {
        let definition = self.definition();
        if definition.is_convertible(unit)
        {
            Ok(UnitDefinition { base: definition.base, multiplier: unit.multiplier() / definition.multiplier() })
        }
        else
        {
            Err(RuntimeUnitError::IncompatibleUnitConversion(format!("Could not convert from base units of {} to {}", definition.unit_string(), unit.unit_string())))
        }
    }
    /// Try converting from this unit to another.
    fn convert_unchecked(&self, unit: Self) -> UnitDefinition
    {
        let definition = self.definition();
        UnitDefinition { base: definition.base, multiplier: unit.definition().multiplier() / definition.multiplier() }
    }
}