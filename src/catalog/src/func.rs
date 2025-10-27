use mojito_expr::func::sig::FuncDef;

#[derive(Clone)]
pub struct FunctionCatalog {
    pub name: String,
    pub func: FuncDef,
    // TODO(pgao): other fields here
}
