use std::collections::HashMap;

use fuse_ast::{
    Atom, BinaryOperator, BinaryOperatorKind, BindingPatternKind, CallExpression, Chunk,
    Expression, Function, Identifier, VariableDeclaration,
};
use fuse_common::ReferenceType;
use fuse_visitor::{
    walk_binary_operator_mut, walk_call_expression_mut, walk_function_mut,
    walk_variable_declaration_mut, ScopeVisitor, VisitorMut,
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

struct IdentifierMap(HashMap<Atom, ReferenceType>);

impl IdentifierMap {
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
    fn identifier_reference(&mut self, atom: &Atom) -> Option<ReferenceType> {
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

    /// Set a `ReferenceType` for the given identifier's `Atom` in the current scope.
    /// Would return the last `ReferenceType` if we are shadowing it.
    fn set_identifier_reference(
        &mut self,
        atom: Atom,
        ref_id: ReferenceType,
    ) -> Option<ReferenceType> {
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

pub struct Module {
    scope: ScopeTree,
    last_reference: ReferenceType,
}

impl Module {
    fn new() -> Self {
        Self {
            scope: ScopeTree::root_scope(),
            last_reference: 0,
        }
    }

    fn declare_identifier(&mut self, ident: &Identifier) {
        self.last_reference += 1;
        self.scope
            .set_identifier_reference(ident.name.clone(), self.last_reference);
        ident.reference.set(Some(self.last_reference))
    }

    fn reference_identifier(&mut self, ident: &Identifier) {
        let reference = self.scope.identifier_reference(&ident.name);
        ident.reference.set(reference)
    }
}

pub struct Resolver<'ast> {
    source: &'ast str,
    modules: HashMap<&'ast str, Module>,
}

impl<'ast> Resolver<'ast> {
    pub fn new(source: &'ast str) -> Self {
        Self {
            source,
            modules: HashMap::new(),
        }
    }

    pub fn resolve(&mut self, chunk: &'ast mut Chunk) -> ResolverResult {
        ResolverResult {
            errors: Vec::default(),
        }
    }
}

pub struct ResolverResult {
    pub errors: Vec<ResolverError>,
}

pub struct ResolverError {}
