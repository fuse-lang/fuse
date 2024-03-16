use fuse_ast::AstNode;

pub trait NodeVisitor {
    fn enter_node(&mut self, node: AstNode) {}
    fn leave_node(&mut self, node: AstNode) {}
}
