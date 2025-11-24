use crate::array::prop::PropertyArray;
use crate::scalar::{Scalar, ScalarRef};
use crate::store_types::PropertyValue;

// here we use PropertyValue instead of AnyValue because we restrict the
// heterogeneity of values only on property values, which does not include node/rel/path types
impl Scalar for PropertyValue {
    type ArrayType = PropertyArray;
    type RefType<'a> = &'a PropertyValue;

    fn as_scalar_ref(&self) -> Self::RefType<'_> {
        todo!()
    }
}

impl<'a> ScalarRef<'a> for &'a PropertyValue {
    type ArrayType = PropertyArray;
    type ScalarType = PropertyValue;

    fn to_owned_scalar(&self) -> Self::ScalarType {
        todo!()
    }
}
