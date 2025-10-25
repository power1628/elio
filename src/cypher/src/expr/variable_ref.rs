use mojito_common::data_type::DataType;

use crate::variable::VariableName;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct VariableRef {
    pub name: VariableName,
    typ: DataType,
}

impl VariableRef {
    pub fn new(name: VariableName, typ: DataType) -> Self {
        Self { name, typ }
    }
}
