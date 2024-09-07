use core::marker::PhantomData;

use super::prelude::*;
use super::*;

use crate::numeric_array::ArrayLength;

#[derive(Default, Clone)]
pub struct FFTConv<F, Size> {
    _marker1: PhantomData<F>,
    _marker2: PhantomData<Size>,
}

impl<F, Size> FFTConv<F, Size>
where
    F: Real,
    Size: ArrayLength + Send + Sync,
{
    pub fn new() -> Self {
        Self {
            _marker1: PhantomData,
            _marker2: PhantomData,
        }
    }
}

impl<F, Size> AudioNode for FFTConv<F, Size>
where
    F: Real,
    Size: ArrayLength + Send + Sync,
{
    const ID: u64 = 4242; // TODO: create ID on merge
    type Inputs = Size;
    type Outputs = Size;

    #[inline]
    fn tick(&mut self, input: &Frame<f32, Self::Inputs>) -> Frame<f32, Self::Outputs> {
        todo!();
    }
}
