mod node_visitor;
mod scope_visitor;
mod visitor;
mod visitor_mut;

pub use node_visitor::*;
pub use scope_visitor::*;
pub use visitor::*;
pub use visitor_mut::*;

/// Placeholder macro for visiting, It is used instead of calling visitor methods directly.
/// Would be useful if we need some operation to happen for every visit.
#[macro_export]
macro_rules! visit {
    ($expr:expr) => {
        $expr
    };
}

#[macro_export]
macro_rules! visit_list {
    ($visitor:ident.$method:ident($list:expr $(, $($extra_args:expr), *)?)) => {
        for elem in $list {
            visit!($visitor.$method(elem $(, $($extra_args),*)?))
        }
    };
}

#[macro_export]
macro_rules! visit_scope {
    ($visitor:ident => $block:expr) => {
        $visitor.enter_scope();
        $block;
        $visitor.leave_scope();
    };
}
