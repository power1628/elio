use crate::array::mask::Mask;

#[derive(Clone, Debug)]
pub struct BoolArray {
    bits: Mask,
    valid: Mask,
}

pub struct BoolArrayBuilder {}
