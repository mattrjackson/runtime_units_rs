use core::{fmt::Display,  ops::{Div, DivAssign, Mul, MulAssign}};
use std::ops::{Add, Neg, Sub};
use bitfield_struct::bitfield;

use crate::errors::RuntimeUnitError;
type Ratio8 = num_rational::Ratio<i8>;
#[bitfield(u8, default=false)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Hash)]
pub struct Rational8 {
    #[bits(4)]
    pub numerator: i8,
    #[bits(4)]
    pub denominator: i8,
}
impl Default for Rational8
{
    #[inline]
    fn default() -> Self {
        Self::new().with_denominator(1).with_numerator(0)
    }
}

impl Display for Rational8
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_fraction()
        {
            write!(f, "{}/{}", self.numerator(), self.denominator())
        }
        else {
            write!(f, "{}", self.numerator())
        }
    }
}

impl From<Ratio8> for Rational8
{
    fn from(value: Ratio8) -> Self {
        Self::new().with_numerator(*value.numer()).with_denominator(*value.denom())
    }
}

impl Rational8
{
    ///
    /// Return 
    /// 
    #[inline]
    pub const fn unity() -> Self
    {
        Self::new().with_numerator(1).with_denominator(1)
    }

    ///
    /// Determine if this is represents a fraction
    /// 
    #[inline]
    pub fn is_fraction(&self) -> bool
    {
        self.denominator() > 1
    }

    ///
    /// Return the floating point representation of the power for this unit component
    /// 
    #[inline]
    pub fn fraction(&self) -> f64
    {
        self.numerator() as f64 / self.denominator() as f64
    }
}
impl From<i8> for Rational8
{
    fn from(value: i8) -> Self {
        Rational8::new().with_numerator(value).with_denominator(1)
    }
}
impl Into<f32> for Rational8
{
    #[inline]
    fn into(self) -> f32 {
        if self.denominator() == 1 
        {
            self.numerator() as f32
        }
        else
        {
            self.numerator() as f32 / self.denominator() as f32
        }
    }
}

impl Into<f64> for Rational8
{
    #[inline]
    fn into(self) -> f64 {
        if self.denominator() == 1 
        {
            self.numerator() as f64
        }
        else
        {
            self.numerator() as f64 / self.denominator() as f64
        }
    }
}
impl From<f64> for Rational8
{
    fn from(value: f64) -> Self {        
        Ratio8::approximate_float(value).unwrap().into()
    }
}
impl Add<Rational8> for Rational8
{
    type Output=Rational8;
    #[inline]
    fn add(self, rhs: Rational8) -> Self::Output {
        if self.is_fraction() || rhs.is_fraction()
        {
            Ratio8::approximate_float(self.fraction() + rhs.fraction()).unwrap().into()            
        }
        else
        {
            self.with_numerator(self.numerator() + rhs.numerator()).with_denominator(self.denominator())
        }
        
    }
}
impl Sub<Rational8> for Rational8
{
    type Output=Rational8;
    #[inline]
    fn sub(self, rhs: Rational8) -> Self::Output {
        if self.is_fraction() || rhs.is_fraction() || self.denominator() != rhs.denominator()
        {            
            Ratio8::approximate_float(self.fraction() - rhs.fraction()).unwrap().into()            
        }
        else
        {
            self.with_numerator(self.numerator() - rhs.numerator()).with_denominator(self.denominator())
        }
    }
}
impl Mul<i8> for Rational8
{
    type Output=Rational8;
    #[inline]
    fn mul(self, rhs: i8) -> Self::Output {        
        if self.is_fraction()
        {            
            Ratio8::approximate_float(self.fraction() * rhs as f64).unwrap().into()
        }
        else
        {
            self.with_numerator(self.numerator() * rhs).with_denominator(1)
        }
    }
}

impl Mul<f64> for Rational8
{
    type Output=Rational8;
    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {        
        let power = self.fraction() * rhs;
        Ratio8::approximate_float(power).unwrap().into()                    
    }
}

impl Neg for Rational8
{
    type Output=Rational8;

    fn neg(self) -> Self::Output {
        self.with_numerator(-self.numerator())
    }
}
#[bitfield(u64, default=false)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Hash)]
#[doc="Storage of primitives used to define a given unit."]
pub struct UnitBase
{    
    #[bits(8)]
    meter: Rational8, 
    #[bits(8)]
    second: Rational8,
    #[bits(8)]
    kilogram: Rational8,
    #[bits(8)]
    ampere: Rational8,
    #[bits(8)]
    candela: Rational8,
    #[bits(8)]
    kelvin: Rational8,
    #[bits(8)]
    mole: Rational8,
    #[bits(8)]
    _unused: Rational8,
}
impl Default for UnitBase
{
    fn default() -> Self 
    {
        UnitBase::new().
        with_meter(Rational8::default()).
        with_second(Rational8::default()).
        with_kilogram(Rational8::default()).    
        with_ampere(Rational8::default()).
        with_candela(Rational8::default()).
        with_kelvin(Rational8::default()).
        with_mole(Rational8::default())    
    }
}
impl UnitBase
{   
    /// A method to get the power of a given unit component.    
    fn get_element(&self, index: u8) -> Rational8
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
    ///
    /// Generate inverse of current unit (e.g. m -> 1/m).
    /// This is effectively setting the numerator to the negative of the current value
    pub fn inv(&self) -> Self
    {
        self.with_ampere(-self.ampere()).
        with_candela(-self.candela()).
        with_kelvin(-self.kelvin()).
        with_kilogram(-self.kilogram()).
        with_meter(-self.meter()).
        with_mole(-self.mole()).
        with_second(-self.second())
    }
    
}
impl Display for UnitBase
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result 
    {
        let numerator = (0..7).map(|i| 
            {
                let value =  self.get_element(i as u8);
                if value.numerator().is_positive()
                {         
                    if !value.is_fraction()
                    {
                        format!("{}", self.unit_name(i as u8)).to_owned()
                    }
                    else
                    {
                        format!("{}^{}", self.unit_name(i as u8), value).to_owned() 
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
                    if value.numerator().is_negative()
                    {
                        let value = -value;
                        if !value.is_fraction()
                        {
                            format!("{}", self.unit_name(i as u8)).to_owned()
                        }
                        else
                        {
                            format!("{}^{}", self.unit_name(i as u8), value).to_owned() 
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
                    if denominator.contains("*")
                    {
                        format!("1/({denominator})")
                    }
                    else
                    {
                        format!("1/{denominator}")
                    }
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
        UnitBase::default().
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
    pub(crate) fn new_length() -> Self
    {
        UnitBase::default().with_meter(Rational8::unity())
    }
    #[allow(unused)]
    pub(crate) fn new_mass() -> Self
    {
        UnitBase::default().with_kilogram(Rational8::unity())
    }
    #[allow(unused)]
    pub(crate) fn new_time() -> Self
    {
        UnitBase::default().with_second(Rational8::unity())
    }
    #[allow(unused)]
    pub(crate) fn new_current() -> Self
    {
        UnitBase::default().with_ampere(Rational8::unity())
    }
    #[allow(unused)]
    pub(crate) fn new_temperature() -> Self
    {
        UnitBase::default().with_kelvin(Rational8::unity())
    }
    #[allow(unused)]
    pub(crate) fn new_luminance() -> Self
    {
        UnitBase::default().with_candela(Rational8::unity())
    }

    #[allow(unused)]
    pub(crate) fn dimensionless() -> Self
    {
        UnitBase::default()
    }
    
    pub(crate) fn powi(&self, power: i8) -> Self
    {
        UnitBase::default().
        with_meter(self.meter()*power).
        with_second(self.second()*power).
        with_kilogram(self.kilogram()*power).
        with_ampere(self.ampere()*power).
        with_candela(self.candela()*power).
        with_kelvin(self.kelvin()*power).
        with_mole(self.mole()*power)
    }
    pub(crate) fn powf(&self, power: f64) -> Self
    {
        UnitBase::default().
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
        UnitDefinition { base: UnitBase::default().with_meter(meter.into()).with_kilogram(kilogram.into()).with_second(second.into()).with_ampere(ampere.into()).with_kelvin(kelvin.into()).with_mole(mole.into()).with_candela(candela.into()), multiplier }
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

    #[doc="Raise a unit to an floating point power."]
    pub fn powf(&self, power: f64) -> UnitDefinition
    {        
        UnitDefinition { base: self.base.powf(power), multiplier: self.multiplier.powf(power) }
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
    #[doc="Compute inverse of the current units"]
    pub fn inv(&self) -> Self
    {
        Self { base: self.base.inv(), multiplier: 1.0 /self.multiplier }
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


#[inline(always)]
pub(crate) fn to_unit_base(value: (f64, f64, f64, f64, f64, f64, f64)) -> UnitBase
{
    UnitBase::default().
    with_meter(value.0.into()).
    with_kilogram(value.1.into()).
    with_second(value.2.into()).
    with_ampere(value.3.into()).
    with_kelvin(value.4.into()).
    with_mole(value.5.into()).
    with_candela(value.6.into())
}


impl Display for UnitDefinition
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base)
    }
}
