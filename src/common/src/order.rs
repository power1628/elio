use crate::variable::VariableName;
use derive_more;

#[derive(Clone, Debug, Copy, Default, PartialEq, Eq, derive_more::Display)]
pub enum SortDirection {
    #[default]
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
pub struct ColumnOrder {
    pub column: VariableName,
    pub direction: SortDirection,
}
