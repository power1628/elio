use mojito_parser::parser::cypher_parser;

#[test]
fn test_string_literal() {
    let input = "'value1'";

    println!("Parsed: {:?}", cypher_parser::expr(input));
}
