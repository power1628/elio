use std::{marker::PhantomData, ops::Range};

use bytes::{Bytes, BytesMut};
use std::mem::size_of;

// Typed buffer for storing elements of type T.
// Require T must be primitive types
#[derive(Clone, Debug)]
pub struct Buffer<T> {
    data: Bytes,
    len: usize,
    _phantom: PhantomData<T>,
}

impl<T> Buffer<T> {
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

impl<T> std::ops::Index<usize> for Buffer<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

// index range
impl<T> std::ops::Index<Range<usize>> for Buffer<T> {
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

impl<T> BufferMut<T> {
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

    pub fn freeze(self) -> Buffer<T> {
        Buffer {
            data: self.data.into(),
            len: self.len,
            _phantom: PhantomData,
        }
    }
}
