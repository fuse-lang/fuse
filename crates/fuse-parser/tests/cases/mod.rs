use std::{ffi::OsStr, fs, path::PathBuf};

use fuse_parser::{lexer::Lexer, parse};

struct Context<'a> {
    root: PathBuf,
    test_dir: PathBuf,
    source_name: &'a str,
    settings: insta::Settings,
}

impl<'a> Context<'a> {
    fn path(&self) -> PathBuf {
        self.root.join(&self.test_dir)
    }

    fn settings(&self) -> insta::Settings {
        self.settings.clone()
    }
}

#[test]
fn pass() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let ctx = Context {
        root,
        test_dir: PathBuf::new().join("tests").join("cases").join("pass"),
        source_name: "case.fuse",
        settings: insta::Settings::clone_current(),
    };

    // TODO: better way for pointing to the test cases directory.
    let cases = load_cases(&ctx);

    for case in cases {
        run(&ctx, case, false, false);
    }
}

// helpers
fn load_cases(ctx: &Context) -> Vec<PathBuf> {
    fs::read_dir(ctx.path())
        .unwrap()
        .map(|x| x.unwrap())
        .filter(|x| x.metadata().is_ok_and(|meta| meta.is_dir()))
        .map(|node| node.path())
        .collect()
}

fn run(ctx: &Context, case_dir: PathBuf, expect_error: bool, expect_panic: bool) {
    let mut settings = ctx.settings();
    let source_path = case_dir.join(ctx.source_name);

    settings.set_input_file(&source_path);
    settings.set_snapshot_path(case_dir);
    settings.set_prepend_module_to_snapshot(false);

    let _guard = settings.bind_to_scope();
    test_lexer(source_path.as_os_str());
    test_parser(source_path.as_os_str(), expect_error, expect_panic);
}

fn test_lexer(src_path: &OsStr) {
    let source = fs::read_to_string(src_path).unwrap();
    let reference = source.clone();
    let tokens: Vec<_> = Lexer::new(&source).collect();

    assert_eq!(
        source, reference,
        "Lexer changes the content of the original source buffer."
    );

    insta::assert_ron_snapshot!("tokens", tokens);
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

    insta::assert_ron_snapshot!("ast", chunk);
}
