use mojito_propb::map::{PropertyMap, PropertyMapRef};

use crate::array::prop_map::PropertyMapArray;
use crate::scalar::{Scalar, ScalarRef};
// use crate::store_types::PropertyValue;

#[derive(
    Clone, Debug, Default, derive_more::DerefMut, derive_more::Deref, derive_more::From, derive_more::Into, PartialEq,
)]
// pub struct PropertyMapValue(pub Vec<(PropertyKeyId, PropertyValue)>);
pub struct PropertyMapValue(pub PropertyMap);

impl Scalar for PropertyMapValue {
    type ArrayType = PropertyMapArray;
    type RefType<'a> = PropertyMapValueRef<'a>;

    fn as_scalar_ref(&self) -> Self::RefType<'_> {
        PropertyMapValueRef(self.0.as_ref())
    }
}

#[derive(
    Clone, Copy, Debug, derive_more::DerefMut, derive_more::Deref, derive_more::From, derive_more::Into, PartialEq,
)]
// pub struct PropertyMapValueRef<'a>(pub &'a [(PropertyKeyId, PropertyValue)]);
pub struct PropertyMapValueRef<'a>(PropertyMapRef<'a>);

impl<'a> ScalarRef<'a> for PropertyMapValueRef<'a> {
    type ArrayType = PropertyMapArray;
    type ScalarType = PropertyMapValue;

    fn to_owned_scalar(&self) -> Self::ScalarType {
        Self::ScalarType::from(self.0.to_owned())
    }
}
