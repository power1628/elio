-- path with variable
MATCH p = ()-[]-() RETURN p

/*
RootIR { names: [p] }
└─IrSingleQueryPart
  ├─QueryGraph { nodes: [anon@0, anon@1], rels: [(anon@0)<-[anon@2]->(anon@1)] }
  └─Project { items: [p@4 AS p@3] }
RootPlan { names: [p] }
└─ProduceResult { return_columns: p@4 }
  └─Project { exprs: [p@4 AS p@3] }
    └─Expand { from: anon@0, to: anon@1, rel: anon@2, direction: -, types: [], kind: All }
      └─AllNodeScan { variable: anon@0 }
*/

