//! Function catalog item

/// When system startup we should add builtin function to the catalog
pub struct FunctionCatalog {
    /// Function name
    pub name: String,
    /// Function definition
    pub func: FuncDef,
    // TODO(pgao): priviledge and owner here
}
