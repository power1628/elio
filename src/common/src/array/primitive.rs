use crate::array::buffer::{Buffer, BufferMut};
use crate::array::mask::{Mask, MaskMut};
use crate::array::{Array, ArrayBuilder, ArrayImpl, PrimitiveType};
use crate::scalar::{Scalar, ScalarRef};
use crate::{NodeId, RelationshipId};

#[derive(Clone, Debug)]
pub struct PrimitiveArray<T: PrimitiveType> {
    data: Buffer<T>,
    valid: Mask,
}

impl<T> Array for PrimitiveArray<T>
where
    T: PrimitiveType,
    T: Scalar<ArrayType = Self>,
    for<'a> T: ScalarRef<'a, ScalarType = T, ArrayType = Self>,
    for<'a> T: Scalar<RefType<'a> = T>,
    Self: Into<ArrayImpl>,
    Self: From<ArrayImpl>,
{
    type Builder = PrimitiveArrayBuilder<T>;
    type OwnedItem = T;
    // primive types, just copy the value
    type RefItem<'a> = T;

    fn get(&self, _idx: usize) -> Option<Self::RefItem<'_>> {
        todo!()
    }

    unsafe fn get_unchecked(&self, _idx: usize) -> Self::RefItem<'_> {
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

pub struct PrimitiveArrayBuilder<T: PrimitiveType> {
    data: BufferMut<T>,
    valid: MaskMut,
}

impl<T> ArrayBuilder for PrimitiveArrayBuilder<T>
where
    T: PrimitiveType,
    T: Scalar<ArrayType = PrimitiveArray<T>>,
    for<'a> T: ScalarRef<'a, ScalarType = T, ArrayType = PrimitiveArray<T>>,
    for<'a> T: Scalar<RefType<'a> = T>,
    PrimitiveArray<T>: Into<ArrayImpl>,
    PrimitiveArray<T>: From<ArrayImpl>,
    PrimitiveArray<T>: std::fmt::Debug,
{
    type Array = PrimitiveArray<T>;

    fn with_capacity(_capacity: usize) -> Self {
        todo!()
    }

    fn push(&mut self, _value: Option<<Self::Array as Array>::RefItem<'_>>) {
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

pub type U16Array = PrimitiveArray<u16>;
pub type U16ArrayBuilder = PrimitiveArrayBuilder<u16>;

pub type NodeIdArray = PrimitiveArray<NodeId>;
pub type NodeIdArrayBuilder = PrimitiveArrayBuilder<NodeId>;
pub type RelIdArray = PrimitiveArray<RelationshipId>;
pub type RelIdArrayBuilder = PrimitiveArrayBuilder<RelationshipId>;
