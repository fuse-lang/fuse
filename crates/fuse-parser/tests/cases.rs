use fuse_ast::{
    BindingPattern, BindingPatternKind, Statement, VariableDeclaration, VariableDeclarationKind,
};
use fuse_common::Span;
use fuse_parser::parse;

#[test]
fn test_simple() {
    assert_eq!(true, true);
    let ast = parse("const r#const = \"Hello, world\"");
    panic!("{ast:?}");
    // assert_eq!(
    //     ast.chunk.unwrap().body.statements[0],
    //     Statement::VariableDeclaration(VariableDeclaration {
    //         span: Span::new(0, 13),
    //         kind: VariableDeclarationKind::Const,
    //         binding: BindingPattern {
    //             kind: BindingPatternKind::Identifier,
    //             type_annotation: None,
    //             optional: false,
    //         }
    //     })
    // )
}
