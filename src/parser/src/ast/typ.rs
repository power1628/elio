use super::Expr;

// statement attributes
pub struct Attribute {
    pub name: String,
    pub value: Box<Expr>,
}

pub struct PropertyDef {
    pub name: String,
    pub typ: String,    // data type
    pub nullable: bool, // whether the column can be null
    pub attributes: Vec<Attribute>,
}

pub struct ConstraintSpec {
    pub kind: ConstraintKind,
    pub columns: Vec<String>,
}

pub enum ConstraintKind {
    Unique,
    PrimaryKey,
}
