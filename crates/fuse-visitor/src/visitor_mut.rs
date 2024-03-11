use crate::{visit, visit_list, visit_scope};
use fuse_ast::ast::*;

pub trait VisitorMut<'ast>: Sized {
    fn enter_scope(&mut self) {}

    fn leave_scope(&mut self) {}

    fn visit_chunk(&mut self, chunk: &'ast mut Chunk) {
        walk_block_mut(self, &mut chunk.body)
    }

    fn visit_block(&mut self, block: &'ast mut Block) {
        walk_block_mut(self, block)
    }

    fn visit_statement_mut(&mut self, statement: &'ast mut Statement) {
        walk_statement_mut(self, statement)
    }
}

pub fn walk_block_mut<'ast, V: VisitorMut<'ast>>(visitor: &mut V, block: &'ast mut Block) {
    visit_scope!(visitor => {
        visit_list!(visitor.visit_statement_mut(&mut block.statements));
    });
}

pub fn walk_statement_mut<'ast, V: VisitorMut<'ast>>(
    visitor: &mut V,
    statement: &'ast mut Statement,
) {
    match statement {
        Statement::Empty(_) => {}
        // Statement::Expression(expr) => visit!(visitor.visit_expression(expr)),
        // Statement::VariableDeclaration(decl) => visit!(visitor.visit_variable_declaration(decl)),
        // Statement::FunctionDeclaration(func) => visit!(visitor.visit_function_declaration(func)),
        // Statement::EnumDeclaration(decl) => visit!(visitor.visit_enum_declaration(decl)),
        // Statement::StructDeclaration(Box<StructDeclaration>),
        // Statement::ImplStatement(Box<ImplStatement>),
        _ => todo!(),
    }
}
