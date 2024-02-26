use insta::assert_yaml_snapshot;
use std::{ffi::OsStr, fs, path::PathBuf};

use fuse_parser::{lexer::Lexer, parse};

#[test]
fn pass() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // TODO: better way for pointing to the test cases directory.
    let cases = load_cases(&root.join("tests").join("cases").join("pass"));

    for case in cases {
        run(case, false, false);
    }
}

// helpers
fn load_cases(dir: &PathBuf) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .unwrap()
        .map(|x| x.unwrap())
        .filter(|x| x.metadata().is_ok_and(|meta| meta.is_dir()))
        .map(|node| node.path())
        .collect()
}

fn run(path: PathBuf, expect_error: bool, expect_panic: bool) {
    let path = path.join("case.fuse");

    test_lexer(path.as_os_str());
    test_parser(path.as_os_str(), expect_error, expect_panic);
}

fn test_lexer(src_path: &OsStr) {
    let source = fs::read_to_string(src_path).unwrap();
    let reference = source.clone();
    let tokens: Vec<_> = Lexer::new(&source).collect();

    assert_eq!(
        source, reference,
        "Lexer changes the content of the original source buffer."
    );

    assert_yaml_snapshot!("tokens", tokens);
}

fn test_parser(path: &OsStr, expect_error: bool, expect_panic: bool) {
    let source = fs::read_to_string(path).unwrap();
    let reference = source.clone();
    let parsed = parse(source.as_str());

    assert_eq!(
        source, reference,
        "Lexer changes the content of the original source buffer."
    );

    assert_eq!(
        expect_panic, parsed.paniced,
        "Panic state ({}) was different from the expected value ({}).",
        parsed.paniced, expect_panic
    );

    assert_eq!(
        expect_error,
        !parsed.errors.is_empty(),
        "Error vector is different from expectations."
    );

    let chunk = parsed.chunk;
    let errors = parsed.errors;

    assert_yaml_snapshot!("ast", chunk);
}
