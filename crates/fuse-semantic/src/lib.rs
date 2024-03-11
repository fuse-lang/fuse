use std::collections::HashMap;

use fuse_ast::{
    walk_binding_pattern, walk_variable_declaration, Atom, BindingPattern, BindingPatternKind,
    Chunk, Identifier, VariableDeclaration, Visitor,
};
use fuse_common::ReferenceId;

type ScopeId = ReferenceId;

struct IdentifierMap(HashMap<Atom, ReferenceId>);

impl IdentifierMap {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn insert(&mut self, atom: Atom, ref_id: ReferenceId) {
        debug_assert!(!self.0.contains_key(&atom));
        self.0.insert(atom, ref_id);
    }
}

struct ScopeTree {
    current: ScopeId,
    identifier_maps: Vec<IdentifierMap>,
    parent_ids: Vec<ScopeId>,
}

impl ScopeTree {
    fn root_scope() -> Self {
        Self {
            current: 0,
            parent_ids: vec![0],
            identifier_maps: vec![IdentifierMap::new()],
        }
    }

    fn push_stack(&mut self) -> ScopeId {
        self.identifier_maps.push(IdentifierMap::new());
        self.parent_ids.push(self.current);

        let scope_id = self.identifier_maps.len() - 1;

        // length of all arrays should be same.
        debug_assert!(self.identifier_maps.len() == self.parent_ids.len());
        self.current = scope_id;
        scope_id
    }

    fn pop_stack(&mut self) {
        assert_ne!(
            self.current, 0,
            "Attempt to pop the root scope from the stack."
        );

        self.current = self.parent();
    }

    fn push_identifier(&mut self, atom: Atom, ref_id: ReferenceId) {
        self.identifier_maps[self.current].insert(atom, ref_id);
    }

    fn parent(&self) -> ScopeId {
        assert_ne!(
            self.current, 0,
            "Attempt to access the root scope's parent."
        );
        self.parent_ids[self.current]
    }
}

pub struct Semantic<'ast> {
    source: &'ast str,
    chunk: &'ast Chunk,
    scope: ScopeTree,
    last_reference: ReferenceId,
}

impl<'ast> Semantic<'ast> {
    pub fn new(source: &'ast str, chunk: &'ast Chunk) -> Self {
        Self {
            source,
            chunk,
            scope: ScopeTree::root_scope(),
            last_reference: 0,
        }
    }

    pub fn build(mut self) {
        self.visit_chunk(&self.chunk)
    }

    fn reference_identifier(&mut self, ident: &Identifier) {
        self.last_reference += 1;
        self.scope
            .push_identifier(ident.name.clone(), self.last_reference);
        ident.reference.set(Some(self.last_reference))
    }
}

impl<'ast> Visitor<'ast> for Semantic<'ast> {
    fn enter_scope(&mut self) {
        println!("IN");
        self.scope.push_stack();
    }

    fn leave_scope(&mut self) {
        println!("OUT");
        self.scope.pop_stack();
    }

    fn visit_identifier(&mut self, ident: &Identifier) {
        println!("{ident:?}")
    }

    fn visit_variable_declaration(&mut self, decl: &'ast VariableDeclaration) {
        match &decl.binding.kind {
            BindingPatternKind::Identifier(bind) => self.reference_identifier(&bind.identifier),
            _ => todo!(),
        }

        walk_variable_declaration(self, decl)
    }

    fn visit_binding_pattern(&mut self, pattern: &'ast BindingPattern) {
        match &pattern.kind {
            BindingPatternKind::Identifier(ident) => {
                println!("{ident:?}")
            }
            _ => {}
        }
        walk_binding_pattern(self, pattern)
    }
}
