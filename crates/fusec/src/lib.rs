use fuse_parser::Parser;
use fuse_semantic::Semantic;

fn compile_chunk(source: &str) {
    let parsed = Parser::new(source).parse();
    let semantic = Semantic::new(source, parsed.chunk.unwrap());
}

#[test]
fn manual_test() {
    compile_chunk(r#"
        let x = 123
        let y = x
        "#)
}
