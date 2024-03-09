use fuse_parser::Parser;

fn compile_chunk(source: &str) {
    let parsed_tree = Parser::new(source).parse();
    todo!("Compiler isn't done yet!")
}
