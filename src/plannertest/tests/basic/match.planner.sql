-- match without label
MATCH (n) RETURN n

/*
RootIR { names: [n] }
└─IrSingleQueryPart
  ├─QueryGraph { nodes: [n@0] }
  └─Project { items: [n@1 AS n@0] }
RootPlan { names: [n] }
└─ProduceResult { return_columns: n@1 }
  └─Project { exprs: [n@1 AS n@0] }
    └─AllNodeScan { variable: n@0 }
*/

-- match with label
MATCH (n:Person) RETURN n

/*
RootIR { names: [n] }
└─IrSingleQueryPart
  ├─QueryGraph { nodes: [n@0], filter: n@0 HasAll[Person] }
  └─Project { items: [n@1 AS n@0] }
RootPlan { names: [n] }
└─ProduceResult { return_columns: n@1 }
  └─Project { exprs: [n@1 AS n@0] }
    └─Filter { condition: n@0 HasAll[Person] }
      └─AllNodeScan { variable: n@0 }
*/

-- match and return wild card
MATCH (n:Person) RETURN *

/*
RootIR { names: [n] }
└─IrSingleQueryPart
  ├─QueryGraph { nodes: [n@0], filter: n@0 HasAll[Person] }
  └─Project { items: [n@0 AS n@0] }
RootPlan { names: [n] }
└─ProduceResult { return_columns: n@0 }
  └─Project { exprs: [n@0 AS n@0] }
    └─Filter { condition: n@0 HasAll[Person] }
      └─AllNodeScan { variable: n@0 }
*/

-- match with projection
MATCH (n:Person) RETURN n.name

/*
RootIR { names: [n.name] }
└─IrSingleQueryPart
  ├─QueryGraph { nodes: [n@0], filter: n@0 HasAll[Person] }
  └─Project { items: [nname@1 AS n@0.name] }
RootPlan { names: [n.name] }
└─ProduceResult { return_columns: nname@1 }
  └─Project { exprs: [nname@1 AS n@0.name] }
    └─Filter { condition: n@0 HasAll[Person] }
      └─AllNodeScan { variable: n@0 }
*/

