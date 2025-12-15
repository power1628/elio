use indexmap::IndexSet;
use mojito_common::SemanticDirection;
use mojito_common::data_type::DataType;
use mojito_common::variable::VariableName;

use crate::expr::ExprNode;

// output virtual path only
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProjectPath {
    pub steps: Vec<PathStep>,
    // path variable
    pub variable: VariableName,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, derive_more::Display)]
pub enum PathStep {
    #[display("({})", _0)]
    NodeStep(VariableName),
    #[display("{ldir}[{rel}]{rdir}({other})", ldir = direction.l_arrow(), rdir = direction.r_arrow())]
    SingleRelStep {
        rel: VariableName,
        direction: SemanticDirection,
        other: VariableName,
    },
    #[display("{ldir}[{rel}*]{rdir}({other})", ldir = direction.l_arrow(), rdir = direction.r_arrow())]
    MutliRelStep {
        rel: VariableName,
        direction: SemanticDirection,
        other: VariableName,
    },
}

impl PathStep {
    pub fn used_variable(&self) -> IndexSet<VariableName> {
        let mut set = IndexSet::new();
        match self {
            Self::NodeStep(var) => {
                set.insert(var.clone());
            }
            Self::SingleRelStep { rel, other, .. } => {
                set.insert(rel.clone());
                set.insert(other.clone());
            }
            Self::MutliRelStep { rel, other, .. } => {
                set.insert(rel.clone());
                set.insert(other.clone());
            }
        }
        set
    }
}

impl ExprNode for ProjectPath {
    fn typ(&self) -> DataType {
        DataType::VirtualPath
    }
}

impl ProjectPath {
    pub fn used_variable(&self) -> IndexSet<VariableName> {
        let mut set = IndexSet::new();
        for step in &self.steps {
            set.extend(step.used_variable());
        }
        set
    }

    pub fn pretty(&self) -> String {
        format!(
            "{} = {}",
            self.variable,
            self.steps.iter().map(|s| s.to_string()).collect::<Vec<_>>().join("")
        )
    }
}
