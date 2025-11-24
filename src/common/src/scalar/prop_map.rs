use crate::PropertyKeyId;
use crate::array::prop_map::PropertyMapArray;
use crate::scalar::{Scalar, ScalarRef};
use crate::store_types::PropertyValue;

#[derive(Clone, Debug)]
pub struct PropertyMapValue(Vec<(PropertyKeyId, PropertyValue)>);

impl Scalar for PropertyMapValue {
    type ArrayType = PropertyMapArray;
    type RefType<'a> = &'a PropertyMapValue;

    fn as_scalar_ref(&self) -> Self::RefType<'_> {
        todo!()
    }
}

impl<'a> ScalarRef<'a> for &'a PropertyMapValue {
    type ArrayType = PropertyMapArray;
    type ScalarType = PropertyMapValue;

    fn to_owned_scalar(&self) -> Self::ScalarType {
        todo!()
    }
}
