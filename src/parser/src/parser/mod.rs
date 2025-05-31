peg::parser! {
  pub grammar cypher_parser() for str {
    use crate::ast::*;

    /// ---------------------
    /// Whitespace
    /// ---------------------
    rule _() = [' ' | '\t' | '\r' | '\n']*


    /// ---------------------
    /// Statement
    /// ---------------------
    pub rule statement() -> Statement
        = _ s:(create_database() / create_vertex_type() / create_edge_type()) _ {s}

    /// create database statement
    pub rule create_database() -> Statement
        = CREATE() _ DATABASE() _ db_name:ident() _ options:attribute_list()? {
            Statement::CreateDatabase(Box::new(CreateDatabase {
                db_name: db_name.to_string(),
                not_exists: false,
                options: options.unwrap_or_default(),
            }))
        }

    /// create vertex type statement
    pub rule create_vertex_type() -> Statement
        = CREATE() _ VERTEX() _ TYPE() _ name:ident() _ options:attribute_list()? {
            todo!()
        }

    /// create edge type statement
    pub rule create_edge_type() -> Statement
        = CREATE() EDGE() TYPE() name:ident() options:attribute_list()? {
            todo!()
        }


    /// ---------------------
    /// Expression
    /// ---------------------
    pub rule expr() -> Expr
        = precedence! {
            l:literal() { l }
        }

    rule literal() -> Expr
        = b:(TRUE() / FALSE()) { Expr::new_boolean(b == "true") }
        / f:float_literal() { Expr::new_float(f.to_string()) }
        / i:integer_literal() { Expr::new_integer(i.to_string()) }
        / s:string_literal() { Expr::new_string(s.to_string()) }
        / n:null_literal() { n }

    rule integer_literal() -> &'input str
        = $(number())
    rule float_literal() -> &'input str
        = $(number() "." number())
    rule string_literal() -> &'input str
        = "'" content:$((!['\''] [_])* ) "'" {
            content
        }
        / "\"" content:$((!['\"'] [_])* ) "\"" {
            content
        }
    rule null_literal() -> Expr
        = NULL() { Expr::new_null() }

    /// ---------------------
    /// Common
    /// ---------------------
    rule number() -> &'input str
        = first:$(['0'..='9']+) {
            first
        }

    rule ident() -> &'input str
        = first:$(['a'..='z' | 'A'..='Z'] ['a'..='z' | 'A'..='Z' | '0'..='9']*) {
            first
        }

    rule attribute_list() -> Vec<OptionKV>
        = "(" _ kvs:(option_kv() ** ",") _ ")" {
            kvs
        }
    rule option_kv() -> OptionKV
        = _ key:ident() _ ":" _ value:expr() _ {
            OptionKV { name: key.to_string(), value: Box::new(value) }
        }



    /// ---------------------
    /// Key Words
    /// ---------------------
    rule NULL() -> &'static str
        = ['n' | 'N'] ['u' | 'U'] ['l' | 'L'] ['l' | 'L'] { "null" }
    rule TRUE() -> &'static str
        = ['t' | 'T'] ['r' | 'R'] ['u' | 'U'] ['e' | 'E'] { "true" }
    rule FALSE() -> &'static str
        = ['f' | 'F'] ['a' | 'A'] ['l' | 'L'] ['s' | 'S'] ['e' | 'E'] { "false" }
    rule CREATE() -> &'static str
        = ['c' | 'C'] ['r' | 'R'] ['e' | 'E'] ['a' | 'A'] ['t' | 'T'] ['e' | 'E'] { "create" }
    rule DATABASE() -> &'static str
        = ['d' | 'D'] ['a' | 'A'] ['t' | 'T'] ['a' | 'A'] ['b' | 'B'] ['a' | 'A'] ['s' | 'S'] ['e' | 'E'] { "database" }
    rule VERTEX() -> &'static str
        = ['v' | 'V'] ['e' | 'E'] ['r' | 'R'] ['t' | 'T'] ['e' | 'E'] ['x' | 'X'] { "vertex" }
    rule TYPE() -> &'static str
        = ['t' | 'T'] ['y' | 'Y'] ['p' | 'P'] ['e' | 'E'] { "type" }
    rule EDGE() -> &'static str
        = ['e' | 'E'] ['d' | 'D'] ['g' | 'G'] ['e' | 'E'] { "edge" }
  }
}
