-- (no id or description)
CREATE (n:Person {name: 'Alice', age: 30})

/*
RootPlan { names: [n] }
└─Project { exprs: [] }
  └─CreateNode { labels: [Person], properties: create_map{name: Alice, age: 30} }
    └─Unit
*/

