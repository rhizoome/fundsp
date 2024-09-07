use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, AtomicPtr, Ordering};

use super::audionode::*;
use super::prelude::U0;
use super::*;
use numeric_array::ArrayLength;

#[derive(Default)]
struct StreamBuffer {}

impl Clone for StreamBuffer {
    fn clone(&self) -> Self {
        todo!();
    }
}

#[derive(Default)]
pub struct FileStream<Size> {
    _marker: PhantomData<Size>,
}

impl<Size> FileStream<Size>
where
    Size: ArrayLength + Sync + Send,
{
    pub fn new() -> Self {
        todo!();
    }
}

impl<Size> Clone for FileStream<Size>
where
    Size: ArrayLength + Sync + Send,
{
    fn clone(&self) -> Self {
        todo!();
    }
}

impl<Size> AudioNode for FileStream<Size>
where
    Size: ArrayLength + Sync + Send,
{
    const ID: u64 = 4242; // TODO: give correct ID on or after merge.
    type Inputs = U0;
    type Outputs = Size;

    #[inline]
    fn tick(&mut self, input: &Frame<f32, Self::Inputs>) -> Frame<f32, Self::Outputs> {
        todo!();
    }
}
