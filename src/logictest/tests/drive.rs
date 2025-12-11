extern crate logictest;

use std::path::Path;

use logictest::graphdb::{EmbeddedGraphDB, graphdb_column_validator};
use sqllogictest::runner::Runner;
use sqllogictest::{default_normalizer, default_validator};
use tempfile::tempdir;

datatest_stable::harness! {{test=run_slt_file, root="tests/basic/", pattern=r".*"},}

fn run_slt_file(path: &Path) -> datatest_stable::Result<()> {
    let make_conn = || async {
        let temp_dir = tempdir().expect("failed to create temp dir for testing db");
        let db = EmbeddedGraphDB::open(temp_dir)?;
        Ok(db)
    };

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime");

    let mut runner = Runner::new(make_conn);

    // get env, if REWRITE=1, call run_file_async, else call run_file
    let rewrite = std::env::var("REWRITE").is_ok();
    if rewrite {
        let col_separator = "\t";
        let validator = default_validator;
        let normalizer = default_normalizer;
        let column_type_validator = graphdb_column_validator;
        rt.block_on(runner.update_test_file(path, col_separator, validator, normalizer, column_type_validator))?;
    } else {
        rt.block_on(runner.run_file_async(path))?;
    }

    Ok(())
}
