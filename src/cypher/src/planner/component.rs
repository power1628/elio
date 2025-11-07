//! Strategy to plan an connected component.

use std::collections::VecDeque;

use indexmap::IndexSet;
use itertools::Itertools;
use mojito_common::variable::VariableName;

use crate::{
    ir::{node_connection::RelPattern, query_graph::QueryGraph},
    plan_node::{
        AllNodeScan, AllNodeScanInner, Argument, ArgumentInner, Expand, ExpandInner, ExpandKind, Filter, FilterInner,
        PlanNode,
    },
};

use super::*;

// This is an simple implementation of planning an query graph.
pub(crate) fn plan_simple(ctx: &mut PlannerContext, qg: &QueryGraph) -> Result<Box<PlanExpr>, PlanError> {
    let mut solver = TraversalSolver::new(ctx, qg);
    solver.solve()?;
    let mut root = solver.root;
    // solve filter
    root = Filter::new(FilterInner {
        input: root,
        condition: qg.filter.clone(),
    })
    .into();

    Ok(root)
}

/// Solve the query graph by Traversal strategy:
/// 1. select start node to traversal, generate an plan leaf
///   - Argument
///   - NodeScan
/// 2. select node connection by the given node
///   - Expand
/// 3. if the node have multiple node connections, we have two strategy
///    3.1 DFS: this is what we currently doing
///    3.2 BFS
///
/// This only solves graph traversal, filter and get properties not solved by this class.
///
/// Topology is our first class citizon, purly solve the graph traversal.
struct TraversalSolver<'a> {
    pub ctx: &'a mut PlannerContext,
    pub qg: &'a QueryGraph,
    solved: IndexSet<VariableName>,
    stack: VecDeque<&'a RelPattern>,
    root: Box<PlanExpr>,
}

impl<'a> TraversalSolver<'a> {
    // initialize with generated leaf plan
    fn new(ctx: &'a mut PlannerContext, qg: &'a QueryGraph) -> Self {
        assert!(!qg.is_empty());
        let imported = qg.imported().iter().cloned().collect_vec();
        let mut solved = IndexSet::new();
        let mut stack = VecDeque::new();

        let mut qg_nodes = qg.nodes.iter();

        let mut root = if !imported.is_empty() {
            // imported as argument as plan leaf
            let inner = ArgumentInner {
                variables: imported.clone(),
                ctx: ctx.ctx.clone(),
            };
            let root: PlanExpr = Argument::new(inner).into();
            // if arguments have node connections, push the connection on stack
            for arg in imported.iter().filter(|i| i.is_node()).map(|x| &x.name) {
                // push in reverse order, since it's stack
                for conn in qg.connections(arg).collect_vec().into_iter().rev() {
                    stack.push_back(conn);
                }
            }

            // at this point, all arguments are solved
            imported.iter().for_each(|i| {
                solved.insert(i.name.clone());
            });
            Some(root)
        } else {
            None
        };

        if stack.is_empty() {
            // if argument does not have connections, select node and plan an node scan with argument
            // SAFETY: the qg must have at least on node
            let first = qg_nodes.next().unwrap();
            let inner = AllNodeScanInner {
                variable: first.clone(),
                arguments: imported,
                ctx: ctx.ctx.clone(),
            };
            root = Some(AllNodeScan::new(inner).into());
            solved.insert(first.clone());
            // push connections on stack
            for conn in qg.connections(first).collect_vec().into_iter().rev() {
                stack.push_back(conn);
            }
        }

        Self {
            ctx,
            qg,
            solved,
            stack,
            // SAFETY: imported.is_empty() and !stack.is_empty() won't happen at the same time.
            root: root.unwrap().into(),
        }
    }

    // DFS traversal
    // the final plan will be placed at root
    fn solve(&mut self) -> Result<(), PlanError> {
        while let Some(rel) = self.stack.pop_back() {
            if !self.solved.contains(&rel.variable) {
                self.solve_node_connection_by_expand(rel)?;
            }
        }
        Ok(())
    }

    fn solve_node_connection_by_expand(
        &mut self,
        _rel @ RelPattern {
            variable,
            endpoints: (left, right),
            dir,
            types,
            // TODO(pgao): supoort variable length
            length,
        }: &'a RelPattern,
    ) -> Result<(), PlanError> {
        if !length.is_simple() {
            return Err(PlanError::not_supported(
                "variable length relationship is not supported yet".to_string(),
            ));
        }
        let (kind, from, to, direction, expanded_noded) = {
            match (self.solved.contains(left), self.solved.contains(right)) {
                (true, true) => (ExpandKind::Into, left, right, *dir, None),
                (true, false) => (ExpandKind::All, left, right, *dir, Some(right.clone())),
                (false, true) => (ExpandKind::All, right, left, dir.rev(), Some(left.clone())),
                (false, false) => unreachable!(),
            }
        };

        let empty = PlanExpr::empty(self.root.ctx()).boxed();
        let inner = ExpandInner {
            input: std::mem::replace(&mut self.root, empty),
            from: from.clone(),
            to: to.clone(),
            rel: variable.clone(),
            direction,
            types: types.clone(),
            kind,
        };
        self.root = Expand::new(inner).into();

        self.solved.insert(variable.clone());

        // push new expanded vertex's connections into stack
        if let Some(expanded) = expanded_noded {
            self.solved.insert(expanded.clone());
            self.qg
                .connections(&expanded)
                .collect_vec()
                .into_iter()
                .rev()
                .for_each(|rel| self.stack.push_back(rel));
        }

        Ok(())
    }
}
