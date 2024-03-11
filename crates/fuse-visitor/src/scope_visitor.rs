pub trait ScopeVisitor {
    fn enter_scope(&mut self) {}
    fn leave_scope(&mut self) {}
}
