use indexmap::IndexSet;
use mojito_common::variable::VariableName;

pub use super::*;

impl Expr {
    pub fn collect_variables(&self) -> IndexSet<VariableName> {
        let mut vars = IndexSet::new();
        match self {
            Expr::VariableRef(variable_ref) => {
                vars.insert(variable_ref.name.clone());
            }
            Expr::PropertyAccess(property_access) => vars.extend(property_access.expr.collect_variables()),
            Expr::Constant(_) => {}
            Expr::FuncCall(func_call) => {
                vars.extend(func_call.args.iter().flat_map(|arg| arg.collect_variables()));
            }
            Expr::AggCall(agg_call) => {
                vars.extend(agg_call.args.iter().flat_map(|arg| arg.collect_variables()));
            }
            Expr::Subquery(_) => todo!("subquery not supported"),
            Expr::LabelExpr(label_expr) => {
                vars.extend(label_expr.entity.collect_variables());
            }
            Expr::CreateMap(CreateMap { properties }) => {
                vars.extend(properties.iter().flat_map(|(_, expr)| expr.collect_variables()));
            }
        }
        vars
    }

    pub fn depend_only_on(&self, vars: &IndexSet<VariableName>) -> bool {
        self.collect_variables().is_subset(vars)
    }
}
