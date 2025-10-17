use mojito_common::data_type::DataType;

use crate::variable::VariableName;

pub struct VariableRef {
    pub name: VariableName,
    typ: DataType,
}
