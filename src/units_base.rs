use core::{fmt::Display,  ops::{Div, DivAssign, Mul, MulAssign}};
use bitfield_struct::bitfield;

use crate::errors::RuntimeUnitError;

#[bitfield(u64)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Hash)]
#[doc="Storage of primitives used to define a given unit."]
pub struct UnitBase
{
    #[bits(8)]
    meter: i8,
    #[bits(8)]
    second: i8,
    #[bits(8)]
    kilogram: i8,
    #[bits(8)]
    ampere: i8,
    #[bits(8)]
    candela: i8,
    #[bits(8)]
    kelvin: i8,
    #[bits(8)]
    mole: i8,
    #[bits(8)]
    _unused: i8,
}
impl UnitBase
{   
    /// A method to get the power of a given unit component.    
    fn get_element(&self, index: u8) -> i8
    {
        match index
        {
            0 => self.meter(),
            1 => self.second(),
            2 => self.kilogram(),
            3 => self.ampere(),
            4 => self.candela(),
            5 => self.kelvin(),
            6 => self.mole(),
            _ => panic!("Unsupported index {} provided to get_element", index)
        }
    }
    /// Get unit name for a given index from the `get_element` method.
    fn unit_name(&self, index: u8) -> &'static str
    {
        match index
        {
            0 => "m",
            1 => "s",
            2 => "kg",
            3 => "A",
            4 => "cd",
            5 => "K",
            6 => "mol",
            _ => panic!("Unsupported index {} provided to get_element", index)
        }
    }

    
}
impl Display for UnitBase
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result 
    {
        let numerator = (0..7).map(|i| 
            {
                let value =  self.get_element(i as u8);
                if value.is_positive()
                {
                    if value > 1
                    {
                        format!("{}^{}", self.unit_name(i as u8), value).to_owned() 
                    }
                    else
                    {
                        format!("{}", self.unit_name(i as u8)).to_owned()
                    }
                }
                else
                {
                    "".to_owned()
                }
            }).filter(|x|!x.is_empty()).collect::<Vec<String>>().join("*");
            let denominator = (0..7).map(|i| 
                {
                    let value =  self.get_element(i as u8);
                    if value.is_negative()
                    {
                        if value < -1
                        {
                            format!("{}^{}", self.unit_name(i as u8), -value).to_owned() 
                        }
                        else
                        {
                            format!("{}", self.unit_name(i as u8)).to_owned()
                        }
                    }
                    else
                    {
                        "".to_owned()
                    }
                }).filter(|x|!x.is_empty()).collect::<Vec<String>>().join("*");
                
            let result = if numerator.is_empty() 
            {
                if denominator.is_empty()
                {
                    "dimensionless".to_owned()
                }
                else
                {
                    format!("1/{denominator}")
                }            
            }
            else
            {
                if denominator.is_empty()
                {
                    numerator
                }
                else
                {
                    if denominator.contains("*")
                    {
                        format!("{numerator}/({denominator})")
                    }
                    else
                    {
                        format!("{numerator}/{denominator}")
                    }
                }
            };
            write!(f, "{result}")
    }
}

impl Mul for UnitBase
{
    type Output = UnitBase;

    fn mul(self, rhs: Self) -> Self::Output {        
        UnitBase::new().
        with_meter(self.meter() + rhs.meter()).
        with_second(self.second() + rhs.second()).
        with_kilogram(self.kilogram() + rhs.kilogram()).
        with_ampere(self.ampere() + rhs.ampere()).
        with_candela(self.candela() + rhs.candela()).
        with_kelvin(self.kelvin() + rhs.kelvin()).
        with_mole(self.mole() + rhs.mole())
    }
}
impl MulAssign for UnitBase
{
    fn mul_assign(&mut self, rhs: Self) {
        self.set_meter(self.meter() + rhs.meter());
        self.set_second(self.second() + rhs.second());
        self.set_kilogram(self.kilogram() + rhs.kilogram());
        self.set_ampere(self.ampere() + rhs.ampere());
        self.set_candela(self.candela() + rhs.candela());
        self.set_kelvin(self.kelvin() + rhs.kelvin());
        self.set_mole(self.mole() + rhs.mole());
    }
}

impl Div for UnitBase
{
    type Output = UnitBase;

    fn div(self, rhs: Self) -> Self::Output {
        UnitBase::new().
        with_meter(self.meter() - rhs.meter()).
        with_second(self.second() - rhs.second()).
        with_kilogram(self.kilogram() - rhs.kilogram()).
        with_ampere(self.ampere() - rhs.ampere()).
        with_candela(self.candela() - rhs.candela()).
        with_kelvin(self.kelvin() - rhs.kelvin()).
        with_mole(self.mole() - rhs.mole())
    }
}

impl DivAssign for UnitBase
{
    fn div_assign(&mut self, rhs: Self) {
        self.set_meter(self.meter() - rhs.meter());
        self.set_second(self.second() - rhs.second());
        self.set_kilogram(self.kilogram() - rhs.kilogram());
        self.set_ampere(self.ampere() - rhs.ampere());
        self.set_candela(self.candela() - rhs.candela());
        self.set_kelvin(self.kelvin() - rhs.kelvin());
        self.set_mole(self.mole() - rhs.mole());
    }
}

impl UnitBase
{
    #[allow(unused)]
    pub(crate) const fn new_length() -> Self
    {
        UnitBase::new().with_meter(1)
    }
    #[allow(unused)]
    pub(crate) const fn new_mass() -> Self
    {
        UnitBase::new().with_kilogram(1)
    }
    #[allow(unused)]
    pub(crate) const fn new_time() -> Self
    {
        UnitBase::new().with_second(1)
    }
    #[allow(unused)]
    pub(crate) const fn new_current() -> Self
    {
        UnitBase::new().with_ampere(1)
    }
    #[allow(unused)]
    pub(crate) const fn new_temperature() -> Self
    {
        UnitBase::new().with_kelvin(1)
    }
    #[allow(unused)]
    pub(crate) const fn new_luminance() -> Self
    {
        UnitBase::new().with_candela(1)
    }

    #[allow(unused)]
    pub(crate) const fn dimensionless() -> Self
    {
        UnitBase::new()
    }
    
    pub(crate) fn powi(&self, power: i8) -> Self
    {
        UnitBase::new().
        with_meter(self.meter()*power).
        with_second(self.second()*power).
        with_kilogram(self.kilogram()*power).
        with_ampere(self.ampere()*power).
        with_candela(self.candela()*power).
        with_kelvin(self.kelvin()*power).
        with_mole(self.mole()*power)
    }
}

/// Storage for a single unit and its multiplier to convert it to the base unit.
#[derive(PartialEq, Debug, Clone, Copy)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnitDefinition
{
    pub(crate) base: UnitBase,
    pub(crate) multiplier: f64
}
impl Eq for UnitDefinition{}
impl UnitDefinition
{    
    #[doc="Create a new `UnitDefinition` manually by specifying powers of each base unit, as well as the multiplier."]      
    pub fn new(multiplier: f64, meter: i8, kilogram: i8, second: i8, ampere: i8, kelvin: i8, mole: i8, candela: i8) -> Self
    {
        UnitDefinition { base: UnitBase::new().with_meter(meter).with_kilogram(kilogram).with_second(second).with_ampere(ampere).with_kelvin(kelvin).with_mole(mole).with_candela(candela), multiplier }
    }
    #[doc="Returns a dimensionless `UnitDefinition`."]      
    pub fn dimensionless() -> Self
    {
        UnitDefinition { base: UnitBase::default(), multiplier: 1.0 }
    }
    #[doc="Check whether this unit can be converted to a a given `unit`."]      
    pub fn is_convertible(&self, unit: UnitDefinition) -> bool
    {
        self.base == unit.base
    }
    #[doc="Raise a unit to an integer power."]
    pub fn powi(&self, power: i8) -> UnitDefinition
    {
        UnitDefinition { base: self.base.powi(power), multiplier: self.multiplier.powi(power as i32) }
    }
    #[doc="Retrieve multiplier that converts this unit to its base quantity."]
    pub fn multiplier(&self) -> f64
    {
        self.multiplier
    }
    #[doc="Get the string representation of the base unit."]
    pub fn unit_string(&self) -> String
    {
        self.base.to_string()
    }
    #[doc="Approximate equality for two units, given some relative error `rel_error`"]
    pub fn approx_eq(&self, other: UnitDefinition, rel_error: f64) -> bool
    {
        other.base == self.base && if self.multiplier == 0.0 { (self.multiplier-other.multiplier).abs()} else { (1.0-other.multiplier/self.multiplier).abs() } <= rel_error 
    }
    #[doc="Compute the conversion factor required to convert current `UnitDefinition` to another"]
    pub fn try_convert(&self, unit: Self) -> Result<f64, RuntimeUnitError>
    {
        if unit.base == self.base
        {
            Ok(self.convert_unchecked(unit))
        }
        else
        {
            Err(RuntimeUnitError::IncompatibleUnitConversion(format!("Could not convert from base units of {} to {}", self.unit_string(), unit.unit_string())))
        }
    }
    #[doc="Compute the conversion factor required to convert current `UnitDefinition` to another"]
    pub fn convert_unchecked(&self, unit: Self) -> f64
    {
        if *self == unit
        {
            1.0
        }
        else 
        {
            self.multiplier / unit.multiplier()
        }
    }
}
impl Mul<UnitDefinition> for UnitDefinition
{
    type Output = UnitDefinition;

    fn mul(self, rhs: UnitDefinition) -> Self::Output {
        UnitDefinition{
            multiplier: self.multiplier*rhs.multiplier,
            base: self.base*rhs.base
        }
    }
}
impl Div<UnitDefinition> for UnitDefinition
{
    type Output=UnitDefinition;

    fn div(self, rhs: UnitDefinition) -> Self::Output {
        UnitDefinition{
            multiplier: self.multiplier/rhs.multiplier,
            base: self.base/rhs.base
        }
    }
}

impl DivAssign for UnitDefinition
{
    fn div_assign(&mut self, rhs: UnitDefinition) {
        self.multiplier /= rhs.multiplier;
        self.base /= rhs.base;
    }
}

impl MulAssign for UnitDefinition
{
    fn mul_assign(&mut self, rhs: Self) {
        self.multiplier *= rhs.multiplier;
        self.base *=  rhs.base
    }
}
///
/// Internal datatype to map UOM dimensions to runtime units. 
///
#[derive(Copy, Clone)]
#[repr(i8)]
#[allow(unused)]
pub(crate) enum UOMDimensions
{
    N5 = -5,
    N4 = -4,
    N3 = -3,
    N2 = -2,
    N1 = -1,
    Z0 = 0,
    P1 = 1,
    P2 = 2,
    P3 = 3,
    P4 = 4,
    P5 = 5
}

impl UOMDimensions
{
    #[inline(always)]
    pub(crate) const fn to_unit_base(value: (UOMDimensions, UOMDimensions, UOMDimensions, UOMDimensions, UOMDimensions, UOMDimensions, UOMDimensions)) -> UnitBase
    {
        UnitBase::new().
        with_meter(value.0 as i8).
        with_kilogram(value.1 as i8).
        with_second(value.2 as i8).
        with_ampere(value.3 as i8).
        with_kelvin(value.4 as i8).
        with_mole(value.5 as i8).
        with_candela(value.6 as i8)
    }
}