use miette::SourceSpan;

use crate::SourceId;

#[derive(Clone, Debug, PartialEq)]
pub struct Spanned<T>
where
    T: Clone + std::fmt::Debug,
{
    pub item: T,
    pub span: Span,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    pub source_id: SourceId,
    pub start: usize,
    pub end: usize,
}

impl From<Span> for SourceSpan {
    fn from(s: Span) -> Self {
        Self::new(s.start.into(), (s.end - s.start).into())
    }
}

impl Span {
    pub fn new(source_id: SourceId, start: usize, end: usize) -> Self {
        Span {
            source_id,
            start,
            end,
        }
    }

    pub fn garbage() -> Self {
        Self {
            source_id: 0,
            start: 0,
            end: 0,
        }
    }

    pub fn combine(spans: &[Span]) -> Self {
        let length = spans.len();

        if length == 0 {
            Span::garbage()
        } else if length == 1 {
            spans[0]
        } else {
            Self {
                source_id: spans[0].source_id,
                start: spans[0].start,
                end: spans[length - 1].end,
            }
        }
    }

    pub fn offset(&self, offset: usize) -> Self {
        Span {
            source_id: self.source_id,
            start: self.start - offset,
            end: self.end - offset,
        }
    }

    pub fn contains(&self, pos: usize) -> bool {
        pos >= self.start && pos < self.end
    }

    pub fn past(&self) -> Span {
        Span {
            source_id: self.source_id,
            start: self.end,
            end: self.end,
        }
    }
}
