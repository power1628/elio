use indexmap::IndexMap;
use mojito_common::variable::VariableName;
use pretty_xmlish::{Pretty, PrettyConfig, XmlNode};

use crate::ir::horizon::QueryHorizon;
use crate::ir::query_graph::QueryGraph;

pub struct IrQueryRoot {
    pub inner: IrQuery,
    // mapping from variable name to output names
    // TODO(pgao): should we record the datatype here?
    pub names: IndexMap<String, VariableName>,
}

impl IrQueryRoot {
    pub fn explain(&self) -> String {
        let fields = vec![(
            "names",
            Pretty::Array(self.names.iter().map(|(k, _)| Pretty::display(k)).collect()),
        )];
        let children = vec![Pretty::Record(self.inner.xmlnode())];
        let tree = Pretty::simple_record("RootIR", fields, children);
        let mut config = PrettyConfig {
            indent: 3,
            width: 2048,
            need_boundaries: false,
            reduced_spaces: true,
        };
        let mut output = String::with_capacity(2048);
        config.unicode(&mut output, &tree);
        output
    }
}

pub struct IrQuery {
    pub queries: Vec<IrSingleQuery>,
    pub union_all: bool,
}

impl IrQuery {
    pub fn is_union(&self) -> bool {
        self.queries.len() > 1
    }

    pub fn xmlnode(&self) -> XmlNode<'_> {
        if self.is_union() {
            let fields = vec![
                (
                    "inputs",
                    Pretty::Array(self.queries.iter().map(|q| Pretty::Record(q.xmlnode())).collect()),
                ),
                ("distinct", Pretty::debug(&!self.union_all)),
            ];
            XmlNode::simple_record("UnionQuery", fields, Default::default())
        } else {
            self.queries[0].xmlnode()
        }
    }
}

#[derive(Default)]
pub struct IrSingleQuery {
    pub parts: Vec<IrSingleQueryPart>,
    // pub query_graph: QueryGraph,
    // pub horizon: QueryHorizon,
    // pub tail: Option<Box<IrSingleQuery>>,
    // TODO(pgao): the interesting_order may be derived here.
    // pub interesting_order: OrderingChoice,
}

impl IrSingleQuery {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn xmlnode(&self) -> XmlNode<'_> {
        let mut iter = self.parts.iter();
        let mut root = iter.next().unwrap().xmlnode();

        for nxt in iter {
            let mut node = nxt.xmlnode();
            node.children.push(Pretty::Record(root));
            root = node;
        }
        root
    }
}

#[derive(Default)]
pub struct IrSingleQueryPart {
    pub query_graph: QueryGraph,
    // for update and create clause, there may be no projection at the end
    pub horizon: Option<QueryHorizon>,
}

impl IrSingleQueryPart {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn with_projection(&mut self, proj: QueryHorizon) {
        self.horizon = Some(proj);
    }

    pub fn update_projection<F>(&mut self, f: F)
    where
        F: FnOnce(&mut QueryHorizon),
    {
        if let Some(proj) = &mut self.horizon {
            f(proj);
        }
    }

    pub fn xmlnode(&self) -> XmlNode<'_> {
        let mut children = vec![];
        children.push(Pretty::Record(self.query_graph.xmlnode()));
        if let Some(proj) = &self.horizon {
            children.push(Pretty::Record(proj.xmlnode()));
        }
        XmlNode::simple_record("IrSingleQueryPart", vec![], children)
    }
}
