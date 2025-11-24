use crate::array::buffer::{Buffer, BufferMut};
use crate::array::mask::{Mask, MaskMut};
use crate::array::{Array, ArrayBuilder, PrimitiveType};

#[derive(Clone, Debug)]
pub struct PrimitiveArray<T: PrimitiveType> {
    data: Buffer<T>,
    valid: Mask,
}

impl<T: PrimitiveType> Array for PrimitiveArray<T> {
    type Builder = PrimitiveArrayBuilder<T>;
    type OwnedItem = T;
    type RefItem<'a> = &'a T;

    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>> {
        todo!()
    }

    unsafe fn get_unchecked(&self, idx: usize) -> Self::RefItem<'_> {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }

    fn iter(&self) -> super::ArrayIterator<'_, Self> {
        todo!()
    }

    fn data_type(&self) -> crate::data_type::DataType {
        todo!()
    }
}

pub struct PrimitiveArrayBuilder<T> {
    data: BufferMut<T>,
    valid: MaskMut,
}

impl<T: PrimitiveType> ArrayBuilder for PrimitiveArrayBuilder<T> {
    type Array = PrimitiveArray<T>;

    fn with_capacity(capacity: usize) -> Self {
        todo!()
    }

    fn push(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>) {
        todo!()
    }

    fn finish(self) -> Self::Array {
        todo!()
    }
}

pub type IntegerArray = PrimitiveArray<i64>;
pub type IntegerArrayBuilder = PrimitiveArrayBuilder<i64>;
pub type FloatArray = PrimitiveArray<f64>;
pub type FloatArrayBuilder = PrimitiveArrayBuilder<f64>;
pub type TokenIdArray = PrimitiveArray<u16>;
pub type TokenIdArrayBuilder = PrimitiveArrayBuilder<u16>;
pub type NodeIdArray = PrimitiveArray<u64>;
pub type NodeIdArrayBuilder = PrimitiveArrayBuilder<u64>;
pub type RelIdArray = PrimitiveArray<u64>;
pub type RelIdArrayBuilder = PrimitiveArrayBuilder<u64>;
