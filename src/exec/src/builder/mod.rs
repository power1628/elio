use std::backtrace::Backtrace;
use std::sync::Arc;

use itertools::Itertools;
use mojito_common::variable::VariableName;
use mojito_cypher::plan_node::{self, CreateNode, PlanExpr, PlanNode, Project};
use mojito_cypher::planner::RootPlan;

use crate::builder::expression::{BuildExprContext, build_expression};
use crate::executor::all_node_scan::AllNodeScanExectuor;
use crate::executor::create_node::CreateNodeExectuor;
use crate::executor::produce_result::ProduceResultExecutor;
use crate::executor::project::ProjectExecutor;
use crate::executor::unit::UnitExecutor;
use crate::executor::{BoxedExecutor, Executor};
use crate::task::TaskExecContext;

pub mod expression;

#[derive(thiserror::Error, Debug)]
pub enum BuildError {
    #[error("variable {0} not found in schema")]
    VariableNotFound(VariableName, #[backtrace] Backtrace),
    #[error("token {0} not resolved")]
    UnresolvedToken(String, #[backtrace] Backtrace),
}

impl BuildError {
    pub fn variable_not_found(var_name: VariableName) -> Self {
        Self::VariableNotFound(var_name, Backtrace::capture())
    }

    pub fn unresolved_token(token: String) -> Self {
        Self::UnresolvedToken(token, Backtrace::capture())
    }
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
        PlanExpr::AllNodeScan(all_node_scan) => build_all_node_scan(ctx, all_node_scan, inputs),
        PlanExpr::GetProperty(_get_property) => todo!(),
        PlanExpr::Expand(_expand) => todo!(),
        PlanExpr::Apply(_apply) => todo!(),
        PlanExpr::Argument(_argument) => todo!(),
        PlanExpr::Unit(_unit) => Ok(UnitExecutor::default().boxed()),
        PlanExpr::ProduceResult(produce_result) => build_produce_result(ctx, produce_result, inputs),
        PlanExpr::CreateNode(create_node) => build_create_node(ctx, create_node, inputs),
        PlanExpr::CreateRel(_create_rel) => todo!(),
        PlanExpr::Project(project) => build_project(ctx, project, inputs),
        PlanExpr::Sort(_sort) => todo!(),
        PlanExpr::Filter(_filter) => todo!(),
        PlanExpr::Pagination(_pagination) => todo!(),
        PlanExpr::Empty(_empty) => todo!(),
    }
}

fn build_all_node_scan(
    _ctx: &mut ExecutorBuildContext,
    all_node_scan: &plan_node::AllNodeScan,
    inputs: Vec<BoxedExecutor>,
) -> Result<BoxedExecutor, BuildError> {
    assert_eq!(inputs.len(), 0);
    let schema = all_node_scan.schema();
    Ok(AllNodeScanExectuor::new(schema).boxed())
}

fn build_produce_result(
    _ctx: &mut ExecutorBuildContext,
    produce_result: &plan_node::ProduceResult,
    inputs: Vec<BoxedExecutor>,
) -> Result<BoxedExecutor, BuildError> {
    assert_eq!(inputs.len(), 1);
    let [input]: [BoxedExecutor; 1] = inputs.try_into().unwrap();

    let schema = input.schema().clone();
    let name2col = schema.name_to_col_map();
    let return_columns = produce_result
        .inner()
        .return_columns
        .iter()
        .map(|var| name2col[var])
        .collect_vec();

    Ok(ProduceResultExecutor {
        input,
        return_columns,
        schema: produce_result.schema().clone(),
    }
    .boxed())
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
        schema: node.schema().clone(),
    }
    .boxed())
}

fn build_project(
    ctx: &mut ExecutorBuildContext,
    node: &Project,
    inputs: Vec<BoxedExecutor>,
) -> Result<BoxedExecutor, BuildError> {
    assert_eq!(inputs.len(), 1);
    let [input]: [BoxedExecutor; 1] = inputs.try_into().unwrap();

    let schema = input.schema().clone();
    let ectx = BuildExprContext::new(&schema, ctx);

    let out_name_to_col = node.schema().name_to_col_map();

    // findout project item order
    let project_items = &node.inner().projections;
    let mut out_idx_to_idx = vec![0; project_items.len()];
    let mut exprs = vec![];
    for (i, (var, expr)) in project_items.iter().enumerate() {
        let out_idx = out_name_to_col[var];
        out_idx_to_idx[out_idx] = i;
        let expr = build_expression(&ectx, expr)?;
        exprs.push(Some(expr));
    }

    // build exprs
    let mut project_exprs = vec![];
    for in_idx in out_idx_to_idx {
        project_exprs.push(exprs[in_idx].take().unwrap());
    }

    Ok(ProjectExecutor {
        input,
        exprs: project_exprs,
        // we assume the schema is with the same order of projections
        schema: node.schema().clone(),
    }
    .boxed())
}
