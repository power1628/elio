use std::marker::PhantomData;
use std::mem::size_of;
use std::ops::Range;

use bytes::{Bytes, BytesMut};

use crate::{NodeId, RelationshipId};

pub trait BufferElementType: Send + Sync + Clone + Copy + std::fmt::Debug + Default {}

impl BufferElementType for u8 {}
impl BufferElementType for u16 {}
impl BufferElementType for u32 {}
impl BufferElementType for usize {}
impl BufferElementType for i64 {}
impl BufferElementType for u64 {}
impl BufferElementType for f64 {}
impl BufferElementType for NodeId {}
impl BufferElementType for RelationshipId {}

// Typed buffer for storing elements of type T.
// Require T must be primitive types
#[derive(Clone, Debug)]
pub struct Buffer<T: BufferElementType> {
    data: Bytes,
    len: usize,
    _phantom: PhantomData<T>,
}

impl<T: BufferElementType> Buffer<T> {
    pub fn new(data: Bytes, len: usize) -> Self {
        Self {
            data,
            len,
            _phantom: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.data.len() / size_of::<T>()
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn as_slice(&self) -> &[T] {
        let len = self.len();
        let data = self.data.as_ref();
        unsafe { std::slice::from_raw_parts(data.as_ptr() as *const T, len) }
    }

    pub fn into_mut(self) -> BufferMut<T> {
        BufferMut {
            data: self.data.into(),
            len: self.len,
            _phantom: PhantomData,
        }
    }
}

impl<T: BufferElementType> std::ops::Index<usize> for Buffer<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

// index range
impl<T: BufferElementType> std::ops::Index<Range<usize>> for Buffer<T> {
    type Output = [T];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.as_slice()[index]
    }
}

pub struct BufferMut<T> {
    data: BytesMut,
    len: usize,
    _phantom: PhantomData<T>,
}

impl<T: BufferElementType> BufferMut<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: BytesMut::with_capacity(capacity * size_of::<T>()),
            len: 0,
            _phantom: PhantomData,
        }
    }

    pub fn zeroed(len: usize) -> Self {
        Self {
            data: BytesMut::zeroed(len * size_of::<T>()),
            len,
            _phantom: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.data.capacity() / size_of::<T>()
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn as_slice(&self) -> &[T] {
        let len = self.len();
        let data = self.data.as_ref();
        unsafe { std::slice::from_raw_parts(data.as_ptr() as *const T, len) }
    }

    pub fn as_slice_mut(&mut self) -> &mut [T] {
        let len = self.len();
        let data = self.data.as_mut();
        unsafe { std::slice::from_raw_parts_mut(data.as_mut_ptr() as *mut T, len) }
    }

    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional * size_of::<T>());
    }

    pub fn push(&mut self, value: T)
    where
        T: Copy,
    {
        // convert T to &[u8]
        let extend = unsafe { std::slice::from_raw_parts(&value as *const T as *const u8, size_of::<T>()) };
        self.data.extend_from_slice(extend);
        self.len += 1;
    }

    pub fn extend_from_slice(&mut self, slice: &[T])
    where
        T: Copy,
    {
        let extend = unsafe { std::slice::from_raw_parts(slice.as_ptr() as *const u8, std::mem::size_of_val(slice)) };
        self.data.extend_from_slice(extend);
        self.len += slice.len();
    }

    pub fn freeze(self) -> Buffer<T>
    where
        T: BufferElementType,
    {
        Buffer {
            data: self.data.into(),
            len: self.len,
            _phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_new() {
        let data = Bytes::from_static(&[1, 2, 3, 4, 5, 6, 7, 8]);
        let buffer = Buffer::<u16>::new(data, 4);
        assert_eq!(buffer.len(), 4);
        assert_eq!(buffer.capacity(), 4);
        assert!(!buffer.is_empty());
        assert_eq!(buffer[0], 0x0201);
        assert_eq!(buffer[1], 0x0403);
        assert_eq!(buffer[2], 0x0605);
        assert_eq!(buffer[3], 0x0807);
    }

    #[test]
    fn test_buffer_empty() {
        let buffer = Buffer::<u32>::new(Bytes::new(), 0);
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.capacity(), 0);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_buffer_as_slice() {
        let data = Bytes::from_static(&[10, 0, 20, 0, 30, 0]);
        let buffer = Buffer::<u16>::new(data, 3);
        assert_eq!(buffer.as_slice(), &[10, 20, 30]);
    }

    #[test]
    fn test_buffer_into_mut() {
        let data = Bytes::from_static(&[1, 2, 3, 4]);
        let buffer = Buffer::<u8>::new(data, 4);
        let mut buffer_mut = buffer.into_mut();
        assert_eq!(buffer_mut.len(), 4);
        buffer_mut.push(5);
        assert_eq!(buffer_mut.len(), 5);
        let new_buffer = buffer_mut.freeze();
        assert_eq!(new_buffer.len(), 5);
        assert_eq!(new_buffer.as_slice(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_buffer_mut_with_capacity() {
        let buffer_mut = BufferMut::<u32>::with_capacity(10);
        assert_eq!(buffer_mut.len(), 0);
        assert!(buffer_mut.capacity() >= 10);
        assert!(buffer_mut.is_empty());
    }

    #[test]
    fn test_buffer_mut_zeroed() {
        let buffer_mut = BufferMut::<u16>::zeroed(5);
        assert_eq!(buffer_mut.len(), 5);
        assert_eq!(buffer_mut.as_slice(), &[0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_buffer_mut_push_and_extend() {
        let mut buffer_mut = BufferMut::<u8>::with_capacity(5);
        buffer_mut.push(1);
        buffer_mut.push(2);
        assert_eq!(buffer_mut.len(), 2);
        assert_eq!(buffer_mut.as_slice(), &[1, 2]);

        buffer_mut.extend_from_slice(&[3, 4, 5]);
        assert_eq!(buffer_mut.len(), 5);
        assert_eq!(buffer_mut.as_slice(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_buffer_mut_freeze() {
        let mut buffer_mut = BufferMut::<i64>::with_capacity(3);
        buffer_mut.push(-1);
        buffer_mut.push(-2);
        let buffer = buffer_mut.freeze();
        assert_eq!(buffer.len(), 2);
        assert_eq!(buffer.as_slice(), &[-1, -2]);
    }

    #[test]
    fn test_buffer_index_range() {
        let data = Bytes::from_static(&[1, 2, 3, 4, 5, 6, 7, 8]);
        let buffer = Buffer::<u8>::new(data, 8);
        assert_eq!(&buffer[1..4], &[2, 3, 4]);
    }

    #[test]
    fn test_buffer_as_slice_mut() {
        let mut buffer_mut = BufferMut::<u8>::zeroed(3);
        buffer_mut.as_slice_mut()[0] = 10;
        buffer_mut.as_slice_mut()[2] = 30;
        assert_eq!(buffer_mut.as_slice(), &[10, 0, 30]);
    }

    #[test]
    fn test_buffer_reserve() {
        let mut buffer_mut = BufferMut::<u8>::with_capacity(2);
        buffer_mut.push(0u8);
        buffer_mut.push(1u8);
        assert!(buffer_mut.capacity() >= 2);
        buffer_mut.reserve(10);
        assert!(buffer_mut.capacity() >= 12, "capacity: {}", buffer_mut.capacity());
    }

    // Test with NodeId and RelationshipId
    use crate::{NodeId, RelationshipId};
    #[test]
    fn test_buffer_with_node_id() {
        let mut buffer_mut = BufferMut::<NodeId>::with_capacity(2);
        buffer_mut.push(NodeId(100));
        buffer_mut.push(NodeId(200));
        let buffer = buffer_mut.freeze();
        assert_eq!(buffer.len(), 2);
        assert_eq!(buffer[0], NodeId(100));
        assert_eq!(buffer[1], NodeId(200));
    }

    #[test]
    fn test_buffer_with_relationship_id() {
        let mut buffer_mut = BufferMut::<RelationshipId>::with_capacity(2);
        buffer_mut.push(RelationshipId(1000));
        buffer_mut.push(RelationshipId(2000));
        let buffer = buffer_mut.freeze();
        assert_eq!(buffer.len(), 2);
        assert_eq!(buffer[0], RelationshipId(1000));
        assert_eq!(buffer[1], RelationshipId(2000));
    }
}
