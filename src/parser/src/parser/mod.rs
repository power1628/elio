peg::parser! {
  pub grammar cypher_parser() for str {
    use crate::ast::*;
    use either::Either;

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
        = CREATE() _ DATABASE() _ not_exists:if_not_exists() _ db_name:ident() _ options:with_attribute_list()? {
            Statement::CreateDatabase(Box::new(CreateDatabase {
                db_name: db_name.to_string(),
                not_exists: false,
                options: options.unwrap_or_default(),
            }))
        }

    /// create vertex type statement
    /// CREATE VERTEX TYPE IF NOT EXISTS name (column1 type1 nullable, column2 type2 nullable, PRIMARY KEY (column1))
    /// WITH (option1: value1, option2: value2)
    pub rule create_vertex_type() -> Statement
        = CREATE() _ VERTEX() _ TYPE() _ not_exists:if_not_exists() _ name:ident() _ "(" _ column_or_constraint:(column_def_or_constraint() ** ",") _ ")"  _ options:with_attribute_list()? {
            let (columns, constraints) = {
                let mut columns = vec![];
                let mut constraints = vec![];
                for item in column_or_constraint {
                    match item {
                        Either::Left(col) => columns.push(col),
                        Either::Right(constraint) => constraints.push(constraint),
                    }
                }
                (columns, constraints)
            };

            Statement::CreateVertexType(Box::new(CreateVertexType {
                name: name.to_string(),
                not_exists: false,
                columns,
                constrait: constraints,
                options: options.unwrap_or_default(),
            }))
        }

    /// create edge type statement
    /// CREATE EDGE TYPE IF NOT EXISTS name (FROM from_vertex_type, TO to_vertex_type, column1 type1 nullable, column2 type2 nullable, PRIMARY KEY (column1))
    /// WITH (option1: value1, option2: value2)
    pub rule create_edge_type() -> Statement
        = CREATE() _ EDGE() _ TYPE() _ not_exists:if_not_exists() _ name:ident() _ "(" _ FROM() _ from:ident() _ "," _ TO() _ to:ident() _ "," _ column_or_constraint:(column_def_or_constraint() ** ",") _ ")" _  options:with_attribute_list()? {
            let (columns, constraints) = {
                let mut columns = vec![];
                let mut constraints = vec![];
                for item in column_or_constraint {
                    match item {
                        Either::Left(col) => columns.push(col),
                        Either::Right(constraint) => constraints.push(constraint),
                    }
                }
                (columns, constraints)
            };

            Statement::CreateEdgeType(Box::new(CreateEdgeType {
                name: name.to_string(),
                not_exists: false,
                from: from.to_string(),
                to: to.to_string(),
                columns,
                constrait: constraints,
                options: options.unwrap_or_default(),
            }))
        }

    rule if_not_exists() -> bool
        = IF() _ NOT() _ EXISTS()   { true }
          / { false }

    rule with_attribute_list() -> Vec<OptionKV>
        = WITH() _ kvs:attribute_list() { kvs }

    rule column_def_or_constraint() -> Either<ColumnDef,ConstraintSpec>
        = def:column_def() { Either::Left(def) }
        / constraint:constraint_spec() { Either::Right(constraint) }

    rule column_def() -> ColumnDef
        = _ name:ident() _ typ:data_type() nullable:column_nullable()? _ {
            ColumnDef {
                name: name.to_string(),
                typ,
                nullable: nullable.unwrap_or(true),
            }
        }

    rule constraint_spec() -> ConstraintSpec
        = _ "PRIMARY" _ "KEY" _ "(" _ columns:(ident() ** ",") _ ")" _ {
            ConstraintSpec::PrimaryKey{columns: columns.into_iter().map(|c| c.to_string()).collect() }
        }
        / _ "PRIMARY" _ "KEY" _ ident:ident() _ {
            ConstraintSpec::PrimaryKey{columns: vec![ident.to_string()] }
        }

    rule column_nullable() -> bool
        = _ NOT() _ NULL() { false }
        / _ NULL() { true }

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
        = "'" content:$((!['\''] [_])* ) "'" { content }
        / "\"" content:$((!['\"'] [_])* ) "\"" { content }
    rule null_literal() -> Expr
        = NULL() { Expr::new_null() }

    /// ---------------------
    /// Data Types
    /// ---------------------
    rule data_type() -> DataType
        = INTEGER() { DataType::Integer }
        / FLOAT() { DataType::Float }
        / STRING() { DataType::String }
        / BOOLEAN() { DataType::Boolean }



    /// ---------------------
    /// Common
    /// ---------------------
    rule number() -> &'input str
        = first:$(['0'..='9']+) { first }

    rule ident() -> &'input str
        = first:$(['a'..='z' | 'A'..='Z'] ['a'..='z' | 'A'..='Z' | '0'..='9']*) { first }

    rule attribute_list() -> Vec<OptionKV>
        = "(" _ kvs:(option_kv() ** ",") _ ")" { kvs }
    rule option_kv() -> OptionKV
        = _ key:ident() _ ":" _ value:expr() _ { OptionKV { name: key.to_string(), value: Box::new(value) } }



    /// ---------------------
    /// Key Words
    /// ---------------------
    rule NULL() -> &'static str
        = ['n' | 'N'] ['u' | 'U'] ['l' | 'L'] ['l' | 'L'] { "NULL" }
    rule TRUE() -> &'static str
        = ['t' | 'T'] ['r' | 'R'] ['u' | 'U'] ['e' | 'E'] { "TRUE" }
    rule FALSE() -> &'static str
        = ['f' | 'F'] ['a' | 'A'] ['l' | 'L'] ['s' | 'S'] ['e' | 'E'] { "FALSE" }
    rule CREATE() -> &'static str
        = ['c' | 'C'] ['r' | 'R'] ['e' | 'E'] ['a' | 'A'] ['t' | 'T'] ['e' | 'E'] { "CREATE" }
    rule DATABASE() -> &'static str
        = ['d' | 'D'] ['a' | 'A'] ['t' | 'T'] ['a' | 'A'] ['b' | 'B'] ['a' | 'A'] ['s' | 'S'] ['e' | 'E'] { "DATABASE" }
    rule VERTEX() -> &'static str
        = ['v' | 'V'] ['e' | 'E'] ['r' | 'R'] ['t' | 'T'] ['e' | 'E'] ['x' | 'X'] { "VERTEX" }
    rule TYPE() -> &'static str
        = ['t' | 'T'] ['y' | 'Y'] ['p' | 'P'] ['e' | 'E'] { "TYPE" }
    rule EDGE() -> &'static str
        = ['e' | 'E'] ['d' | 'D'] ['g' | 'G'] ['e' | 'E'] { "EDGE" }
    rule WITH() -> &'static str
        = ['w' | 'W'] ['i' | 'I'] ['t' | 'T'] ['h' | 'H'] { "WITH" }
    rule IF() -> &'static str
        = ['i' | 'I'] ['f' | 'F'] { "IF" }
    rule NOT() -> &'static str
        = ['n' | 'N'] ['o' | 'O'] ['t' | 'T'] { "NOT" }
    rule EXISTS() -> &'static str
        = ['e' | 'E'] ['x' | 'X'] ['i' | 'I'] ['s' | 'S'] ['t' | 'T'] ['s' | 'S'] { "EXISTS" }

    rule INTEGER() -> &'static str
        = ['i' | 'I'] ['n' | 'N'] ['t' | 'T'] ['e' | 'E'] ['g' | 'G'] ['e' | 'E'] ['r' | 'R'] { "INTEGER" }
    rule FLOAT() -> &'static str
        = ['f' | 'F'] ['l' | 'L'] ['o' | 'O'] ['a' | 'A'] ['t' | 'T'] { "FLOAT" }
    rule STRING() -> &'static str
        = ['s' | 'S'] ['t' | 'T'] ['r' | 'R'] ['i' | 'I'] ['n' | 'N'] ['g' | 'G'] { "STRING" }
    rule BOOLEAN() -> &'static str
        = ['b' | 'B'] ['o' | 'O'] ['o' | 'O'] ['l' | 'L'] ['e' | 'E'] ['a' | 'A'] ['n' | 'N'] { "BOOLEAN" }

    rule PRIMARY() -> &'static str
        = ['p' | 'P'] ['r' | 'R'] ['i' | 'I'] ['m' | 'M'] ['a' | 'A'] ['r' | 'R'] ['y' | 'Y'] { "PRIMARY" }
    rule KEY() -> &'static str
        = ['k' | 'K'] ['e' | 'E'] ['y' | 'Y'] { "KEY" }

    rule FROM() -> &'static str
        = ['f' | 'F'] ['r' | 'R'] ['o' | 'O'] ['m' | 'M'] { "FROM" }
    rule TO() -> &'static str
        = ['t' | 'T'] ['o' | 'O'] { "TO" }
  }
}
