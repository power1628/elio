use educe::{self, Educe};
use itertools::Itertools;

use super::*;
// TODO(pgao): associate the catalog object here?
// seems we should have an logical plan here?

#[derive(Debug, Clone)]
pub struct AllNodeScan {
    pub base: PlanBase,
    pub(crate) inner: AllNodeScanInner,
}

impl AllNodeScan {
    pub fn new(inner: AllNodeScanInner) -> Self {
        Self {
            base: inner.build_base(),
            inner,
        }
    }
}

impl PlanNode for AllNodeScan {
    type Inner = AllNodeScanInner;

    fn inner(&self) -> &Self::Inner {
        &self.inner
    }

    fn pretty(&self) -> XmlNode<'_> {
        let mut fields = vec![("variable", Pretty::from(self.inner.variable.as_ref()))];
        if !self.inner.arguments.is_empty() {
            fields.push((
                "arguments",
                Pretty::Array(
                    self.inner
                        .arguments
                        .iter()
                        .map(|x| Pretty::from(x.name.as_ref()))
                        .collect_vec(),
                ),
            ));
        }
        XmlNode::simple_record("AllNodeScan", fields, Default::default())
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

    fn inputs(&self) -> Vec<&PlanExpr> {
        vec![]
    }
}
