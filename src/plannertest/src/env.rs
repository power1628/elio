use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
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

    pub fn plan_query(self: &Arc<Self>, ast: &ast::RegularQuery) -> anyhow::Result<String> {
        let plan = mojito_cypher::session::plan_query(self.clone(), ast)?;
        Ok(plan.explain())
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
    pub fn task_plan(&self, result: &mut String, cypher: &str, task: &str, opts: &TaskOption) -> anyhow::Result<()> {}
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
    async fn run(&mut self, test_case: &ParsedTestCase) -> anyhow::Result<String> {}
}
