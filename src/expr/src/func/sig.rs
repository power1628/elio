use std::collections::HashMap;

use mojito_common::data_type::DataType;

/// Function definition
#[derive(Clone)]
pub struct FuncDef {
    pub name: String, // function name
    pub impls: Vec<FuncImpl>,
    pub is_agg: bool,
}

/// Function implementation
#[derive(Clone)]
pub struct FuncImpl {
    pub args: Vec<FuncImplArg>,
    pub ret: FuncImplReturn,
    // TODO(power): add evaluatable function implementation
    // maybe we can hack the arrow-udf project
}

impl FuncImpl {
    // if match, return the return type
    pub fn matches(&self, args: &[DataType]) -> Option<DataType> {
        if self.args.len() != args.len() {
            return None;
        }
        for (i, arg) in self.args.iter().enumerate() {
            match arg {
                FuncImplArg::Exact(dt) => {
                    if dt != &args[i] {
                        return None;
                    }
                }
                FuncImplArg::Templated(_) => {
                    // TODO(power): add type inference for templated arguments
                    unimplemented!()
                }
            }
        }
        Some(self.ret.resolve_ret(args))
    }
}

#[derive(Clone)]
pub enum FuncImplArg {
    /// Exact argument type, e.g. `Int` in `map(Int, [1, 2, 3])`
    Exact(DataType),
    /// Templated argument type, e.g. `add<T>(T, T)`
    Templated(String),
}

#[derive(Clone)]
pub enum FuncImplReturn {
    /// Exact return type, e.g. `add(Int, Int) -> Int`
    Exact(DataType),
    /// Templated return type, e.g. `add<T>(T, T) -> T`
    Templated(String),
}

impl FuncImplReturn {
    // resolve return type
    pub fn resolve_ret(&self, _args: &[DataType]) -> DataType {
        match self {
            FuncImplReturn::Exact(data_type) => data_type.clone(),
            FuncImplReturn::Templated(_t) => {
                // TODO(power): add type inference for templated return type
                unimplemented!()
            }
        }
    }
}

pub type FunctionRegistry = HashMap<String, FuncDef>;
