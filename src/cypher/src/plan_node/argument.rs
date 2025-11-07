use educe::Educe;

use super::*;

#[derive(Debug, Clone)]
pub struct Argument {
    pub base: PlanBase,
    inner: ArgumentInner,
}

impl Argument {
    pub fn new(inner: ArgumentInner) -> Self {
        Self {
            base: inner.build_base(),
            inner,
        }
    }
}

#[derive(Educe)]
#[educe(Debug, Clone)]
pub struct ArgumentInner {
    pub variables: Vec<Variable>,
    #[educe(Debug(ignore))]
    pub ctx: Arc<PlanContext>,
}

impl ArgumentInner {
    fn build_schema(&self) -> Arc<Schema> {
        Arc::new(self.variables.iter().cloned().collect::<Schema>())
    }
}

impl InnerNode for ArgumentInner {
    fn build_base(&self) -> PlanBase {
        PlanBase::new(self.build_schema(), self.ctx.clone())
    }
}
