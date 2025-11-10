use super::*;

#[derive(Debug, Clone)]
pub struct Pagination {
    pub base: PlanBase,
    inner: PaginationInner,
}

impl Pagination {
    pub fn new(inner: PaginationInner) -> Self {
        Self {
            base: inner.build_base(),
            inner,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PaginationInner {
    pub input: Box<PlanExpr>,
    pub offset: i64,
    // -1 means all records should be returned
    pub limit: i64,
}

impl InnerNode for PaginationInner {
    fn build_base(&self) -> PlanBase {
        PlanBase::new(self.input.schema(), self.input.ctx())
    }
}
