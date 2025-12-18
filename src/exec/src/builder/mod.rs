use std::backtrace::Backtrace;
use std::sync::Arc;
use std::usize;

use mojito_common::schema::Name2ColumnMap;
use mojito_common::variable::VariableName;
use mojito_cypher::plan_node::{self, CreateNode, PlanExpr, PlanNode, Project};
use mojito_cypher::planner::RootPlan;

use crate::builder::expression::{BuildExprContext, build_expression};
use crate::executor::all_node_scan::AllNodeScanExectuor;
use crate::executor::create_node::{CreateNodeExectuor, CreateNodeItem};
use crate::executor::create_rel::{CreateRelExectuor, CreateRelItem};
use crate::executor::expand::ExpandAllExecutor;
use crate::executor::produce_result::ProduceResultExecutor;
use crate::executor::project::ProjectExecutor;
use crate::executor::unit::UnitExecutor;
use crate::executor::var_expand::{
    TRAIL_PATH_MODE_FACTORY, VarExpandAllStrategy, VarExpandExecutor, VarExpandIntoFilter, WALK_PATH_MODE_FACTORY,
};
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
        PlanExpr::Expand(expand) => build_expand(ctx, expand, inputs),
        PlanExpr::VarExpand(var_expand) => build_var_expand(ctx, var_expand, inputs),
        PlanExpr::Apply(_apply) => todo!(),
        PlanExpr::Argument(_argument) => todo!(),
        PlanExpr::Unit(_unit) => Ok(UnitExecutor::default().boxed()),
        PlanExpr::ProduceResult(produce_result) => build_produce_result(ctx, produce_result, inputs),
        PlanExpr::CreateNode(create_node) => build_create_node(ctx, create_node, inputs),
        PlanExpr::CreateRel(create_rel) => build_create_rel(ctx, create_rel, inputs),
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

fn build_expand(
    _ctx: &mut ExecutorBuildContext,
    expand: &plan_node::Expand,
    inputs: Vec<BoxedExecutor>,
) -> Result<BoxedExecutor, BuildError> {
    assert_eq!(inputs.len(), 1);
    let [input]: [BoxedExecutor; 1] = inputs.try_into().unwrap();
    let schema = input.schema();
    let name2col = schema.name_to_col_map();

    let from = name2col
        .get(&expand.inner().from)
        .copied()
        .ok_or_else(|| BuildError::variable_not_found(expand.inner().from.clone()))?;

    // assume all rtypes are token ids
    let rtype = expand
        .inner()
        .types
        .iter()
        .map(|x| match x {
            mojito_common::IrToken::Resolved { token, .. } => Ok(*token),
            mojito_common::IrToken::Unresolved(name) => Err(BuildError::unresolved_token(name.to_string())),
        })
        .collect::<Result<Vec<_>, _>>()?;

    if expand.inner().kind == plan_node::ExpandKind::All {
        return Ok(ExpandAllExecutor {
            input,
            from,
            dir: expand.inner().direction,
            rtype,
            schema: expand.schema().clone(),
        }
        .boxed());
    }
    todo!("expand kind {:?}", expand.inner().kind);
}

fn build_var_expand(
    _ctx: &mut ExecutorBuildContext,
    expand: &plan_node::VarExpand,
    inputs: Vec<BoxedExecutor>,
) -> Result<BoxedExecutor, BuildError> {
    assert_eq!(inputs.len(), 1);
    let [input]: [BoxedExecutor; 1] = inputs.try_into().unwrap();
    let schema = input.schema();
    let name2col = schema.name_to_col_map();

    let from = name2col
        .get(&expand.inner().from)
        .copied()
        .ok_or_else(|| BuildError::variable_not_found(expand.inner().from.clone()))?;

    let to = match expand.inner().kind {
        plan_node::ExpandKind::All => None,
        plan_node::ExpandKind::Into => Some(
            name2col
                .get(&expand.inner().to)
                .copied()
                .ok_or_else(|| BuildError::variable_not_found(expand.inner().to.clone()))?,
        ),
    };

    // assume all rtypes are token ids
    let rtype = expand
        .inner()
        .rel_pattern
        .types
        .iter()
        .map(|x| match x {
            mojito_common::IrToken::Resolved { token, .. } => Ok(*token),
            mojito_common::IrToken::Unresolved(name) => Err(BuildError::unresolved_token(name.to_string())),
        })
        .collect::<Result<Vec<_>, _>>()?;

    let (len_min, len_max) = expand.inner().rel_pattern.length.as_range().unwrap();

    match (expand.inner().path_mode, to) {
        (plan_node::PathMode::Trail, None) => Ok(VarExpandExecutor {
            input,
            from,
            dir: expand.inner().rel_pattern.dir,
            rel_types: rtype.into_boxed_slice().into(),
            len_min,
            len_max: len_max.unwrap_or(usize::MAX),
            node_filter: None,
            rel_filter: None,
            schema: expand.schema().clone(),
            path_container_factory: &TRAIL_PATH_MODE_FACTORY,
            expand_kind_filter: VarExpandAllStrategy,
        }
        .boxed()),
        (plan_node::PathMode::Trail, Some(to_idx)) => Ok(VarExpandExecutor {
            input,
            from,
            dir: expand.inner().rel_pattern.dir,
            rel_types: rtype.into_boxed_slice().into(),
            len_min,
            len_max: len_max.unwrap_or(usize::MAX),
            node_filter: None,
            rel_filter: None,
            schema: expand.schema().clone(),
            path_container_factory: &TRAIL_PATH_MODE_FACTORY,
            expand_kind_filter: VarExpandIntoFilter { to_idx },
        }
        .boxed()),
        (plan_node::PathMode::Walk, None) => Ok(VarExpandExecutor {
            input,
            from,
            dir: expand.inner().rel_pattern.dir,
            rel_types: rtype.into_boxed_slice().into(),
            len_min,
            len_max: len_max.unwrap_or(usize::MAX),
            node_filter: None,
            rel_filter: None,
            schema: expand.schema().clone(),
            path_container_factory: &WALK_PATH_MODE_FACTORY,
            expand_kind_filter: VarExpandAllStrategy,
        }
        .boxed()),

        (plan_node::PathMode::Walk, Some(to_idx)) => Ok(VarExpandExecutor {
            input,
            from,
            dir: expand.inner().rel_pattern.dir,
            rel_types: rtype.into_boxed_slice().into(),
            len_min,
            len_max: len_max.unwrap_or(usize::MAX),
            node_filter: None,
            rel_filter: None,
            schema: expand.schema().clone(),
            path_container_factory: &WALK_PATH_MODE_FACTORY,
            expand_kind_filter: VarExpandIntoFilter { to_idx },
        }
        .boxed()),
    }
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
        .map(|var| {
            name2col
                .get(var)
                .copied()
                .ok_or_else(|| BuildError::variable_not_found(var.clone()))
        })
        .collect::<Result<Vec<_>, _>>()?;

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

    fn build_item(node: &plan_node::CreateNodeItem, ectx: &BuildExprContext) -> Result<CreateNodeItem, BuildError> {
        Ok(CreateNodeItem {
            labels: node.labels.clone(),
            properties: build_expression(ectx, &node.properties)?,
            variable: node.variable.clone(),
        })
    }

    let items = node
        .inner
        .nodes
        .iter()
        .map(|x| build_item(x, &ectx))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(CreateNodeExectuor {
        input,
        items,
        schema: node.schema().clone(),
    }
    .boxed())
}

fn build_create_rel(
    ctx: &mut ExecutorBuildContext,
    node: &plan_node::CreateRel,
    inputs: Vec<BoxedExecutor>,
) -> Result<BoxedExecutor, BuildError> {
    assert_eq!(inputs.len(), 1);
    let [input]: [BoxedExecutor; 1] = inputs.try_into().unwrap();

    let schema = input.schema().clone();
    let ectx = BuildExprContext::new(&schema, ctx);
    let name2col = schema.name_to_col_map();

    fn build_item(
        name2col: &Name2ColumnMap,
        rel: &plan_node::CreateRelItem,
        ectx: &BuildExprContext,
    ) -> Result<CreateRelItem, BuildError> {
        let start = name2col[&rel.start_node.name];
        let end = name2col[&rel.end_node.name];

        Ok(CreateRelItem {
            properties: build_expression(ectx, &rel.properties)?,
            rtype: rel.reltype.name().clone(),
            start,
            end,
        })
    }

    let items = node
        .inner()
        .rels
        .iter()
        .map(|x| build_item(&name2col, x, &ectx))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(CreateRelExectuor {
        input,
        items,
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
