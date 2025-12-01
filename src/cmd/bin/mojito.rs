use std::collections::HashMap;

use clap::Parser;
use mojito_core::db_env::{DbConfig, DbEnv};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "db path", default_value = "db")]
    db_path: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let db_config = DbConfig::with_db_path(args.db_path);

    let db = DbEnv::open(&db_config).unwrap();

    let sess = db.new_session();

    let query = "CREATE (n:Person {name: 'Alice', age: 30})";

    sess.execute(query.to_string(), HashMap::new()).await.unwrap();
}
