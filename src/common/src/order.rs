use crate::variable::VariableName;

#[derive(Clone, Debug, Copy, Default, PartialEq, Eq)]
pub enum Direction {
    #[default]
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
pub struct ColumnOrder {
    pub column: VariableName,
    pub direction: Direction,
}
