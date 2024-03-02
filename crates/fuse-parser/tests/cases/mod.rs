use std::{ffi::OsStr, fs, path::PathBuf};

use fuse_parser::{lexer::Lexer, parse};

macro_rules! expect_eq {
    (
        $lhs:expr,
        $rhs:expr,
        $format:literal,
        path: $path:expr,
        dump: $dump:ident,
        $($args:expr$(,)?)*
    ) => {
        assert_eq!($lhs, $rhs, "{} : {}\ndump: {:?}", format!($format, $($args,)*), $path, $dump);
    };
}

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

// #[test]
fn fail() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let ctx = Context {
        root,
        test_dir: PathBuf::new().join("tests").join("cases").join("fail"),
        source_name: "case.fuse",
        settings: insta::Settings::clone_current(),
    };

    // TODO: better way for pointing to the test cases directory.
    let cases = load_cases(&ctx);

    for case in cases {
        run(&ctx, case, true, false);
    }
}

#[test]
fn panic() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let ctx = Context {
        root,
        test_dir: PathBuf::new().join("tests").join("cases").join("panic"),
        source_name: "case.fuse",
        settings: insta::Settings::clone_current(),
    };

    // TODO: better way for pointing to the test cases directory.
    let cases = load_cases(&ctx);

    for case in cases {
        run(&ctx, case, true, true);
    }
}

// helpers
fn load_cases(ctx: &Context) -> Vec<PathBuf> {
    fs::read_dir(ctx.path())
        .expect(&format!("Failed to read {}", ctx.path().to_str().unwrap()))
        .filter_map(|x| x.ok())
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
    test_parser(source_path.as_os_str(), expect_error, expect_panic);
    test_lexer(source_path.as_os_str());
}

fn test_lexer(src_path: &OsStr) {
    let source = read_source_normalized(src_path).unwrap();
    let reference = source.clone();
    let tokens: Vec<_> = Lexer::new(&source).collect();

    expect_eq!(
        source, reference,
        "Lexer changes the content of the original source buffer.",
        path: src_path.to_str().unwrap_or("unknown source"),
        dump: tokens,
    );

    insta::assert_ron_snapshot!("tokens", tokens);
}

fn test_parser(path: &OsStr, expect_error: bool, expect_panic: bool) {
    let source = read_source_normalized(path).unwrap();
    let reference = source.clone();
    let parsed = parse(source.as_str());

    expect_eq!(
        source, reference,
        "Lexer changes the content of the original source buffer.",
        path: path.to_str().unwrap_or("unknown case!"),
        dump: parsed,
    );

    expect_eq!(
        expect_panic,
        parsed.paniced,
        "Panic state ({}) was different from the expected value ({}).",
        path: path.to_str().unwrap_or("unknown case!"),
        dump: parsed,
        parsed.paniced,
        expect_panic,
    );

    expect_eq!(
        expect_error,
        !parsed.errors.is_empty(),
        "Error vector is different from expectations.",
        path: path.to_str().unwrap_or("unknown case!"),
        dump: parsed,
    );

    if !expect_panic {
        insta::assert_ron_snapshot!("ast", parsed.chunk);
    }

    if expect_error {
        insta::assert_ron_snapshot!("errors", parsed.errors);
    }
}

fn read_source_normalized(path: &OsStr) -> Result<String, std::io::Error> {
    fs::read_to_string(path).map(|it| it.replace("\r\n", "\n"))
}
