-- Create a node with properties, without return clause
CREATE (n:Person {name: 'Alice', age: 30})

/*
RootIR { names: [] }
└─IrSingleQueryPart
  └─QueryGraph
    └─mutating_pattern
      └─CreatePattern { nodes: [(n@0):Person create_map{name: Alice, age: 30}], rels: [] }
RootPlan { names: [] }
└─CreateNode { variable: n@0, labels: [Person], properties: create_map{name: Alice, age: 30} }
  └─Unit
*/

-- Create a node with properties, with return clause
CREATE (n:Person {name: 'Alice', age: 30}) RETURN *

/*
RootIR { names: [n] }
└─IrSingleQueryPart
  ├─QueryGraph
  │ └─mutating_pattern
  │   └─CreatePattern { nodes: [(n@0):Person create_map{name: Alice, age: 30}], rels: [] }
  └─Project { items: [n@0 AS n@0] }
RootPlan { names: [n] }
└─Project { exprs: [n@0 AS n@0] }
  └─CreateNode { variable: n@0, labels: [Person], properties: create_map{name: Alice, age: 30} }
    └─Unit
*/

-- Create a node with properties, with return clause, only return node
CREATE (n:Person {name: 'Alice', age: 30}) RETURN n

/*
RootIR { names: [n] }
└─IrSingleQueryPart
  ├─QueryGraph
  │ └─mutating_pattern
  │   └─CreatePattern { nodes: [(n@0):Person create_map{name: Alice, age: 30}], rels: [] }
  └─Project { items: [n@1 AS n@0] }
RootPlan { names: [n] }
└─Project { exprs: [n@1 AS n@0] }
  └─CreateNode { variable: n@0, labels: [Person], properties: create_map{name: Alice, age: 30} }
    └─Unit
*/

-- create multiple nodes
CREATE (n:Person {name: 'Alice', age: 30}) CREATE (m:Person {name: 'Bob', age: 31})

/*
RootIR { names: [] }
└─IrSingleQueryPart
  └─QueryGraph
    └─mutating_pattern
      ├─CreatePattern { nodes: [(n@0):Person create_map{name: Alice, age: 30}], rels: [] }
      └─CreatePattern { nodes: [(m@1):Person create_map{name: Bob, age: 31}], rels: [] }
RootPlan { names: [] }
└─CreateNode { variable: m@1, labels: [Person], properties: create_map{name: Bob, age: 31} }
  └─CreateNode { variable: n@0, labels: [Person], properties: create_map{name: Alice, age: 30} }
    └─Unit
*/

