use crate::array::ArrayImpl;
use crate::array::list::ListArray;
use crate::scalar::{Scalar, ScalarRef};

#[derive(Clone, Debug)]
pub struct ListValue(ArrayImpl);

impl Scalar for ListValue {
    type ArrayType = ListArray;
    type RefType<'a> = ListValueRef<'a>;

    fn as_scalar_ref(&self) -> Self::RefType<'_> {
        todo!()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ListValueRef<'a>(&'a ArrayImpl);

impl<'a> ScalarRef<'a> for ListValueRef<'a> {
    type ArrayType = ListArray;
    type ScalarType = ListValue;

    fn to_owned_scalar(&self) -> Self::ScalarType {
        todo!()
    }
}
