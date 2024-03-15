use fuse_common::Span;

pub trait GetSpan {
    fn span(&self) -> Span;
}
