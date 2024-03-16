use std::collections::HashMap;

use fuse_ast::{
    Atom, BindingPatternKind, Chunk, Function, Identifier, MemberExpression, MemberExpressionLHS,
    MemberExpressionRHS, VariableDeclaration,
};
use fuse_common::ReferenceType;
use fuse_visitor::{
    walk_function_mut, walk_member_expression_mut, walk_variable_declaration_mut, ScopeVisitor,
    VisitorMut,
};

#[derive(Debug, PartialEq, Clone, Copy)]
struct ScopeId(ReferenceType);

impl ScopeId {
    #[inline(always)]
    const fn as_index(self) -> ReferenceType {
        self.0
    }

    #[inline(always)]
    const fn is_root(&self) -> bool {
        self.0 == 0
    }
}

impl PartialEq<ReferenceType> for ScopeId {
    fn eq(&self, other: &ReferenceType) -> bool {
        self.0 == *other
    }
}

struct IdentDeclMap(HashMap<Atom, ReferenceType>);

impl IdentDeclMap {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn insert(&mut self, atom: Atom, ref_id: ReferenceType) -> Option<ReferenceType> {
        self.0.insert(atom, ref_id)
    }

    fn get(&self, atom: &Atom) -> Option<ReferenceType> {
        self.0.get(atom).map(|r| r.clone())
    }
}

enum ReferenceKind {
    Scope,
    Member,
}

struct ScopeTree {
    current: ScopeId,
    ident_decl_maps: Vec<IdentDeclMap>,
    /// Maps between `ReferenceId` and `ReferenceKind`
    reference_kinds: Vec<ReferenceKind>,
    parent_ids: Vec<ScopeId>,
}

/// Tree operations for `ScopeTree`
impl ScopeTree {
    fn root_scope() -> Self {
        Self {
            current: ScopeId(0),
            parent_ids: vec![ScopeId(0)],
            ident_decl_maps: vec![IdentDeclMap::new()],
            reference_kinds: Vec::new(),
        }
    }

    fn push_stack(&mut self) -> ScopeId {
        self.ident_decl_maps.push(IdentDeclMap::new());
        self.parent_ids.push(self.current);

        // length of all arrays should be same.
        debug_assert!(self.ident_decl_maps.len() == self.parent_ids.len());

        self.current = ScopeId(self.ident_decl_maps.len() - 1);
        self.current
    }

    fn pop_stack(&mut self) {
        assert_ne!(
            self.current, 0,
            "Attempt to pop the root scope from the stack."
        );

        self.current = self.parent();
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

/// Identifier operations for `ScopeTree`
impl ScopeTree {
    /// Get an identifier reference from current scope or its parents.
    /// This function is implemented using loops instead of recursion.
    fn scope_identifier_reference(&mut self, atom: &Atom) -> Option<ReferenceType> {
        let mut scope_id = self.current;
        let mut reference;
        loop {
            reference = self.ident_decl_maps[scope_id.as_index()].get(atom);

            if reference.is_some() || scope_id.is_root() {
                break;
            } else {
                scope_id = self.parent_of(scope_id);
            }
        }
        reference
    }

    /// Set a `ReferenceType` for the given identifier's `Atom` in the current scope.
    /// Would return the last `ReferenceType` if we are shadowing it.
    fn set_scope_identifier_reference(
        &mut self,
        atom: Atom,
        ref_id: ReferenceType,
    ) -> Option<ReferenceType> {
        self.ident_decl_maps[self.current.as_index()].insert(atom, ref_id)
    }

    fn member_identifier_reference(
        &mut self,
        atom: Atom,
        ref_id: ReferenceType,
    ) -> Option<ReferenceType> {
        self.ident_decl_maps[self.current.as_index()].insert(atom, ref_id)
    }
}

pub struct Semantic<'ast> {
    source: &'ast str,
    scope: ScopeTree,
    last_reference: ReferenceType,
}

impl<'ast> Semantic<'ast> {
    pub fn new(source: &'ast str) -> Self {
        Self {
            source,
            scope: ScopeTree::root_scope(),
            last_reference: 0,
        }
    }

    pub fn build(&mut self, chunk: &'ast mut Chunk) -> SemanticResult {
        self.visit_chunk_mut(chunk);
        SemanticResult {
            errors: Vec::default(),
        }
    }

    fn declare_identifier(&mut self, ident: &Identifier) {
        self.last_reference += 1;
        self.scope
            .set_scope_identifier_reference(ident.name.clone(), self.last_reference);
        ident.reference.set(Some(self.last_reference))
    }

    fn reference_scope_identifier(&mut self, ident: &Identifier) {
        let reference = self.scope.scope_identifier_reference(&ident.name);
        ident.reference.set(reference)
    }

    fn resolve_member_identifier(&mut self, ident: &Identifier, sup: Option<&Identifier>) {
        let Some(sup) = sup else {
            return self.declare_member_identifier(ident, None);
        };
        let reference = self.scope.scope_identifier_reference(&ident.name);
        ident.reference.set(reference);
        // todo!()
    }

    fn declare_member_identifier(&mut self, ident: &Identifier, sup: Option<&Identifier>) {
        let reference = self.scope.scope_identifier_reference(&ident.name);
        ident.reference.set(reference);
        // todo!()
    }
}

impl<'ast> VisitorMut<'ast> for Semantic<'ast> {
    fn visit_identifier_mut(&mut self, ident: &'ast mut Identifier) {
        if ident.reference.get_mut().is_none() {
            self.reference_scope_identifier(ident);
        }
    }

    fn visit_variable_declaration_mut(&mut self, decl: &'ast mut VariableDeclaration) {
        match &decl.binding.kind {
            BindingPatternKind::Identifier(bind) => self.declare_identifier(&bind.identifier),
            _ => todo!(),
        }

        walk_variable_declaration_mut(self, decl)
    }

    fn visit_function_declaration_mut(&mut self, decl: &'ast mut Function) {
        let identifier = decl
            .signature
            .identifier
            .as_ref()
            .expect("All function declarations need an identifier.");
        self.declare_identifier(identifier);
        walk_function_mut(self, decl)
    }

    fn visit_member_expression_mut(&mut self, member: &'ast mut MemberExpression) {
        let lhs = member.lhs.as_ref();
        let rhs = member.rhs.as_ref();
        let sup = match lhs {
            MemberExpressionLHS::Identifier(ident) => {
                self.reference_scope_identifier(ident);
                Some(ident)
            }
            _ => None,
        };
        if let MemberExpressionRHS::Identifier(ident) = rhs {
            self.resolve_member_identifier(ident, sup)
        }
        walk_member_expression_mut(self, member)
    }
}

impl<'ast> ScopeVisitor for Semantic<'ast> {
    fn enter_scope(&mut self) {
        self.scope.push_stack();
    }

    fn leave_scope(&mut self) {
        self.scope.pop_stack();
    }
}

pub struct SemanticResult {
    pub errors: Vec<SemanticError>,
}

pub struct SemanticError {}
