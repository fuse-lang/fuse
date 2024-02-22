use crate::Span;

pub trait SpanView {
    fn view(self, span: &Span) -> Self;
}

impl<'a> SpanView for &'a str {
    #[inline]
    fn view(self, span: &Span) -> Self {
        &self[span.start as usize..span.end as usize]
    }
}
