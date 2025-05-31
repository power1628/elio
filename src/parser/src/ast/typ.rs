use super::Expr;
use derive_more::Display;

// statement attributes
#[derive(Debug, Display)]
#[display("{}: {}", name, value)]
pub struct OptionKV {
    pub name: String,
    pub value: Box<Expr>,
}

#[derive(Debug, Display)]
#[display("{} {} {}", name, typ, if *nullable { "NULL" } else { "NOT NULL" })]
pub struct ColumnDef {
    pub name: String,
    pub typ: DataType,  // data type
    pub nullable: bool, // whether the column can be null
}

#[derive(Debug, Display)]
#[display("PRIMARY KEY ({})", columns.join(", "))]
pub enum ConstraintSpec {
    PrimaryKey { columns: Vec<String> },
}

#[derive(Debug, Display)]
pub enum DataType {
    #[display("INTEGER")]
    Integer,
    #[display("FLOAT")]
    Float,
    #[display("STRING")]
    String,
    #[display("BOOLEAN")]
    Boolean,
}
