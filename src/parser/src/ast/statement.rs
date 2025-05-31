use super::{ColumnDef, ConstraintSpec, OptionKV};
use derive_more::Display;

#[derive(Debug, Display)]
#[display("{}", _0)]
pub enum Statement {
    // Analyze
    // Explain
    // CreateDatabase
    CreateDatabase(Box<CreateDatabase>),
    CreateVertexType(Box<CreateVertexType>),
    CreateEdgeType(Box<CreateEdgeType>),
    //Query(Box<Query>),
}

#[derive(Debug, Display)]
#[display("CREATE DATABASE {}{} WITH ({})", if *not_exists {"IF NOT EXISTS "} else {""}, db_name, 
    options.iter().map(|opt| opt.to_string()).collect::<Vec<_>>().join(", "))]
pub struct CreateDatabase {
    pub db_name: String,
    // create only if not exists
    pub not_exists: bool,
    pub options: Vec<OptionKV>,
}

#[derive(Debug, Display)]
#[display("CREATE VERTEX TYPE {}{} ({}) WITH ({})", if *not_exists {"IF NOT EXISTS "} else {""}, name, 
    columns.iter().map(|col| col.to_string()).chain(constrait.iter().map(|c| c.to_string())).collect::<Vec<_>>().join(", "),
    options.iter().map(|opt| opt.to_string()).collect::<Vec<_>>().join(", "))]
pub struct CreateVertexType {
    pub name: String,
    pub not_exists: bool,
    pub columns: Vec<ColumnDef>,
    pub constrait: Vec<ConstraintSpec>,
    pub options: Vec<OptionKV>,
}

#[derive(Debug, Display)]
#[display("CREATE EDGE TYPE {}{} (FROM {}, TO {}, {}) WITH ({})", if *not_exists {"IF NOT EXISTS "} else {""}, name, from, to,
    columns.iter().map(|col| col.to_string()).chain(constrait.iter().map(|c| c.to_string())).collect::<Vec<_>>().join(", "),
    options.iter().map(|opt| opt.to_string()).collect::<Vec<_>>().join(", "))]
pub struct CreateEdgeType {
    pub name: String,
    pub not_exists: bool,
    pub from: String,
    pub to: String,
    pub columns: Vec<ColumnDef>,
    pub constrait: Vec<ConstraintSpec>,
    pub options: Vec<OptionKV>,
}
