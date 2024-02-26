use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum RuntimeUnitError
{
    IncompatibleUnitConversion(String)
}

impl Display for RuntimeUnitError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "{:?}", self)
    }
}