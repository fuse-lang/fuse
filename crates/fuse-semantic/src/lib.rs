use std::collections::HashMap;

use fuse_ast::{
    walk_binding_pattern, walk_function, walk_variable_declaration, Atom, BindingPattern,
    BindingPatternKind, Chunk, Identifier, VariableDeclaration, Visitor,
};
use fuse_common::ReferenceId;

#[derive(Debug, PartialEq, Clone, Copy)]
struct ScopeId(ReferenceId);

impl ScopeId {
    #[inline(always)]
    const fn as_index(self) -> ReferenceId {
        self.0
    }

    #[inline(always)]
    const fn is_root(&self) -> bool {
        self.0 == 0
    }
}

impl PartialEq<ReferenceId> for ScopeId {
    fn eq(&self, other: &ReferenceId) -> bool {
        self.0 == *other
    }
}

struct IdentifierMap(HashMap<Atom, ReferenceId>);

impl IdentifierMap {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn insert(&mut self, atom: Atom, ref_id: ReferenceId) -> Option<ReferenceId> {
        self.0.insert(atom, ref_id)
    }

    fn get(&self, atom: &Atom) -> Option<ReferenceId> {
        self.0.get(atom).map(|r| r.clone())
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
            current: ScopeId(0),
            parent_ids: vec![ScopeId(0)],
            identifier_maps: vec![IdentifierMap::new()],
        }
    }

    fn push_stack(&mut self) -> ScopeId {
        self.identifier_maps.push(IdentifierMap::new());
        self.parent_ids.push(self.current);

        // length of all arrays should be same.
        debug_assert!(self.identifier_maps.len() == self.parent_ids.len());

        self.current = ScopeId(self.identifier_maps.len() - 1);
        self.current
    }

    fn pop_stack(&mut self) {
        assert_ne!(
            self.current, 0,
            "Attempt to pop the root scope from the stack."
        );

        self.current = self.parent();
    }

    /// Get an identifier reference from current scope or its parents.
    /// This function is implemented using loops instead of recursion.
    fn identifier_reference(&mut self, atom: &Atom) -> Option<ReferenceId> {
        let mut scope_id = self.current;
        let mut reference;
        loop {
            reference = self.identifier_maps[scope_id.as_index()].get(atom);

            if reference.is_some() || scope_id.is_root() {
                break;
            } else {
                scope_id = self.parent_of(scope_id);
            }
        }
        reference
    }

    /// Set a `ReferenceId` for the given identifier's `Atom` in the current scope.
    /// Would return the last `ReferenceId` if we are shadowing it.
    fn set_identifier_reference(&mut self, atom: Atom, ref_id: ReferenceId) -> Option<ReferenceId> {
        self.identifier_maps[self.current.as_index()].insert(atom, ref_id)
    }

    fn parent(&self) -> ScopeId {
        assert_ne!(
            self.current, 0,
            "Attempt to access the root scope's parent."
        );
        self.parent_ids[self.current.as_index()]
    }

    fn parent_of(&self, children_id: ScopeId) -> ScopeId {
        assert_ne!(
            self.current, 0,
            "Attempt to access the root scope's parent."
        );
        self.parent_ids[children_id.as_index()]
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

    fn declare_identifier(&mut self, ident: &Identifier) {
        self.last_reference += 1;
        self.scope
            .set_identifier_reference(ident.name.clone(), self.last_reference);
        ident.reference.set(Some(self.last_reference))
    }

    fn reference_identifier(&mut self, ident: &Identifier) {
        let reference = self
            .scope
            .identifier_reference(&ident.name)
            .expect("Reference to undefined identifier.");
        ident.reference.set(Some(reference))
    }
}

impl<'ast> Visitor<'ast> for Semantic<'ast> {
    fn enter_scope(&mut self) {
        self.scope.push_stack();
    }

    fn leave_scope(&mut self) {
        self.scope.pop_stack();
    }

    fn visit_identifier(&mut self, ident: &Identifier) {
        self.reference_identifier(ident);
    }

    fn visit_variable_declaration(&mut self, decl: &'ast VariableDeclaration) {
        match &decl.binding.kind {
            BindingPatternKind::Identifier(bind) => self.declare_identifier(&bind.identifier),
            _ => todo!(),
        }

        walk_variable_declaration(self, decl)
    }

    fn visit_function_declaration(&mut self, decl: &'ast fuse_ast::Function) {
        let identifier = decl
            .signature
            .identifier
            .as_ref()
            .expect("All function declarations need an identifier.");
        self.declare_identifier(identifier);
        walk_function(self, decl)
    }
}
