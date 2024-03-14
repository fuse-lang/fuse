use fuse_parser::Parser;
use fuse_semantic::Semantic;

fn compile_chunk(source: &str) {
    let parsed = Parser::new(source).parse();
    assert!(!parsed.paniced);
    assert!(parsed.errors.len() == 0);
    let mut chunk = parsed.chunk.unwrap();
    let semantic = Semantic::new(source).build(&mut chunk);
    // panic!("{:#?}", chunk)
}

#[test]
fn manual_test() {
    compile_chunk(
        r#"
        let a = 0
        let c = 1
        let d = a.b.c
        "#,
    )
}
