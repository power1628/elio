use std::sync::Arc;

use mojito_common::IrToken;
use mojito_common::array::chunk::DataChunk;
use mojito_common::array::{AnyArray, AnyArrayBuilder, Array, ArrayImpl, ArrayRef, PhysicalType, StructArray};
use mojito_common::data_type::DataType;
use mojito_common::scalar::StructValueRef;

use crate::error::EvalError;
use crate::impl_::{BoxedExpression, EvalCtx, Expression};

// Property Access or Field Access
// Expected input to be type of Struct
// If input is Node/Rel type, will access the property fields via key.token.
// If input is VirtualNode type, will access property fields from the store on the fly.
// Current does not support VirtualRel type, since we assume the virtual rel type is retrived by
// Expand operator.
// if input is struct type, will access sub fields via key.name
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

// TODO(pgao): materialize node at the initialize of expression evaluation to reduce reduandent storage access
impl Expression for FieldAccessExpr {
    fn typ(&self) -> &DataType {
        &self.typ
    }

    fn eval_batch(&self, chunk: &DataChunk, ctx: &dyn EvalCtx) -> Result<ArrayRef, EvalError> {
        let input = self.input.eval_batch(chunk, ctx)?;
        let key = self.key.name();

        // struct type
        if let ArrayImpl::Struct(input) = input.as_ref() {
            return struct_field_access(input, key);
        }

        // node type
        if let ArrayImpl::Node(input) = input.as_ref() {
            // the output must be Any
            let output = access_properties(input.props_iter(), input.len(), key);
            return Ok(Arc::new(output.into()));
        }
        // rel type
        if let ArrayImpl::Rel(input) = input.as_ref() {
            // the output must be Any
            let mut builder = AnyArrayBuilder::with_capacity(input.len());
            input.props_iter().for_each(|props| {
                if let Some(props) = props {
                    builder.push(props.field_at(key));
                }
            });
            return Ok(Arc::new(builder.finish().into()));
        }

        // virtual node
        if let ArrayImpl::VirtualNode(input) = input.as_ref() {
            // materialize node
            let node = ctx.materialize_node(input)?;
            let output = access_properties(node.props_iter(), input.len(), key);
            return Ok(Arc::new(output.into()));
        }

        Err(EvalError::type_error(
            "FieldAccess expected to have input of VirtualNode/Node/Rel/Struct",
        ))
    }
}

fn struct_field_access(input: &StructArray, field: &str) -> Result<ArrayRef, EvalError> {
    input
        .field_at(field)
        .ok_or(EvalError::FieldNotFound(field.to_string()))
        .cloned()
}

fn access_properties<'a>(
    props_iter: impl Iterator<Item = Option<StructValueRef<'a>>>,
    len: usize,
    key: &str,
) -> AnyArray {
    let mut builder = AnyArrayBuilder::with_capacity(len);
    props_iter.for_each(|props| {
        builder.push(props.and_then(|p| p.field_at(key)));
    });
    builder.finish()
}
