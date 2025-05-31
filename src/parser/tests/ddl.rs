use mojito_parser::parser::cypher_parser;

#[test]
fn test_create_database() {
    let input = "CREATE DATABASE db WITH (option1: 'value1', option2: 42)";
    assert_eq!(
        cypher_parser::statement(input).map(|x| x.to_string()),
        Ok("CREATE DATABASE db WITH (option1: value1, option2: 42)".to_string())
    );
}

#[test]
fn test_create_vertex_type() {
    let input = "CREATE VERTEX TYPE Person (name STRING NOT NULL, age INTEGER) WITH (option1: 'value1')";
    assert_eq!(
        cypher_parser::statement(input).map(|x| x.to_string()),
        Ok("CREATE VERTEX TYPE Person (name STRING NOT NULL, age INTEGER NULL) WITH (option1: value1)".to_string())
    );
}

#[test]
fn test_create_edge_type() {
    let input = "CREATE EDGE TYPE Buy (FROM Person, TO Item, price INTEGER) WITH (option1: 'value1')";
    assert_eq!(
        cypher_parser::statement(input).map(|x| x.to_string()),
        Ok("CREATE EDGE TYPE Buy (FROM Person, TO Item, price INTEGER NULL) WITH (option1: value1)".to_string())
    );
}
