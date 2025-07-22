use mojito_common::{Label, PropertyKey};
use mojito_storage::{
    graph_store::{GraphStore, GraphStoreConfig},
    types::PropertyValue,
};

fn create_tempfile() -> tempfile::NamedTempFile {
    tempfile::NamedTempFile::new().unwrap()
}

fn make_labels() -> Vec<Label> {
    vec![Label::from("Person"), Label::from("Message")]
}

fn make_properties() -> Vec<(PropertyKey, PropertyValue)> {
    vec![
        (PropertyKey::from("name"), PropertyValue::String("alex".to_string())),
        (PropertyKey::from("age"), PropertyValue::Integer(18)),
    ]
}

#[test]
fn test_node_create() {
    let tempfile = create_tempfile();
    let config = GraphStoreConfig {
        path: tempfile.path().to_str().unwrap().to_string(),
    };
    let graph_store = GraphStore::open(&config).unwrap();

    let mut tx = graph_store.begin_write().unwrap();
    let labels = make_labels();
    let properties = make_properties();
    let node_id = tx.node_create(labels, properties).unwrap();
    tx.commit().unwrap();
    assert_eq!(node_id, 0);
}
