-- Create a node with properties, without return clause
CREATE (n:Person {name: 'Alice', age: 30})

/*
RootPlan { names: [] }
└─Project { exprs: [] }
  └─CreateNode { variable: n@0, labels: [Person], properties: create_map{name: Alice, age: 30} }
    └─Unit
*/

-- Create a node with properties, with return clause
CREATE (n:Person {name: 'Alice', age: 30}) RETURN *

/*
RootPlan { names: [n] }
└─Project { exprs: [n@0 AS n@0] }
  └─CreateNode { variable: n@0, labels: [Person], properties: create_map{name: Alice, age: 30} }
    └─Unit
*/

-- Create a node with properties, with return clause, only return node
CREATE (n:Person {name: 'Alice', age: 30}) RETURN n

/*
RootPlan { names: [n] }
└─Project { exprs: [n@1 AS n@0] }
  └─CreateNode { variable: n@0, labels: [Person], properties: create_map{name: Alice, age: 30} }
    └─Unit
*/

-- create multiple nodes
CREATE (n:Person {name: 'Alice', age: 30}) CREATE (m:Person {name: 'Bob', age: 31})

/*
RootPlan { names: [] }
└─Project { exprs: [] }
  └─CreateNode { variable: m@1, labels: [Person], properties: create_map{name: Bob, age: 31} }
    └─CreateNode { variable: n@0, labels: [Person], properties: create_map{name: Alice, age: 30} }
      └─Unit
*/

