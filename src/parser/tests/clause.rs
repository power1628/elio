use insta::assert_snapshot;
use mojito_parser::parser::cypher_parser;

macro_rules! clause {
    ($input:expr) => {
        cypher_parser::clause($input).unwrap().to_string()
    };
}

#[test]
fn test_create() {
    assert_snapshot!(clause!("CREATE (n:Person {name: 'Alice'})"), @"CREATE (n:Person{name: 'Alice'})");
    assert_snapshot!(clause!(
        "CREATE (a:Person{name: 'Alice'}), (b:Person{name: 'Bob'})"), 
        @"CREATE (a:Person{name: 'Alice'}), (b:Person{name: 'Bob'})");
    assert_snapshot!(clause!(
        "CREATE (a:Person{name: 'Alice'}), (b:Person{name: 'Bob'})-[:KNOWS]->(c:Person{name: 'Charlie'})"), 
        @"CREATE (a:Person{name: 'Alice'}), (b:Person{name: 'Bob'})-[:KNOWS]->(c:Person{name: 'Charlie'})");
}
