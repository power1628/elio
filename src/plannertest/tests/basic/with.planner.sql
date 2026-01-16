-- with clause with match clause
MATCH (a) WITH a MATCH (b)--(a) RETURN a,b

/*
RootIR { names: [a, b] }
└─IrSingleQueryPart
  ├─QueryGraph { imported: [a@1], nodes: [b@2, a@1], rels: [(b@2)<-[anon@3:]->(a@1)] }
  ├─Project { items: [a@4 AS a@1, b@5 AS b@2] }
  └─IrSingleQueryPart
    ├─QueryGraph { nodes: [a@0] }
    └─Project { items: [a@1 AS a@0] }
RootPlan { names: [a, b] }
└─ProduceResult { return_columns: a@4,b@5 }
  └─Project { exprs: [a@4 AS a@1, b@5 AS b@2] }
    └─Apply
      ├─Project { exprs: [a@1 AS a@0] }
      │ └─AllNodeScan { variable: a@0 }
      └─ExpandAll { from: a@1, to: b@2, rel: anon@3, direction: -, types: [] }
        └─Argument { variables: [a@1] }
*/

-- with clause with expression
MATCH (a) WITH a, a.age + 1 AS b RETURN a,b

/*
RootIR { names: [a, b] }
└─IrSingleQueryPart
  ├─QueryGraph { imported: [a@1, b@2] }
  ├─Project { items: [a@3 AS a@1, b@4 AS b@2] }
  └─IrSingleQueryPart
    ├─QueryGraph { nodes: [a@0] }
    └─Project { items: [a@1 AS a@0, b@2 AS add(a@0.age, 1)] }
RootPlan { names: [a, b] }
└─ProduceResult { return_columns: a@3,b@4 }
  └─Project { exprs: [a@3 AS a@1, b@4 AS b@2] }
    └─Apply
      ├─Project { exprs: [a@1 AS a@0, b@2 AS add(a@0.age, 1)] }
      │ └─AllNodeScan { variable: a@0 }
      └─Argument { variables: [a@1, b@2] }
*/

-- with clause with match clause
MATCH (a)-[]-(b) WITH a MATCH (b)-[]-(c) RETURN a,b,c

/*
RootIR { names: [a, b, c] }
└─IrSingleQueryPart
  ├─QueryGraph { imported: [a@3], nodes: [b@4, c@5], rels: [(b@4)<-[anon@6:]->(c@5)] }
  ├─Project { items: [a@7 AS a@3, b@8 AS b@4, c@9 AS c@5] }
  └─IrSingleQueryPart
    ├─QueryGraph { nodes: [a@0, b@1], rels: [(a@0)<-[anon@2:]->(b@1)] }
    └─Project { items: [a@3 AS a@0] }
RootPlan { names: [a, b, c] }
└─ProduceResult { return_columns: a@7,b@8,c@9 }
  └─Project { exprs: [a@7 AS a@3, b@8 AS b@4, c@9 AS c@5] }
    └─Apply
      ├─Project { exprs: [a@3 AS a@0] }
      │ └─ExpandAll { from: a@0, to: b@1, rel: anon@2, direction: -, types: [] }
      │   └─AllNodeScan { variable: a@0 }
      └─ExpandAll { from: b@4, to: c@5, rel: anon@6, direction: -, types: [] }
        └─AllNodeScan { variable: b@4, arguments: [a@3] }
*/

-- with clause with match clause
MATCH (a)-[]-(b) WITH a, b MATCH (b)-[]-(c) RETURN a,b,c

/*
RootIR { names: [a, b, c] }
└─IrSingleQueryPart
  ├─QueryGraph { imported: [a@3, b@4], nodes: [b@4, c@5], rels: [(b@4)<-[anon@6:]->(c@5)] }
  ├─Project { items: [a@7 AS a@3, b@8 AS b@4, c@9 AS c@5] }
  └─IrSingleQueryPart
    ├─QueryGraph { nodes: [a@0, b@1], rels: [(a@0)<-[anon@2:]->(b@1)] }
    └─Project { items: [a@3 AS a@0, b@4 AS b@1] }
RootPlan { names: [a, b, c] }
└─ProduceResult { return_columns: a@7,b@8,c@9 }
  └─Project { exprs: [a@7 AS a@3, b@8 AS b@4, c@9 AS c@5] }
    └─Apply
      ├─Project { exprs: [a@3 AS a@0, b@4 AS b@1] }
      │ └─ExpandAll { from: a@0, to: b@1, rel: anon@2, direction: -, types: [] }
      │   └─AllNodeScan { variable: a@0 }
      └─ExpandAll { from: b@4, to: c@5, rel: anon@6, direction: -, types: [] }
        └─Argument { variables: [a@3, b@4] }
*/

