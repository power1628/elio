//! NodeIndexSeek - Index-based node lookup
//!
//! This plan node uses a unique index to directly lookup nodes
//! instead of scanning all nodes and filtering.

use std::sync::Arc;

use itertools::Itertools;
use mojito_common::data_type::DataType;
use mojito_common::schema::{Schema, Variable};
use mojito_common::variable::VariableName;
use mojito_common::{LabelId, PropertyKeyId};
use pretty_xmlish::{Pretty, XmlNode};

use super::*;
use crate::expr::Expr;
use crate::plan_context::PlanContext;
use crate::plan_node::plan_base::PlanBase;

/// NodeIndexSeek uses a unique index to lookup nodes by property values
#[derive(Clone)]
pub struct NodeIndexSeek {
    pub base: PlanBase,
    pub(crate) inner: NodeIndexSeekInner,
}

impl std::fmt::Debug for NodeIndexSeek {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeIndexSeek")
            .field("base", &self.base)
            .field("inner", &self.inner)
            .finish()
    }
}

impl NodeIndexSeek {
    pub fn new(inner: NodeIndexSeekInner) -> Self {
        Self {
            base: inner.build_base(),
            inner,
        }
    }
}

impl PlanNode for NodeIndexSeek {
    type Inner = NodeIndexSeekInner;

    fn inner(&self) -> &Self::Inner {
        &self.inner
    }

    fn xmlnode(&self) -> XmlNode<'_> {
        let mut fields = vec![
            ("variable", Pretty::from(self.inner.variable.as_ref())),
            ("label", Pretty::from(self.inner.label_name.as_str())),
            ("constraint", Pretty::from(self.inner.constraint_name.as_str())),
        ];

        let props = self
            .inner
            .property_names
            .iter()
            .zip(self.inner.property_values.iter())
            .map(|(name, val)| format!("{} = {}", name, val.pretty()))
            .collect_vec();
        fields.push((
            "properties",
            Pretty::Array(props.into_iter().map(Pretty::from).collect_vec()),
        ));

        XmlNode::simple_record("NodeIndexSeek", fields, Default::default())
    }
}

#[derive(Clone)]
pub struct NodeIndexSeekInner {
    /// Output variable name for the node
    pub variable: VariableName,
    pub label_name: String,
    pub label_id: LabelId,
    pub constraint_name: String,
    pub property_names: Vec<String>,
    pub property_key_ids: Vec<PropertyKeyId>,
    pub property_values: Vec<Expr>,
    pub ctx: Arc<PlanContext>,
}

impl std::fmt::Debug for NodeIndexSeekInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeIndexSeekInner")
            .field("variable", &self.variable)
            .field("label_name", &self.label_name)
            .field("label_id", &self.label_id)
            .field("constraint_name", &self.constraint_name)
            .field("property_names", &self.property_names)
            .field("property_key_ids", &self.property_key_ids)
            .field("property_values", &self.property_values)
            .finish_non_exhaustive()
    }
}

impl NodeIndexSeekInner {
    fn build_schema(&self) -> Arc<Schema> {
        let mut schema = Schema::empty();
        schema.fields.push(Variable {
            name: self.variable.clone(),
            typ: DataType::VirtualNode,
        });
        schema.into()
    }
}

impl InnerNode for NodeIndexSeekInner {
    fn build_base(&self) -> PlanBase {
        let schema = self.build_schema();
        PlanBase::new(schema, self.ctx.clone())
    }

    fn inputs(&self) -> Vec<&PlanExpr> {
        vec![]
    }
}
