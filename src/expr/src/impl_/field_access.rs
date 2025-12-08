use std::sync::Arc;

use mojito_common::IrToken;
use mojito_common::array::chunk::DataChunk;
use mojito_common::array::{AnyArrayBuilder, ArrayImpl, ArrayRef, PhysicalType, StructArray};
use mojito_common::data_type::DataType;

use crate::error::EvalError;
use crate::impl_::{BoxedExpression, EvalCtx, Expression};

// Property Access or Field Access
// Expected input to be type of Struct
// If input is Node/Rel type, will access the property fields
// if input is struct type, will access sub fields
#[derive(Debug)]
pub struct FieldAccessExpr {
    pub input: BoxedExpression,
    key: IrToken,
    pub typ: DataType,
    physical_type: PhysicalType,
}

impl FieldAccessExpr {
    pub fn new(input: BoxedExpression, key: IrToken, typ: DataType) -> Self {
        let physical_type = typ.physical_type();
        Self {
            input,
            key,
            typ,
            physical_type,
        }
    }
}

impl Expression for FieldAccessExpr {
    fn typ(&self) -> &DataType {
        &self.typ
    }

    fn eval_batch(&self, chunk: &DataChunk, ctx: &dyn EvalCtx) -> Result<ArrayRef, EvalError> {
        let input = self.input.eval_batch(chunk, ctx)?;
        let key = self.key.name();

        // struct type
        if let ArrayImpl::Struct(input) = input.as_ref() {
            return struct_field_access(input, &key);
        }

        // node type
        if let ArrayImpl::Node(input) = input.as_ref() {
            // the output must be Any
            let mut builder = AnyArrayBuilder::with_capacity(input.len());
            input.props_iter().for_each(|props| {
                if let Some(props) = props {
                    builder.push(props.field_at(&key));
                }
            });
            return Ok(Arc::new(builder.finish().into()));
        }
        // rel type
        if let ArrayImpl::Rel(input) = input.as_ref() {
            // the output must be Any
            let mut builder = AnyArrayBuilder::with_capacity(input.len());
            input.props_iter().for_each(|props| {
                if let Some(props) = props {
                    builder.push(props.field_at(&key));
                }
            });
            return Ok(Arc::new(builder.finish().into()));
        }

        return Err(EvalError::type_error(
            "FieldAccess expected to have input of Node/Rel/Struct",
        ));
    }
}

fn struct_field_access(input: &StructArray, field: &str) -> Result<ArrayRef, EvalError> {
    let subfield = input
        .field_at(field)
        .ok_or(EvalError::FieldNotFound(field.to_string()))?;
    Ok(Arc::new(subfield))
}
