use mojito_common::IrToken;
use mojito_parser::ast::SemanticDirection;

use super::*;

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
    pub(crate) inner: ExpandInner,
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
    pub input: Box<PlanExpr>,
    pub from: VariableName,
    pub to: VariableName,
    pub rel: VariableName,
    pub direction: SemanticDirection,
    pub types: Vec<IrToken>,
    pub kind: ExpandKind,
}

impl ExpandInner {
    fn build_schema(&self) -> Arc<Schema> {
        let mut schema = Schema::from_arc(self.input.schema());
        match self.kind {
            ExpandKind::All => {
                // add [r, to] to output
                schema.fields.push(Variable::new(&self.rel, &DataType::Rel));
                schema.fields.push(Variable::new(&self.to, &DataType::Node));
            }
            // add [r] to output
            ExpandKind::Into => schema.fields.push(Variable::new(&self.rel, &DataType::Rel)),
        }
        schema.into()
    }
}

impl InnerNode for ExpandInner {
    fn build_base(&self) -> PlanBase {
        let schema = self.build_schema();
        PlanBase::new(schema, self.input.ctx())
    }

    fn inputs(&self) -> Vec<&PlanExpr> {
        vec![&self.input]
    }
}
