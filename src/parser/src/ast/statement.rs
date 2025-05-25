use super::{Attribute, ConstraintSpec, PropertyDef};

pub enum Statement {
    // Analyze
    // Explain
    // CreateDatabase
    CreateDatabase(Box<CreateDatabase>),
    CreateVertexType(Box<CreateVertexType>),
    CreateEdgeType(Box<CreateEdgeType>),
    //Query(Box<Query>),
}

pub struct CreateDatabase {
    pub db_name: String,
    // create only if not exists
    pub not_exists: bool,
    pub options: Vec<Attribute>,
}

pub struct CreateVertexType {
    pub name: String,
    pub not_exists: bool,
    // properties for vertex type
    pub properties: Vec<PropertyDef>,
    pub constrait: Option<ConstraintSpec>,
    pub options: Vec<Attribute>,
}

pub struct CreateEdgeType {
    pub name: String,
    pub not_exists: bool,
    pub properties: Vec<PropertyDef>,
    pub options: Vec<Attribute>,
}
