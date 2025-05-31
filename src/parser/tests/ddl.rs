use mojito_parser::ast::*;
use mojito_parser::parser::cypher_parser;

#[test]
fn test_create_database() {
    let input = "
CREATE DATABASE mydb (option1: 'value1', option2: 42)";

    println!("Parsed: {:?}", cypher_parser::statement(input));
}
