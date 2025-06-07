use mojito_parser::parser::cypher_parser;

macro_rules! test_expr {
    ($name:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let input = $input;
            assert_eq!(
                cypher_parser::expr(input).map(|x| x.to_string()),
                Ok($expected.to_string())
            );
        }
    };
}

test_expr!(test_integer_literal, "123", "123");
test_expr!(test_float_literal, "123.456", "123.456");
test_expr!(test_string_literal, "\"hello\"", "'hello'");
test_expr!(test_boolean_literal_true, "true", "TRUE");
test_expr!(test_boolean_literal_false, "false", "FALSE");
test_expr!(test_null_literal, "null", "NULL");
test_expr!(test_unary_operator_negation, "-42", "-(42)");
test_expr!(test_unary_operator_not, "not true", "NOT(TRUE)");
test_expr!(test_binary_operator_addition, "1 + 2", "(1) + (2)");

test_expr!(test_property_access, "node.property", "node.property");
test_expr!(test_function_call, "func(1,2)", "func(1, 2)");
test_expr!(test_nested_expression, "(1 + 2) * 3", "((1) + (2)) * (3)");
