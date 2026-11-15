use sigmaos_craft_vector::{VectorDocument, VectorPath, PathCommand};

#[test]
fn vector_document_integration_aaa() {
    let mut d = VectorDocument::new("My Doc");
    d.add_path(VectorPath { id: "p1".into(), commands: vec![PathCommand::MoveTo { x: 0.0, y: 0.0 }] }).unwrap();
    assert_eq!(d.path_count(), 1);
}
