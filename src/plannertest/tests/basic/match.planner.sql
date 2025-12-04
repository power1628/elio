-- match with label
MATCH (n:Person) RETURN n

/*
RootIR { names: [n] }
└─IrSingleQueryPart
  ├─QueryGraph { nodes: [n@0], filter: n@0 HasAll[Person] }
  └─Project { items: [n@1 AS n@0] }
RootPlan { names: [n] }
└─Project { exprs: [n@1 AS n@0] }
  └─Empty
*/

-- match and return wild card
MATCH (n:Person) RETURN *

/*
RootIR { names: [n] }
└─IrSingleQueryPart
  ├─QueryGraph { nodes: [n@0], filter: n@0 HasAll[Person] }
  └─Project { items: [n@0 AS n@0] }
RootPlan { names: [n] }
└─Project { exprs: [n@0 AS n@0] }
  └─Empty
*/

-- match with projection
MATCH (n:Person) RETURN n.name

/*
RootIR { names: [n.name] }
└─IrSingleQueryPart
  ├─QueryGraph { nodes: [n@0], filter: n@0 HasAll[Person] }
  └─Project { items: [nname@1 AS n@0.name] }
RootPlan { names: [n.name] }
└─Project { exprs: [nname@1 AS n@0.name] }
  └─Empty
*/

