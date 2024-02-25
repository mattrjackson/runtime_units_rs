use std::fmt::Display;

#[derive(Copy, Clone, Debug)]
pub enum RuntimeUnitError
{
    IncompatibleUnitConversion
}

impl Display for RuntimeUnitError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "{:?}", self)
    }
}