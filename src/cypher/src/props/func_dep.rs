use std::collections::HashSet;

use mojito_common::variable::VariableName;

#[derive(Debug, Clone)]
pub struct FuncDepSet {
    pub deps: Vec<FuncDep>,
}

#[derive(Debug, Clone)]
pub struct FuncDep {
    pub from: HashSet<VariableName>,
    pub to: HashSet<VariableName>,
    pub equiv: bool,
}
