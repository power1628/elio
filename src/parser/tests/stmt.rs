use insta::assert_snapshot;
use mojito_parser::parser::cypher_parser;

macro_rules! stmt {
    ($query:expr) => {
        cypher_parser::statement($query).unwrap().to_string()
    };
}

#[test]
fn test_query() {
    assert_snapshot!(stmt!("RETURN n"), @"RETURN n");
    assert_snapshot!(stmt!("RETURN n ORDER BY n.name"), @"RETURN n ORDER BY n.name Asc");
    assert_snapshot!(stmt!("MATCH (n:Person) RETURN n.name"), @"MATCH (n:Person) RETURN n.name");
    assert_snapshot!(stmt!("MATCH (n:Person) RETURN n.name AS name"), @"MATCH (n:Person) RETURN n.name AS name");
    assert_snapshot!(stmt!("MATCH (n:Person) RETURN n.name AS name ORDER BY name"), @"MATCH (n:Person) RETURN n.name AS name ORDER BY name Asc");
    assert_snapshot!(stmt!("MATCH (a)-[]-(b) WITH a.x + b.x AS c RETURN c"), @"MATCH (a)-[]-(b) WITH (a.x) + (b.x) AS c RETURN c");
    assert_snapshot!(stmt!("MATCH (a)-[]-(b) WITH a.x + b.x AS c RETURN c ORDER BY c"), @"MATCH (a)-[]-(b) WITH (a.x) + (b.x) AS c RETURN c ORDER BY c Asc");
    assert_snapshot!(stmt!("MATCH (a)-[]-(b) WITH a.x + b.x AS c WHERE c.y > 100 RETURN c ORDER BY c DESC SKIP 1 LIMIT 1"), @"MATCH (a)-[]-(b) WITH (a.x) + (b.x) AS c WHERE (c.y) > (100) RETURN c ORDER BY c Desc SKIP 1 LIMIT 1");
}
