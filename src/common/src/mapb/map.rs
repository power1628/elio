use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::mapb::entry::{EntryMut, EntryRef, EntryValueRef};
use crate::mapb::meta::EntryMeta;
use crate::scalar::{DatumRef, ScalarRefImpl};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PropertyMap {
    // #layout
    // | len(u16)  | entry * len | value heap |
    // |    2      | 0..len      |            |
    pub(crate) data: Bytes,
}

impl PropertyMap {
    pub fn from_bytes(bytes: Bytes) -> Self {
        Self { data: bytes }
    }

    // number of entries in map
    pub fn len(&self) -> usize {
        (&self.data[0..2]).get_u16_le() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn as_ref(&self) -> PropertyMapRef<'_> {
        PropertyMapRef::new(self.data.as_ref())
    }

    pub fn bytes(&self) -> usize {
        self.data.len()
    }

    pub fn write<T: BufMut>(&self, buf: &mut T) {
        buf.put_slice(&self.data);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PropertyMapRef<'a> {
    // pointer to start of property map
    data: &'a [u8],
    // number of entries in map
    len: usize,
}

impl<'a> PropertyMapRef<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            len: (&data[0..2]).get_u16_le() as usize,
        }
    }

    pub fn bytes(&self) -> usize {
        self.data.len()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn heap(&self) -> &[u8] {
        &self.data[2 + self.len * size_of::<EntryMeta>()..]
    }

    pub fn meta(&self) -> &[EntryMeta] {
        unsafe {
            let meta_bytes = &self.data[2..2 + self.len * size_of::<EntryMeta>()];
            let ptr = meta_bytes.as_ptr() as *const EntryMeta;
            std::slice::from_raw_parts(ptr, self.len)
        }
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = EntryRef<'_>> {
        self.meta().iter().map(|meta| EntryRef::new(self.heap(), meta))
    }

    pub fn to_owned(&self) -> PropertyMap {
        PropertyMap {
            data: self.data.to_vec().into(),
        }
    }

    // TODO(pgao): use binary search
    pub fn get(&self, key_id: u16) -> Option<EntryValueRef<'_>> {
        self.iter().find(|x| x.key() == key_id).map(|x| x.value())
    }
}

// In ProeprtyMapMut, entry meta's offset is actually the size of entry value.
// when freeze, the meta's offset will be the offset
pub struct PropertyMapMut {
    entries: Vec<EntryMut>,
}

impl PropertyMapMut {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
        }
    }

    /// Return failed when property map value does not support given data type
    pub fn insert(&mut self, key_id: u16, datum: DatumRef<'_>) -> Result<(), String> {
        if datum.is_none() {
            self.entries.push(EntryMut::null(key_id));
            return Ok(());
        }

        let datum = datum.unwrap();
        let entry = match datum {
            ScalarRefImpl::Bool(b) => EntryMut::bool(key_id, b),
            ScalarRefImpl::Integer(i) => EntryMut::integer(key_id, i),
            ScalarRefImpl::Float(ordered_float) => EntryMut::float(key_id, *ordered_float),
            ScalarRefImpl::String(s) => EntryMut::string(key_id, s),
            ScalarRefImpl::U16(_) => return Err("u16 cannot be property value".to_owned()),
            ScalarRefImpl::NodeId(_) => return Err("node id cannot be property value".to_owned()),
            ScalarRefImpl::RelId(_) => return Err("relationship id cannot be property value".to_owned()),
            ScalarRefImpl::Node(_) => return Err("node cannot be property value".to_owned()),
            ScalarRefImpl::Rel(_) => return Err("relationship cannot be property value".to_owned()),
            ScalarRefImpl::List(list) => {
                if let Some(slice) = list.as_integer_slice() {
                    EntryMut::list_integer(key_id, slice)
                } else if let Some(slice) = list.as_float_slice() {
                    EntryMut::list_float(key_id, slice)
                } else {
                    return Err("list value must be integer or float".to_owned());
                }
            }
            ScalarRefImpl::Property(value) => match value {
                crate::store_types::PropertyValue::Null => EntryMut::null(key_id),
                crate::store_types::PropertyValue::Boolean(b) => EntryMut::bool(key_id, *b),
                crate::store_types::PropertyValue::Integer(i) => EntryMut::integer(key_id, *i),
                crate::store_types::PropertyValue::Float(ordered_float) => EntryMut::float(key_id, **ordered_float),
                crate::store_types::PropertyValue::String(s) => EntryMut::string(key_id, s),
                crate::store_types::PropertyValue::ListBool(items) => EntryMut::list_bool(key_id, items),
                crate::store_types::PropertyValue::ListInteger(items) => EntryMut::list_integer(key_id, items),
                crate::store_types::PropertyValue::ListFloat(ordered_floats) => {
                    EntryMut::list_float(key_id, ordered_floats)
                }
                crate::store_types::PropertyValue::ListString(items) => EntryMut::list_string(key_id, items),
            },
            ScalarRefImpl::PropertyMap(_) => {
                return Err("property map cannot be property value".to_owned());
            }
        };
        self.entries.push(entry);
        Ok(())
    }

    pub fn insert_bool(&mut self, key_id: u16, value: bool) {
        self.entries.push(EntryMut::bool(key_id, value));
    }

    pub fn insert_string(&mut self, key_id: u16, value: &str) {
        self.entries.push(EntryMut::string(key_id, value));
    }

    pub fn insert_list_string(&mut self, key_id: u16, value: &[String]) {
        self.entries.push(EntryMut::list_string(key_id, value));
    }

    fn sort(&mut self) {
        self.entries.sort_by_key(|entry| entry.key_id());
    }

    // sort keys and serialize to buffer
    // serialized into
    // |num_entries(u16)| + |Entry(12B) * num_entries| + | value heap |
    pub fn freeze(mut self) -> PropertyMap {
        self.sort();
        let cap = size_of::<u16>() + self.entries.iter().map(|x| x.estimated_size()).sum::<usize>();
        let mut buf = BytesMut::with_capacity(cap);
        // u16
        buf.put_u16_le(self.entries.len() as u16);
        let mut offset = 0;
        // entry
        self.entries.iter_mut().for_each(|entry| {
            entry.key.set_offset(offset);
            offset += entry.val.as_ref().map_or(0, |x| x.len());
            buf.put_slice(entry.key.0.as_slice());
        });
        // value heap
        self.entries.iter().for_each(|entry| {
            if let Some(ref val) = entry.val {
                buf.put_slice(&val.buffer);
            }
        });
        PropertyMap::from_bytes(buf.freeze())
    }
}
