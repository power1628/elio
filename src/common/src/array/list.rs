use std::sync::Arc;

use crate::array::buffer::{Buffer, BufferMut};
use crate::array::mask::{Mask, MaskMut};
use crate::array::{Array, ArrayBuilder, ArrayBuilderImpl, ArrayImpl};
use crate::data_type::DataType;
use crate::scalar::list::{ListValue, ListValueRef};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ListArray {
    data: Arc<ArrayImpl>,
    offsets: Buffer<u32>,
    valid: Mask,
}

impl Array for ListArray {
    type Builder = ListArrayBuilder;
    type OwnedItem = ListValue;
    type RefItem<'a> = ListValueRef<'a>;

    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>> {
        self.valid.get(idx).then(|| {
            let start = self.offsets[idx];
            let end = self.offsets[idx + 1];
            ListValueRef::new(self.data.as_ref(), start, end)
        })
    }

    unsafe fn get_unchecked(&self, idx: usize) -> Self::RefItem<'_> {
        let start = self.offsets[idx];
        let end = self.offsets[idx + 1];
        ListValueRef::new(self.data.as_ref(), start, end)
    }

    fn len(&self) -> usize {
        self.offsets.len() - 1
    }

    fn iter(&self) -> super::ArrayIterator<'_, Self> {
        super::ArrayIterator::new(self)
    }

    fn data_type(&self) -> DataType {
        DataType::List(Box::new(self.data.data_type()))
    }
}

#[derive(Debug)]
pub struct ListArrayBuilder {
    data: Box<ArrayBuilderImpl>,
    offsets: BufferMut<u32>,
    valid: MaskMut,
}

impl ArrayBuilder for ListArrayBuilder {
    type Array = ListArray;

    fn with_capacity(capacity: usize, typ: DataType) -> Self {
        let inner_type = {
            match typ {
                DataType::List(inner_typ) => inner_typ,
                _ => panic!("expected list type"),
            }
        };
        let mut offsets = BufferMut::with_capacity(capacity + 1);
        offsets.push(0);
        Self {
            data: Box::new(ArrayBuilderImpl::with_capacity(capacity, *inner_type)),
            offsets,
            valid: MaskMut::with_capacity(capacity),
        }
    }

    fn append_n(&mut self, value: Option<ListValueRef<'_>>, repeat: usize) {
        match value {
            Some(list) => {
                for _ in 0..repeat {
                    for item in list.iter() {
                        self.data.append(item);
                    }
                    self.offsets.push(self.data.len() as u32);
                }
                self.valid.append_n(true, repeat);
            }
            None => {
                self.offsets.push_n(self.data.len() as u32, repeat);
                self.valid.append_n(false, repeat);
            }
        }
    }

    fn finish(self) -> Self::Array {
        let data = self.data.finish();
        let offsets = self.offsets.freeze();
        let valid = self.valid.freeze();
        Self::Array {
            data: Arc::new(data),
            offsets,
            valid,
        }
    }

    fn len(&self) -> usize {
        self.offsets.len() - 1
    }
}
