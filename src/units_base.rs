use std:: ops::{Div, DivAssign, Mul, MulAssign};
use bitfield_struct::bitfield;

#[bitfield(u64)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Hash)]
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
    pub(crate) fn inverse(&self) -> Self
    {
        UnitBase::new().
        with_meter(-self.meter()).
        with_second(-self.second()).
        with_kilogram(-self.kilogram()).
        with_ampere(-self.ampere()).
        with_candela(-self.candela()).
        with_kelvin(-self.kelvin()).
        with_mole(-self.mole())
    }
    pub(crate) fn new_length() -> Self
    {
        UnitBase::new().with_meter(1)
    }
    pub(crate) fn new_mass() -> Self
    {
        UnitBase::new().with_kilogram(1)
    }
    pub(crate) fn new_time() -> Self
    {
        UnitBase::new().with_second(1)
    }
    pub(crate) fn new_current() -> Self
    {
        UnitBase::new().with_ampere(1)
    }
    pub(crate) fn new_temperature() -> Self
    {
        UnitBase::new().with_kelvin(1)
    }
    pub(crate) fn new_luminance() -> Self
    {
        UnitBase::new().with_candela(1)
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
pub struct Unit
{
    pub base: UnitBase,
    pub multiplier: f64
}

impl Unit
{
       
    pub fn is_convertible(&self, unit: Unit) -> bool
    {
        self.base == unit.base
    }
    pub fn powi(&self, power: i8) -> Unit
    {
        Unit { base: self.base.powi(power), multiplier: self.multiplier.powi(power as i32) }
    }
    pub fn multiplier(&self) -> f64
    {
        self.multiplier
    }
}
impl Mul<Unit> for Unit
{
    type Output = Unit;

    fn mul(self, rhs: Unit) -> Self::Output {
        Unit{
            multiplier: self.multiplier*rhs.multiplier,
            base: self.base*rhs.base
        }
    }
}
impl Div<Unit> for Unit
{
    type Output=Unit;

    fn div(self, rhs: Unit) -> Self::Output {
        Unit{
            multiplier: self.multiplier/rhs.multiplier,
            base: self.base/rhs.base
        }
    }
}

impl DivAssign for Unit
{
    fn div_assign(&mut self, rhs: Unit) {
        self.multiplier /= rhs.multiplier;
        self.base /= rhs.base;
    }
}

impl MulAssign for Unit
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

impl From<(UOMDimensions, UOMDimensions, UOMDimensions, UOMDimensions, UOMDimensions, UOMDimensions, UOMDimensions)> for UnitBase
{
    fn from(value: (UOMDimensions, UOMDimensions, UOMDimensions, UOMDimensions, UOMDimensions, UOMDimensions, UOMDimensions)) -> Self {
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