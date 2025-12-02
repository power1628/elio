-- (no id or description)
CREATE (n:Person {name: 'Alice', age: 30})

/*
RootPlan { names: [n] }
└─Project { exprs: [] }
  └─CreateNode { labels: [], properties: create_map{: Alice, : 30} }
    └─Unit
*/

