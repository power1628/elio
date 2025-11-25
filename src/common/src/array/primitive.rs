use crate::array::buffer::{Buffer, BufferMut};
use crate::array::mask::{Mask, MaskMut};
use crate::array::{Array, ArrayBuilder, ArrayImpl, ArrayIterator, PrimitiveArrayElementType};
use crate::data_type::DataType;
use crate::scalar::{Scalar, ScalarRef};
use crate::{NodeId, RelationshipId};

#[derive(Clone, Debug)]
pub struct PrimitiveArray<T: PrimitiveArrayElementType> {
    data: Buffer<T>,
    valid: Mask,
}

impl<T> Array for PrimitiveArray<T>
where
    T: PrimitiveArrayElementType,
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

    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>> {
        self.valid.get(idx).then(|| self.data[idx])
    }

    unsafe fn get_unchecked(&self, idx: usize) -> Self::RefItem<'_> {
        self.data[idx]
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn iter(&self) -> super::ArrayIterator<'_, Self> {
        ArrayIterator::new(self)
    }

    fn data_type(&self) -> DataType {
        T::data_type()
    }
}

impl<T: PrimitiveArrayElementType> PrimitiveArray<T> {
    pub fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }
}

pub struct PrimitiveArrayBuilder<T: PrimitiveArrayElementType> {
    data: BufferMut<T>,
    valid: MaskMut,
}

impl<T> ArrayBuilder for PrimitiveArrayBuilder<T>
where
    T: PrimitiveArrayElementType,
    T: Scalar<ArrayType = PrimitiveArray<T>>,
    for<'a> T: ScalarRef<'a, ScalarType = T, ArrayType = PrimitiveArray<T>>,
    for<'a> T: Scalar<RefType<'a> = T>,
    PrimitiveArray<T>: Into<ArrayImpl>,
    PrimitiveArray<T>: From<ArrayImpl>,
    PrimitiveArray<T>: std::fmt::Debug,
{
    type Array = PrimitiveArray<T>;

    fn with_capacity(capacity: usize, typ: DataType) -> Self {
        assert_eq!(typ, T::data_type());
        Self {
            data: BufferMut::with_capacity(capacity),
            valid: MaskMut::with_capacity(capacity),
        }
    }

    fn push(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>) {
        if let Some(value) = value {
            self.data.push(value);
            self.valid.push(true);
        } else {
            self.valid.push(false);
        }
    }

    fn finish(self) -> Self::Array {
        Self::Array {
            data: self.data.freeze(),
            valid: self.valid.freeze(),
        }
    }

    fn len(&self) -> usize {
        self.data.len()
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
