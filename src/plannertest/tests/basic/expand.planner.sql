-- expand all
MATCH (a)-[r:KNOWS]-(b) RETURN *

/*
RootIR { names: [a, b, r] }
└─IrSingleQueryPart
  ├─QueryGraph { nodes: [a@0, b@1], rels: [(a@0)<-[r@2]->(b@1)] }
  └─Project { items: [a@0 AS a@0, b@1 AS b@1, r@2 AS r@2] }
RootPlan { names: [a, b, r] }
└─ProduceResult { return_columns: a@0,b@1,r@2 }
  └─Project { exprs: [a@0 AS a@0, b@1 AS b@1, r@2 AS r@2] }
    └─Expand { from: a@0, to: b@1, rel: r@2, direction: -, types: [KNOWS], kind: All }
      └─AllNodeScan { variable: a@0 }
*/

