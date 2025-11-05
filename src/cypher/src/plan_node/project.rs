use super::*;

#[derive(Debug, Clone)]
pub struct Project {
    pub base: PlanBase,
    inner: ProjectInner,
}

#[derive(Debug, Clone)]
pub struct ProjectInner {
    input: Box<PlanExpr>,
    projections: Vec<(VariableName, Expr)>,
    // TODO(pgao): func deps
}

impl ProjectInner {
    fn build_schema(&self) -> Arc<Schema> {
        let mut schema = Schema::from_arc(self.input.schema());
        for (var, expr) in &self.projections {
            schema.fields.push(Variable::new(var, &expr.typ()));
        }
        schema.into()
    }
}

impl InnerNode for ProjectInner {
    fn build_base(&self) -> PlanBase {
        PlanBase::new(self.build_schema(), self.input.ctx())
    }
}
