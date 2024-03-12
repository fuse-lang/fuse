use fuse_parser::Parser;
use fuse_inference::Inference;

fn compile_chunk(source: &str) {
    let parsed = Parser::new(source).parse();
    assert!(!parsed.paniced);
    assert!(parsed.errors.len() == 0);
    let mut chunk = parsed.chunk.unwrap();
    let inference = Inference::new(source).resolve(&mut chunk);
    // panic!("{:#?}", chunk)
}

#[test]
fn manual_test() {
    compile_chunk(
        r#"
        let a = 0
        let d = a.b.c
        "#,
    )
}
