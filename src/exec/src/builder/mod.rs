use std::sync::Arc;

use mojito_common::schema::Schema;
use mojito_common::variable::VariableName;
use mojito_cypher::expr::Expr;
use mojito_cypher::plan_node::{CreateNode, PlanExpr};
use mojito_cypher::planner::RootPlan;
use mojito_expr::impl_::BoxedExpression;

use crate::error::ExecError;
use crate::executor::BoxedExecutor;
use crate::executor::create_node::CreateNodeExectuor;
use crate::task::TaskExecContext;

pub mod expression;


#[derive(thiserror::Error)]
#[derive(Debug)]
pub enum BuildError{
    #[error("variable {0} not found in schema")]
    VariableNotFound(VariableName),
    #[error("token {0} not resolved")]
    UnresolvedToken(String),
}

pub struct ExecutorBuilder {
    ctx: Arc<TaskExecContext>,
}

impl ExecutorBuilder {
    pub fn build_root(&mut self, _root @ RootPlan { plan, names }: &RootPlan) -> Result<BoxedExecutor, BuildError> {
        let _inputs = plan.inputs();
        match plan.as_ref() {
            PlanExpr::CreateNode(_create_node) => todo!(),
            PlanExpr::CreateRel(_create_rel) => todo!(),
            PlanExpr::Unit(_unit) => todo!(),
            _ => todo!(),
        }
    }

    fn build_node(&mut self, node: &PlanExpr) -> Result<BoxedExecutor, BuildError>{
        let inputs = node.inputs().iter().map(
            |x| self.build_node(*x)
        ).collect::<Result<Vec<_>,_>>()?;

        match node {
            PlanExpr::AllNodeScan(all_node_scan) => todo!(),
            PlanExpr::GetProperty(get_property) => todo!(),
            PlanExpr::Expand(expand) => todo!(),
            PlanExpr::Apply(apply) => todo!(),
            PlanExpr::Argument(argument) => todo!(),
            PlanExpr::Unit(unit) => todo!(),
            PlanExpr::CreateNode(create_node) => self.build_create_node(create_node, inputs),
            PlanExpr::CreateRel(create_rel) => todo!(),
            PlanExpr::Project(project) => todo!(),
            PlanExpr::Sort(sort) => todo!(),
            PlanExpr::Filter(filter) => todo!(),
            PlanExpr::Pagination(pagination) => todo!(),
            PlanExpr::Empty(empty) => todo!(),
        }
    }

    fn build_create_node(&self, node: &CreateNode, inputs: Vec<BoxedExecutor>) -> Result<BoxedExecutor, BuildError> {
        assert_eq!(inputs.len(), 1);
        CreateNodeExectuor {
            input: inputs[0],
            labels: node.inner.labels,
            properties: todo!(),
        }

        todo!()
    }
}







