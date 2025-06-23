use mojito_parser::parser::cypher_parser;

macro_rules! test_pattern {
    ($name:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let input = $input;
            assert_eq!(
                cypher_parser::pattern(input).map(|x| x
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(",")),
                Ok($expected.to_string())
            );
        }
    };
}

test_pattern!(single_node, "(n)", "(n)");
test_pattern!(single_node_with_var, "(n:Person)", "(n:Person)");

test_pattern!(single_rel, "()-[r]->()", "()-[r]->()");
test_pattern!(single_rel_1, "()-[r:REL]->()", "()-[r:REL]->()");
test_pattern!(single_rel_2, "()--()", "()-[]-()");
test_pattern!(single_rel_3, "()<--()", "()<-[]-()");
test_pattern!(
    single_node_with_var_and_rel,
    "(n:Person)-[r]->(m)",
    "(n:Person)-[r]->(m)"
);
