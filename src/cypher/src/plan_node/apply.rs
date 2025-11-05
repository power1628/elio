use super::*;

#[derive(Debug, Clone)]
pub struct Apply {
    pub base: PlanBase,
    inner: ApplyInner,
}

impl Apply {
    pub fn new(inner: ApplyInner) -> Self {
        Self {
            base: inner.build_base(),
            inner,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApplyInner {
    pub left: Box<PlanExpr>,
    pub right: Box<PlanExpr>,
    // TODO(pgao): should we put argument here?
}

impl ApplyInner {
    fn build_schema(&self) -> Arc<Schema> {
        let mut schema = Schema::from_arc(self.left.schema());
        let right = self.right.schema();
        for item in right.fields.iter() {
            if schema.fields.iter().any(|x| x.name == item.name) {
                continue;
            } else {
                schema.fields.push(item.clone());
            }
        }
        schema.into()
    }
}

impl InnerNode for ApplyInner {
    fn build_base(&self) -> PlanBase {
        PlanBase::new(self.build_schema(), self.left.ctx())
    }
}
