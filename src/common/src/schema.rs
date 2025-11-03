use crate::{data_type::DataType, variable::VariableName};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Schema {
    pub fields: Vec<Variable>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variable {
    pub name: VariableName,
    pub typ: DataType,
}

impl Variable {
    pub fn new(name: &VariableName, typ: &DataType) -> Self {
        Self {
            name: name.clone(),
            typ: typ.clone(),
        }
    }
}
