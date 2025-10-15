//! Ast associated meta data info

pub trait AstMeta {
    type DataType: std::fmt::Debug + Clone;
}

pub struct RawMeta;

impl AstMeta for RawMeta {
    type DataType = ();
}
