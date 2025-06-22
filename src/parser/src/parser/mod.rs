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
        = CREATE() _ VERTEX() _ TYPE() _ not_exists:if_not_exists() _ name:ident() _ "(" _ column_or_constraint:(column_def_or_constraint() ** comma_separator()) _ ")"  _ options:with_attribute_list()? {
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
        = CREATE() _ EDGE() _ TYPE() _ not_exists:if_not_exists() _ name:ident() _ "(" _ FROM() _ from:ident() _ "," _ TO() _ to:ident() _ "," _ column_or_constraint:(column_def_or_constraint() ** comma_separator()) _ ")" _  options:with_attribute_list()? {
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
        = name:ident() _ typ:data_type() nullable:column_nullable()?  {
            ColumnDef {
                name: name.to_string(),
                typ,
                nullable: nullable.unwrap_or(true),
            }
        }

    rule constraint_spec() -> ConstraintSpec
        =  "PRIMARY" _ "KEY" _ "(" _ columns:(ident() ** ",") _ ")"  {
            ConstraintSpec::PrimaryKey{columns: columns.into_iter().map(|c| c.to_string()).collect() }
        }
        /  "PRIMARY" _ "KEY" _ ident:ident()  {
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
            left:(@) _ OR() _ right:@ {Expr::new_binary(left, BinaryOperator::Or, right)}
            --
            left:(@) _ XOR() _ right:@ {Expr::new_binary(left, BinaryOperator::Xor, right)}
            --
            left:(@) _ AND() _ right:@ {Expr::new_binary(left, BinaryOperator::And, right)}
            --
                       NOT() _ right:@ {Expr::new_unary(UnaryOperator::Not, right)}
            --
            left:(@) _ op:$("=" / "!=" / "<>" / "<" / "<=" / ">" / ">=") _ right:@ {
                let operator = match op {
                    "=" => BinaryOperator::Eq,
                    "!="  | "<>" => BinaryOperator::NotEq,
                    "<" => BinaryOperator::Lt,
                    "<=" => BinaryOperator::LtEq,
                    ">" => BinaryOperator::Gt,
                    ">=" => BinaryOperator::GtEq,
                    _ => unreachable!(),
                };
                Expr::new_binary(left, operator, right)
            }
            --
            left:(@) _ op:null_predicate() _ right:@ {
                Expr::new_unary(op, right)
            }
            --
            left:(@) _ op:$("+" / "-" / "||" ) _ right:@ {
                let operator = match op {
                    "+" => BinaryOperator::Add,
                    "-" => BinaryOperator::Subtract,
                    "||" => BinaryOperator::Concat,
                    _ => unreachable!(),
                };
                Expr::new_binary(left, operator, right)
            }
            --
            left:(@) _ op:$("*" / "/" / "%") _ right:@ {
                let operator = match op {
                    "*" => BinaryOperator::Multiply,
                    "/" => BinaryOperator::Divide,
                    "%" => BinaryOperator::Modulo,
                    _ => unreachable!(),
                };
                Expr::new_binary(left, operator, right)
            }
            --
            left:(@) _ op:$("^") _ right:@ {
                Expr::new_binary(left, BinaryOperator::Pow, right)
            }
            --
                       op:$("+" / "-") _ right:@ {
                let operator = match op {
                    "+" => UnaryOperator::UnaryAdd,
                    "-" => UnaryOperator::UnarySubtract,
                    _ => unreachable!(),
                };
                Expr::new_unary(operator, right)
            }
            --
            // postfix
            // property_access
            left:(@) "." key:ident() { Expr::new_property_access(left, key.to_string()) }
            --
            // atom
            a:atom() { a }
        }

    rule atom() -> Expr
        = l:literal() { l }
        / "$" v:ident() { Expr::new_parameter(v.to_string()) }
        / "(" _ e:expr() _ ")" { e }
        / f:function_call() { f }
        / v:variable() { v }

    rule literal() -> Expr
        = b:(TRUE() / FALSE()) { Expr::new_boolean(b == "TRUE") }
        / f:float_literal() { Expr::new_float(f.to_string()) }
        / i:integer_literal() { Expr::new_integer(i.to_string()) }
        / s:string_literal() { Expr::new_string(s.to_string()) }
        / n:null_literal() { n }


    rule function_call() -> Expr
        = name:ident() _ "(" _ args:( expr() ** comma_separator()) _ ")" {
            Expr::new_function_call(name.to_string(), args)
        }

    rule variable() -> Expr
        = v:ident() { Expr::Varaible(v.to_string()) }

    rule null_predicate() -> UnaryOperator
        = is_null() / is_not_null()
    rule is_null() -> UnaryOperator
        = IS() _ NULL() { UnaryOperator::IsNull }
    rule is_not_null() -> UnaryOperator
        = IS() _ NOT() _ NULL() { UnaryOperator::IsNotNull }

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
    /// Label Expression
    /// ---------------------
    pub rule label_expr() -> LabelExpr
        = ":" _ expr:label_expr_inner() {
            expr
        }

    rule label_expr_inner() -> LabelExpr
        = precedence! {
            left:(@) _ op:$("|") _ right:@ { LabelExpr::new_or(left, right)}
            --
            label:ident() { LabelExpr::new_label(label.to_string()) }
        }


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
        = "(" _ kvs:(option_kv() ** comma_separator()) _ ")" { kvs }
    rule option_kv() -> OptionKV
        =  key:ident() _ ":" _ value:expr() { OptionKV { name: key.to_string(), value: Box::new(value) } }

    rule comma_separator() = _ "," _

    /// ---------------------
    /// Key Words
    /// ---------------------
    rule IS() -> &'static str
        = ['i' | 'I'] ['s' | 'S'] { "IS" }
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

    // operator
    rule OR() -> &'static str
        = ['o' | 'O'] ['r' | 'R'] { "OR" }
    rule XOR() -> &'static str
        = ['x' | 'X'] ['o' | 'O'] ['r' | 'R'] { "XOR" }
    rule AND() -> &'static str
        = ['a' | 'A'] ['n' | 'N'] ['d' | 'D'] { "AND" }

  }
}
