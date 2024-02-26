use insta::assert_yaml_snapshot;
use std::{
    fs,
    path::{Path, PathBuf},
};

use fuse_parser::{lexer::Lexer, parse};

#[test]
fn pass() {
    assert_eq!(true, true);
    let ast = parse("const r#const = \"Hello, world\"");
}

// helpers
fn load_cases() -> Vec<PathBuf> {
    let dir = Path::new(file!()).to_path_buf();
    fs::read_dir(dir.parent().unwrap())
        .unwrap()
        .map(|x| x.unwrap())
        .filter(|x| x.metadata().is_ok_and(|meta| meta.is_dir()))
        .map(|node| node.path())
        .collect()
}

fn run(path: PathBuf, expect_panic: bool) {
    let source = fs::read_to_string(path.join("case.fuse")).unwrap();
    let snapshot = fs::read_to_string(path.join("snapshot.yaml")).unwrap();

    // let chunk = parse(source.as_str());
    // assert_yaml_snapshot!(chunk.)
}

fn test_lexer(source: &str, snapshot: &str) {
    let reference = source.clone();
    let tokens: Vec<_> = Lexer::new(&source).collect();

    assert_eq!(
        source, reference,
        "Lexer changes the content of the original source buffer."
    );

    assert_yaml_snapshot!(snapshot, tokens);
}
