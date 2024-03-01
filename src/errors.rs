use core::fmt::Display;

#[derive(Clone, Debug)]
pub enum RuntimeUnitError
{
    IncompatibleUnitConversion(String)
}

impl Display for RuntimeUnitError
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result 
    {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature="std")]
impl std::error::Error for RuntimeUnitError{}