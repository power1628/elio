use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use anyhow::Error;
use async_trait::async_trait;
use itertools::Itertools;
use mojito_catalog::FunctionCatalog;
use mojito_catalog::error::CatalogError;
use mojito_common::{TokenId, TokenKind};
use mojito_cypher::plan_context::PlanContext;
use mojito_cypher::session::PlannerSession;
use mojito_expr::func::FUNCTION_REGISTRY;
use mojito_parser::ast;
use sqlplannertest::ParsedTestCase;

#[derive(Hash, Debug, PartialEq, Eq)]
struct TokenKey {
    kind: TokenKind,
    key: String,
}

#[derive(Debug)]
pub struct MockCatalog {
    functions: HashMap<String, FunctionCatalog>,
    tokens: Mutex<HashMap<TokenKey, TokenId>>,
}

impl Default for MockCatalog {
    fn default() -> Self {
        let functions = FUNCTION_REGISTRY
            .name2def
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    FunctionCatalog {
                        name: k.clone(),
                        func: v.clone(),
                    },
                )
            })
            .collect();
        Self {
            functions,
            tokens: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct MockPlannerSession {
    catalog: Arc<MockCatalog>,
}

impl MockPlannerSession {
    pub fn parse(self: &Arc<Self>, cypher: &str) -> anyhow::Result<ast::Statement> {
        let ast = mojito_cypher::session::parse_statement(cypher)?;
        Ok(ast)
    }

    pub fn bind_query(self: &Arc<Self>, ast: &ast::RegularQuery) -> anyhow::Result<String> {
        let ir = mojito_cypher::session::bind_query(self.clone(), ast)?;
        Ok(ir.explain())
    }

    pub fn plan_query(self: &Arc<Self>, ast: &ast::RegularQuery) -> anyhow::Result<String> {
        let plan = mojito_cypher::session::plan_query(self.clone(), ast)?;
        Ok(plan.explain())
    }

    pub fn execute_ddl(self: &Arc<Self>, _ast: &ast::RegularQuery) -> anyhow::Result<()> {
        todo!()
    }
}

impl PlannerSession for MockPlannerSession {
    fn derive_plan_context(self: Arc<Self>) -> Arc<PlanContext> {
        Arc::new(PlanContext::new(self))
    }

    fn get_or_create_token(&self, token: &str, kind: TokenKind) -> Result<TokenId, CatalogError> {
        let key = TokenKey {
            kind,
            key: token.to_string(),
        };
        let mut tokens = self.catalog.tokens.lock().unwrap();
        let token_id = tokens.len() as TokenId;
        Ok(*tokens.entry(key).or_insert_with(|| token_id))
    }

    fn get_function_by_name(&self, name: &str) -> Option<&FunctionCatalog> {
        self.catalog.functions.get(name)
    }

    fn get_token_id(&self, token: &str, kind: TokenKind) -> Option<TokenId> {
        let key = TokenKey {
            kind,
            key: token.to_string(),
        };
        let tokens = self.catalog.tokens.lock().unwrap();
        tokens.get(&key).cloned()
    }

    fn send_notification(&self, _notification: String) {
        todo!()
    }
}

#[derive(Default)]
pub struct TestEnv {
    catalog: Arc<MockCatalog>,
}

impl TestEnv {
    pub fn new_session(&self) -> MockPlannerSession {
        MockPlannerSession {
            catalog: self.catalog.clone(),
        }
    }

    // generate plan
    pub fn task_plan(&self, result: &mut String, cypher: &str, task: &str, _opts: &TaskOption) -> anyhow::Result<()> {
        let session = Arc::new(self.new_session());
        let stmts = cypher.trim().split(';').collect_vec();
        for stmt in stmts {
            let ast = session.parse(stmt)?;
            match ast {
                ast::Statement::Query(regular_query) => {
                    let plan = session.plan_query(&regular_query)?;
                    result.push_str(&plan);
                    result.push('\n');
                }
                _ => return Err(Error::msg(format!("invalid cypher{} and task{}", stmt, task))),
            }
        }
        Ok(())
    }

    pub fn task_bind(&self, result: &mut String, cypher: &str, task: &str, _opts: &TaskOption) -> anyhow::Result<()> {
        let session = Arc::new(self.new_session());
        let stmts = cypher.trim().split(';').collect_vec();
        for stmt in stmts {
            let ast = session.parse(stmt)?;
            match ast {
                ast::Statement::Query(regular_query) => {
                    let ir = session.bind_query(&regular_query)?;
                    result.push_str(&ir);
                    result.push('\n');
                }
                _ => return Err(Error::msg(format!("invalid cypher{} and task{}", stmt, task))),
            }
        }
        Ok(())
    }
}

// TODO(pgao): task options
#[derive(Debug, Default)]
pub struct TaskOption {
    rules: Vec<String>,
}
/// Available tasks
///   - plan: generate plan
#[async_trait]
impl sqlplannertest::PlannerTestRunner for TestEnv {
    /// Run a test case and return the result
    async fn run(&mut self, test_case: &ParsedTestCase) -> anyhow::Result<String> {
        let mut result = String::new();
        for task in test_case.tasks.iter() {
            if task == "plan" {
                self.task_plan(&mut result, &test_case.sql, task, &TaskOption::default())?;
            }
            if task == "bind" {
                self.task_bind(&mut result, &test_case.sql, task, &TaskOption::default())?;
            }
        }
        Ok(result)
    }
}
