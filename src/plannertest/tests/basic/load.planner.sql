-- Load a CSV file and create nodes
LOAD CSV FROM 'https://example.com/data.csv' AS row 
CREATE (:Person {name: row.name, age: row.age})

/*
RootIR { names: [row] }
└─IrSingleQueryPart
  ├─QueryGraph
  │ └─mutating_pattern
  │   └─CreatePattern { nodes: [(anon@1):Person create_map{name: row@0.name, age: row@0.age}], rels: [] }
  └─IrSingleQueryPart
    ├─QueryGraph
    └─Load { variable: row@0, source_url: https://example.com/data.csv, format: CsvLoadFormat { header: true, delimiter: , } }
RootPlan { names: [row] }
└─ProduceResult { return_columns: row@0 }
  └─Load { source_url: https://example.com/data.csv, variable: row@0, format: CsvLoadFormat { header: true, delimiter: , } }
*/

