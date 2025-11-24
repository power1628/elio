use std::sync::Arc;

use crate::array::buffer::{Buffer, BufferMut};
use crate::array::mask::{Mask, MaskMut};
use crate::array::{Array, ArrayBuilder, ArrayBuilderImpl, ArrayImpl};
use crate::data_type::DataType;
use crate::scalar::list::{ListValue, ListValueRef};

#[derive(Clone, Debug)]
pub struct ListArray {
    data: Arc<ArrayImpl>,
    offsets: Buffer<u32>,
    valid: Mask,
}

impl Array for ListArray {
    type Builder = ListArrayBuilder;
    type OwnedItem = ListValue;
    type RefItem<'a> = ListValueRef<'a>;

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

    fn data_type(&self) -> DataType {
        DataType::List(Box::new(self.data.data_type()))
    }
}

pub struct ListArrayBuilder {
    inner: Box<ArrayBuilderImpl>,
    offsets: BufferMut<u32>,
    valid: MaskMut,
}

impl ArrayBuilder for ListArrayBuilder {
    type Array = ListArray;

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
