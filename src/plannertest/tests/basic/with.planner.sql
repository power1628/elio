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

