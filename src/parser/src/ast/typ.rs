use super::Expr;

// statement attributes
#[derive(Debug)]
pub struct OptionKV {
    pub name: String,
    pub value: Box<Expr>,
}

#[derive(Debug)]
pub struct PropertyDef {
    pub name: String,
    pub typ: String,    // data type
    pub nullable: bool, // whether the column can be null
    pub attributes: Vec<OptionKV>,
}

#[derive(Debug)]
pub struct ConstraintSpec {
    pub kind: ConstraintKind,
    pub columns: Vec<String>,
}

#[derive(Debug)]
pub enum ConstraintKind {
    Unique,
    PrimaryKey,
}
