use std::cmp;

use fuse_common_proc::serializable;

#[serializable]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    #[inline]
    pub const fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    #[inline]
    pub fn with_spans(spans: Vec<Self>) -> Self {
        spans
            .into_iter()
            .reduce(|acc, e| Span::new(cmp::min(acc.start, e.start), cmp::max(acc.end, e.end)))
            .expect("Attempt to construct a `Span` using `with_spans` with an empty list of spans.")
    }
}
