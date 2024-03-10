// based on https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html

use crate::ast::*;

macro_rules! visit_list {
    ($visitor:ident, $method:ident, $list:expr) => {
        todo!()
    };
}

pub trait Visitor<'ast>: Sized {
    fn visit_chunk(&mut self, chunk: &'ast Chunk) {
        walk_block(self, &chunk.body)
    }

    fn visit_statement(&mut self, statement: &'ast Statement) {
        walk_statement(self, &statement)
    }
}

pub fn walk_block<'ast, V: Visitor<'ast>>(visitor: &mut V, block: &'ast Block) {
    visit_list!(visitor, visit_statement, block.statements)
}

pub fn walk_statement<'ast, V: Visitor<'ast>>(visitor: &mut V, statement: &'ast Statement) {
    todo!()
}
