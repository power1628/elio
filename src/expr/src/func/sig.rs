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
                    // special handle for any, we think any type is equal to any other type
                    if dt != &args[i] && *dt != DataType::Any {
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

    /// Match function signature with null coercion support.
    ///
    /// For arguments marked as untyped null, this method will try to infer
    /// their types from the function signature.
    ///
    /// # Arguments
    /// * `args` - The argument types
    /// * `is_untyped_null` - Boolean array indicating which arguments are untyped nulls
    ///
    /// # Returns
    /// * `Some((return_type, coerced_arg_types))` if match succeeds
    /// * `None` if match fails
    pub fn matches_with_null_coercion(
        &self,
        args: &[DataType],
        is_untyped_null: &[bool],
    ) -> Option<(DataType, Vec<DataType>)> {
        if self.args.len() != args.len() || self.args.len() != is_untyped_null.len() {
            return None;
        }

        let mut coerced_types = Vec::with_capacity(args.len());

        for (i, func_arg) in self.args.iter().enumerate() {
            let arg_type = if is_untyped_null[i] {
                // For untyped null, infer type from function signature
                match func_arg {
                    FuncImplArg::Exact(dt) => dt.clone(),
                    FuncImplArg::Union(types) => {
                        // Use first type in union as default
                        types.first().cloned().unwrap_or(DataType::Any)
                    }
                    FuncImplArg::Templated(_) => {
                        // For templated args, keep as Any for now
                        DataType::Any
                    }
                }
            } else {
                // Use original type
                args[i].clone()
            };

            // Check if the coerced type matches the function signature
            match func_arg {
                FuncImplArg::Union(types) => {
                    if !types.contains(&arg_type) {
                        return None;
                    }
                }
                FuncImplArg::Exact(dt) => {
                    if dt != &arg_type && *dt != DataType::Any {
                        return None;
                    }
                }
                FuncImplArg::Templated(_) => {
                    // TODO(power): add type inference for templated arguments
                    unimplemented!()
                }
            }

            coerced_types.push(arg_type);
        }

        Some((self.ret.resolve_ret(&coerced_types), coerced_types))
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
        $crate::func::sig::FuncImplArg::Exact(DataType::$dt)
    };

    ({ anyof $($dt:ident)|+ }) => {
        $crate::func::sig::FuncImplArg::Union(vec![$(DataType::$dt),+])
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
                    use $crate::func::sig::{FuncImplReturn};
                    use mojito_common::data_type::DataType;

                    $crate::func::sig::FuncImpl::new(
                        $name,
                        vec![$($crate::func_impl_arg!($arg_type)),*],
                        FuncImplReturn::Exact(DataType::$ret_type),
                        $func_impl,
                    )
                },)+
            ],
            is_agg: $is_agg,
        }
    }};
}
