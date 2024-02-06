use fuse_parser::parse;

#[test]
fn test_simple() {
    let ast = parse("const x = 123").unwrap();
    assert_eq!(ast, true)
}
