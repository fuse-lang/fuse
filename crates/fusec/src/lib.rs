use fuse_parser::Parser;
use fuse_semantic::Semantic;

fn compile_chunk(source: &str) {
    let parsed = Parser::new(source).parse();
    let mut chunk = parsed.chunk.unwrap();
    let semantic = Semantic::new(source).build(&mut chunk);
    // panic!("{:#?}", chunk)
}

#[test]
fn manual_test() {
    compile_chunk(
        r#"
        let z = 123
        let x = 123
        let y = x
        fn x()
            let x = y
        end

        x()
        "#,
    )
}
