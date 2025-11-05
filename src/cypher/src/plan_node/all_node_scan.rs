use educe::{self, Educe};

use super::*;
// TODO(pgao): associate the catalog object here?
// seems we should have an logical plan here?

#[derive(Debug, Clone)]
pub struct AllNodeScan {
    pub base: PlanBase,
    inner: AllNodeScanInner,
}

impl AllNodeScan {
    pub fn new_with_inner(inner: AllNodeScanInner) -> Self {
        Self {
            base: inner.build_base(),
            inner,
        }
    }
}

#[derive(Educe, Clone)]
#[educe(Debug)]
pub struct AllNodeScanInner {
    pub variable: VariableName,
    pub arguments: Vec<Variable>,
    #[educe(Debug(ignore))]
    pub ctx: Arc<PlanContext>,
}

impl AllNodeScanInner {
    pub fn new(variable: VariableName, arguments: Vec<Variable>, ctx: Arc<PlanContext>) -> Self {
        Self {
            variable,
            arguments,
            ctx,
        }
    }

    fn build_schema(&self) -> Arc<Schema> {
        let mut schema = Schema::empty();
        schema.fields.push(Variable {
            name: self.variable.clone(),
            typ: DataType::Node,
        });
        schema.fields.extend(self.arguments.clone());
        schema.into()
    }
}

impl InnerNode for AllNodeScanInner {
    fn build_base(&self) -> PlanBase {
        let schema = self.build_schema();
        PlanBase::new(schema, self.ctx.clone())
    }
}
