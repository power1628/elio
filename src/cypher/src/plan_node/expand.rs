use mojito_parser::ast::SemanticDirection;

use super::*;
use crate::expr::IrToken;

#[derive(Debug, Clone, Copy)]
pub enum ExpandKind {
    // input (a), output (a)-[r]-(b)
    All,
    // input (a), (b), output (a)-[r]-(b)
    Into,
}

#[derive(Clone, Debug)]
pub struct Expand {
    pub base: PlanBase,
    inner: ExpandInner,
}

impl Expand {
    pub fn new(inner: ExpandInner) -> Self {
        Self {
            base: inner.build_base(),
            inner,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExpandInner {
    input: Box<PlanExpr>,
    from: VariableName,
    // some on ExpandAll
    to: Option<VariableName>,
    rel: VariableName,
    direction: SemanticDirection,
    types: Vec<IrToken>,
    kind: ExpandKind,
}

impl ExpandInner {
    fn build_schema(&self) -> Arc<Schema> {
        let mut schema = Schema::from_arc(self.input.schema());
        match self.kind {
            ExpandKind::All => {
                schema.fields.push(Variable::new(&self.rel, &DataType::Relationship));
                schema
                    .fields
                    .push(Variable::new(self.to.as_ref().unwrap(), &DataType::Node));
            }

            ExpandKind::Into => schema.fields.push(Variable::new(&self.rel, &DataType::Node)),
        }
        schema.into()
    }
}

impl InnerNode for ExpandInner {
    fn build_base(&self) -> PlanBase {
        let schema = self.build_schema();
        PlanBase::new(schema, self.input.ctx())
    }
}
