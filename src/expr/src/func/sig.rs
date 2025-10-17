use std::collections::HashMap;

use mojito_common::data_type::DataType;

/// Function definition
pub struct FuncDef {
    pub name: String, // function name
    pub impls: Vec<FuncImpl>,
}

/// Function implementation
pub struct FuncImpl {
    pub args: Vec<FuncImplArg>,
    pub ret: FuncImplReturn,
    // TODO(power): add evaluatable function implementation
    // maybe we can hack the arrow-udf project
}

pub enum FuncImplArg {
    /// Exact argument type, e.g. `Int` in `map(Int, [1, 2, 3])`
    Exact(DataType),
    /// Templated argument type, e.g. `add<T>(T, T)`
    Templated(String),
}

pub enum FuncImplReturn {
    /// Exact return type, e.g. `add(Int, Int) -> Int`
    Exact(DataType),
    /// Templated return type, e.g. `add<T>(T, T) -> T`
    Templated(String),
}

pub type FunctionRegistry = HashMap<String, FuncDef>;
