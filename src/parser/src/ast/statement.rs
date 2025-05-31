use super::{ConstraintSpec, OptionKV, PropertyDef};

#[derive(Debug)]
pub enum Statement {
    // Analyze
    // Explain
    // CreateDatabase
    CreateDatabase(Box<CreateDatabase>),
    CreateVertexType(Box<CreateVertexType>),
    CreateEdgeType(Box<CreateEdgeType>),
    //Query(Box<Query>),
}

#[derive(Debug)]
pub struct CreateDatabase {
    pub db_name: String,
    // create only if not exists
    pub not_exists: bool,
    pub options: Vec<OptionKV>,
}

#[derive(Debug)]
pub struct CreateVertexType {
    pub name: String,
    pub not_exists: bool,
    // properties for vertex type
    pub properties: Vec<PropertyDef>,
    pub constrait: Option<ConstraintSpec>,
    pub options: Vec<OptionKV>,
}

#[derive(Debug)]
pub struct CreateEdgeType {
    pub name: String,
    pub not_exists: bool,
    pub properties: Vec<PropertyDef>,
    pub options: Vec<OptionKV>,
}
