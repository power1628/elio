use crate::array::buffer::{Buffer, BufferMut};
use crate::array::mask::{Mask, MaskMut};
use crate::array::{Array, ArrayBuilder, ArrayImpl, ArrayIterator, PrimitiveArrayElementType};
use crate::data_type::{DataType, F64};
use crate::scalar::{Scalar, ScalarRef};
use crate::{NodeId, RelationshipId};

#[derive(Clone, PartialEq, Eq, Hash)]
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
    Self: std::fmt::Debug,
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

#[derive(Debug)]
pub struct PrimitiveArrayBuilder<T: PrimitiveArrayElementType> {
    data: BufferMut<T>,
    valid: MaskMut,
}

impl<T> ArrayBuilder for PrimitiveArrayBuilder<T>
where
    T: PrimitiveArrayElementType + Default,
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

    fn append_n(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>, repeat: usize) {
        if let Some(value) = value {
            self.data.push_n(value, repeat);
            self.valid.append_n(true, repeat);
        } else {
            // Push a dummy value for nulls, as it won't be read by `get` due to `valid` mask.
            // This maintains `data` and `valid` lengths consistency.
            self.data.push_n(T::default(), repeat);
            self.valid.append_n(false, repeat);
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

pub type FloatArray = PrimitiveArray<F64>;
pub type FloatArrayBuilder = PrimitiveArrayBuilder<F64>;

pub type U16Array = PrimitiveArray<u16>;
pub type U16ArrayBuilder = PrimitiveArrayBuilder<u16>;

pub type NodeIdArray = PrimitiveArray<NodeId>;
pub type NodeIdArrayBuilder = PrimitiveArrayBuilder<NodeId>;
pub type RelIdArray = PrimitiveArray<RelationshipId>;
pub type RelIdArrayBuilder = PrimitiveArrayBuilder<RelationshipId>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::array::ArrayBuilder;
    use crate::data_type::DataType;

    #[test]
    fn test_primitive_array_builder_push_and_finish() {
        let mut builder = PrimitiveArrayBuilder::<i64>::with_capacity(5, DataType::Integer);
        builder.append(Some(10));
        builder.append(Some(20));
        builder.append(None);
        builder.append(Some(30));
        builder.append(None);

        let arr = builder.finish();

        assert_eq!(arr.len(), 5);
        assert_eq!(arr.get(0), Some(10));
        assert_eq!(arr.get(1), Some(20));
        assert_eq!(arr.get(2), None);
        assert_eq!(arr.get(3), Some(30));
        assert_eq!(arr.get(4), None);

        unsafe {
            assert_eq!(arr.get_unchecked(0), 10);
            assert_eq!(arr.get_unchecked(1), 20);
            assert_eq!(arr.get_unchecked(2), i64::default()); // Default value for null
            assert_eq!(arr.get_unchecked(3), 30);
            assert_eq!(arr.get_unchecked(4), i64::default()); // Default value for null
        }
    }

    #[test]
    fn test_primitive_array_all_valid() {
        let mut builder = PrimitiveArrayBuilder::<F64>::with_capacity(3, DataType::Float);
        builder.append(Some(1.1.into()));
        builder.append(Some(2.2.into()));
        builder.append(Some(3.3.into()));
        let arr = builder.finish();

        assert_eq!(arr.len(), 3);
        assert_eq!(arr.get(0), Some(1.1.into()));
        assert_eq!(arr.get(1), Some(2.2.into()));
        assert_eq!(arr.get(2), Some(3.3.into()));
    }

    #[test]
    fn test_primitive_array_all_invalid() {
        let mut builder = PrimitiveArrayBuilder::<u16>::with_capacity(3, DataType::U16);
        builder.append(None);
        builder.append(None);
        builder.append(None);
        let arr = builder.finish();

        assert_eq!(arr.len(), 3);
        assert_eq!(arr.get(0), None);
        assert_eq!(arr.get(1), None);
        assert_eq!(arr.get(2), None);
    }

    #[test]
    fn test_primitive_array_iter() {
        let mut builder = PrimitiveArrayBuilder::<i64>::with_capacity(5, DataType::Integer);
        builder.append(Some(1));
        builder.append(Some(2));
        builder.append(None);
        builder.append(Some(4));
        builder.append(None);
        let arr = builder.finish();

        let mut iter = arr.iter();
        assert_eq!(iter.next(), Some(Some(1)));
        assert_eq!(iter.next(), Some(Some(2)));
        assert_eq!(iter.next(), Some(None));
        assert_eq!(iter.next(), Some(Some(4)));
        assert_eq!(iter.next(), Some(None));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_primitive_array_is_empty() {
        let builder = PrimitiveArrayBuilder::<i64>::with_capacity(0, DataType::Integer);
        let arr = builder.finish();
        assert!(arr.is_empty());

        let mut builder = PrimitiveArrayBuilder::<i64>::with_capacity(1, DataType::Integer);
        builder.append(Some(100));
        let arr = builder.finish();
        assert!(!arr.is_empty());
    }

    #[test]
    fn test_primitive_array_as_slice() {
        let mut builder = PrimitiveArrayBuilder::<i64>::with_capacity(3, DataType::Integer);
        builder.append(Some(1));
        builder.append(Some(2));
        builder.append(Some(3));
        let arr = builder.finish();

        assert_eq!(arr.as_slice(), &[1, 2, 3]);
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed\n  left: Float\n right: Integer")]
    fn test_primitive_array_builder_with_capacity_wrong_type() {
        let _builder = PrimitiveArrayBuilder::<i64>::with_capacity(5, DataType::Float);
    }
}
