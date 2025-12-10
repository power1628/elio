use std::collections::HashMap;
use std::sync::Arc;

use clap::Parser;
use futures::stream::StreamExt;
use mojito_core::db_env::{DbConfig, DbEnv};
use mojito_core::session::Session;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "db path", default_value = ".db")]
    db_path: String,
}

async fn execute_cypher(sess: &Arc<Session>, cypher: &str) {
    println!("execute {}", cypher);
    let mut stream = sess.execute(cypher.to_string(), HashMap::new()).await.unwrap();
    while let Some(row) = stream.next().await {
        println!("{:?}", row);
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let db_config = DbConfig::with_db_path(args.db_path);

    let db = DbEnv::open(&db_config).unwrap();

    let sess = db.new_session();

    let q1 = "CREATE (n:Person {name: 'Alice', age: 30}) RETURN *";
    let q2 = "MATCH (n) RETURN n";
    let q3 = "CREATE (a:Person {name: 'Alice', age: 30}), (b:Person {name: 'Bob', age: 31}), (a)-[r:KNOWS]->(b)";

    execute_cypher(&sess, q1).await;
    println!("---");
    execute_cypher(&sess, q2).await;
    println!("---");
    execute_cypher(&sess, q3).await;
}
