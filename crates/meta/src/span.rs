use miette::SourceSpan;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    pub start: usize,
    pub end: usize
}

impl From<Span> for SourceSpan {
    fn from(s: Span) -> Self {
        Self::new(s.start.into(), (s.end - s.start).into())
    }
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }

    pub fn combine(spans: &[Span]) -> Self {
        let length = spans.len();

        if length == 0 {
            Span { start: 0, end: 0 }
        } else if length == 1 {
            spans[0]
        } else {
            Self {
                start: spans[0].start,
                end: spans[length - 1].end
            }
        }
    }

    pub fn unknown() -> Self {
        Self::new(0, 0)
    }

    pub fn offset(&self, offset: usize) -> Self {
        Span {
            start: self.start - offset,
            end: self.end - offset
        }
    }

    pub fn contains(&self, pos: usize) -> bool {
        pos >= self.start && pos < self.end
    }

    pub fn past(&self) -> Span {
        Span {
            start: self.end,
            end: self.end
        }
    }
}
