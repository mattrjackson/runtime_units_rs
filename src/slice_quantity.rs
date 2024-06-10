use crate::traits::{Slice, Unit};

// Implement slice trait for Vec and [f64;N]

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