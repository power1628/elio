use crate::PropertyKeyId;
use crate::array::prop_map::PropertyMapArray;
use crate::scalar::{Scalar, ScalarRef};
use crate::store_types::PropertyValue;

#[derive(Clone, Debug, Default, derive_more::DerefMut, derive_more::Deref, derive_more::From, derive_more::Into)]
pub struct PropertyMapValue(Vec<(PropertyKeyId, PropertyValue)>);

impl Scalar for PropertyMapValue {
    type ArrayType = PropertyMapArray;
    type RefType<'a> = PropertyMapValueRef<'a>;

    fn as_scalar_ref(&self) -> Self::RefType<'_> {
        PropertyMapValueRef(&self.0)
    }
}

#[derive(Clone, Copy, Debug, derive_more::DerefMut, derive_more::Deref, derive_more::From, derive_more::Into)]
pub struct PropertyMapValueRef<'a>(&'a [(PropertyKeyId, PropertyValue)]);

impl<'a> ScalarRef<'a> for PropertyMapValueRef<'a> {
    type ArrayType = PropertyMapArray;
    type ScalarType = PropertyMapValue;

    fn to_owned_scalar(&self) -> Self::ScalarType {
        Self::ScalarType::from(self.0.to_vec())
    }
}
