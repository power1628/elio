use std::sync::Arc;

use crate::{data_type::DataType, variable::VariableName};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Schema {
    pub fields: Vec<Variable>,
}

impl Schema {
    pub fn empty() -> Self {
        Self { fields: Vec::new() }
    }

    pub fn from_arc(arc: Arc<Schema>) -> Self {
        (*arc).clone()
    }
}

impl FromIterator<Variable> for Schema {
    fn from_iter<T: IntoIterator<Item = Variable>>(iter: T) -> Self {
        Self {
            fields: iter.into_iter().collect(),
        }
    }
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

    #[inline]
    pub fn is_node(&self) -> bool {
        self.typ.is_node()
    }

    #[inline]
    pub fn is_rel(&self) -> bool {
        self.typ.is_rel()
    }

    #[inline]
    pub fn is_entity(&self) -> bool {
        self.typ.is_entity()
    }
}
