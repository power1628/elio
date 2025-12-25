use mojito_common::data_type::DataType;

use crate::impl_::func_call::FunctionImpl;

/// Function definition
#[derive(Clone, Debug)]
pub struct FuncDef {
    pub name: String, // function name
    pub impls: Vec<FuncImpl>,
    pub is_agg: bool,
    // TODO: propagate nulls
}

/// Function implementation
#[derive(Clone, Debug)]
pub struct FuncImpl {
    // signature id, used to identify the function implementation
    // is the hash of args type.
    pub func_id: String,
    pub args: Vec<FuncImplArg>,
    pub ret: FuncImplReturn,
    // function pointer which is invoked when the function is called
    pub func: FunctionImpl,
}

impl FuncImpl {
    pub fn new(name: &str, args: Vec<FuncImplArg>, ret: FuncImplReturn, func: FunctionImpl) -> Self {
        let signature_id = Self::compute_signature(name, &args);
        Self {
            func_id: signature_id,
            args,
            ret,
            func,
        }
    }

    fn compute_signature(name: &str, args: &[FuncImplArg]) -> String {
        format!(
            "{}({})",
            name,
            args.iter().map(|arg| arg.signature()).collect::<Vec<_>>().join(", ")
        )
    }
}

impl FuncImpl {
    // if match, return the return type
    pub fn matches(&self, args: &[DataType]) -> Option<DataType> {
        if self.args.len() != args.len() {
            return None;
        }
        for (i, arg) in self.args.iter().enumerate() {
            match arg {
                FuncImplArg::Union(types) => {
                    if !types.contains(&args[i]) {
                        return None;
                    }
                }
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

#[derive(Clone, Debug, Hash)]
pub enum FuncImplArg {
    // UNION type will be mapped into Any data type when execution
    // here we use union type here only for semantic checking
    // the actual function implementation siguare will be Any
    Union(Vec<DataType>),
    /// Exact argument type, e.g. `Int` in `map(Int, [1, 2, 3])`
    Exact(DataType),
    /// Templated argument type, e.g. `add<T>(T, T)`
    Templated(String),
}

impl FuncImplArg {
    pub fn signature(&self) -> String {
        match self {
            FuncImplArg::Union(types) => {
                format!(
                    "anyof {}",
                    types.iter().map(|dt| dt.signature()).collect::<Vec<_>>().join(" | ")
                )
            }
            FuncImplArg::Exact(dt) => dt.signature(),
            FuncImplArg::Templated(t) => format!("{}<T>", t),
        }
    }
}

#[derive(Clone, Debug)]
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

#[macro_export]
macro_rules! func_impl_arg {
            ({ exact $dt:ident }) => {
                FuncImplArg::Exact(DataType::$dt)
            };

            ({ anyof $($dt:ident)|+ }) => {
                FuncImplArg::Union(vec![$(DataType::$dt),+])
            };
        }

// generate function implementation
#[macro_export]
macro_rules! define_function {
    (name: $name:expr,
     impls: [
        $({
            args: [$($arg_type:tt),*],
            ret: $ret_type:ident,
            func: $func_impl:ident
        }),+
     ],
     is_agg: $is_agg:expr
    ) => {{
        $crate::func::sig::FuncDef{
            name: $name.to_string(),
            impls: vec![
                $({
                    use $crate::func::sig::{FuncImplArg, FuncImplReturn};
                    use mojito_common::data_type::DataType;
                    use $crate::func_impl_arg;

                    $crate::func::sig::FuncImpl::new(
                        $name,
                        vec![$(func_impl_arg!($arg_type)),*],
                        FuncImplReturn::Exact(DataType::$ret_type),
                        $func_impl,
                    )
                },)+
            ],
            is_agg: $is_agg,
        }
    }};
}
