use std::sync::Arc;

use mojito_common::variable::VariableName;
use mojito_cypher::plan_node::{CreateNode, PlanExpr};
use mojito_cypher::planner::RootPlan;

use crate::builder::expression::{BuildExprContext, build_expression};
use crate::executor::create_node::CreateNodeExectuor;
use crate::executor::unit::UnitExecutor;
use crate::executor::{BoxedExecutor, Executor};
use crate::task::TaskExecContext;

pub mod expression;

#[derive(thiserror::Error, Debug)]
pub enum BuildError {
    #[error("variable {0} not found in schema")]
    VariableNotFound(VariableName),
    #[error("token {0} not resolved")]
    UnresolvedToken(String),
}

pub struct ExecutorBuildContext {
    ctx: Arc<TaskExecContext>,
}

impl ExecutorBuildContext {
    pub fn new(ctx: Arc<TaskExecContext>) -> Self {
        Self { ctx }
    }
}

pub fn build_executor(
    ctx: &mut ExecutorBuildContext,
    _root @ RootPlan { plan, .. }: &RootPlan,
) -> Result<BoxedExecutor, BuildError> {
    build_node(ctx, plan)
}

fn build_node(ctx: &mut ExecutorBuildContext, node: &PlanExpr) -> Result<BoxedExecutor, BuildError> {
    let inputs = node
        .inputs()
        .iter()
        .map(|x| build_node(ctx, x))
        .collect::<Result<Vec<_>, _>>()?;

    match node {
        PlanExpr::AllNodeScan(_all_node_scan) => todo!(),
        PlanExpr::GetProperty(_get_property) => todo!(),
        PlanExpr::Expand(_expand) => todo!(),
        PlanExpr::Apply(_apply) => todo!(),
        PlanExpr::Argument(_argument) => todo!(),
        PlanExpr::Unit(_unit) => Ok(UnitExecutor::default().boxed()),
        PlanExpr::CreateNode(create_node) => build_create_node(ctx, create_node, inputs),
        PlanExpr::CreateRel(_create_rel) => todo!(),
        PlanExpr::Project(_project) => todo!(),
        PlanExpr::Sort(_sort) => todo!(),
        PlanExpr::Filter(_filter) => todo!(),
        PlanExpr::Pagination(_pagination) => todo!(),
        PlanExpr::Empty(_empty) => todo!(),
    }
}

fn build_create_node(
    ctx: &mut ExecutorBuildContext,
    node: &CreateNode,
    inputs: Vec<BoxedExecutor>,
) -> Result<BoxedExecutor, BuildError> {
    assert_eq!(inputs.len(), 1);
    let [input]: [BoxedExecutor; 1] = inputs.try_into().unwrap();

    let schema = input.schema().clone();
    let ectx = BuildExprContext::new(&schema, ctx);

    let properties = build_expression(&ectx, &node.inner.properties)?;

    Ok(CreateNodeExectuor {
        input,
        labels: node.inner.labels.clone(),
        properties,
        schema: schema.clone(),
    }
    .boxed())
}
