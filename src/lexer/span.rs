#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourcePosition(usize);

impl SourcePosition {
    pub fn line(&self, input: &str) -> usize {
        let mut pos = 0;
        1 + input
            .split('\n')
            .take_while(|l| {
                pos += l.len();
                pos <= self.0
            })
            .count()
    }

    pub fn column(&self, input: &str) -> usize {
        let (input, _) = input.split_at(self.0);
        input
            .rfind('\n')
            .map(|pos| self.0 - pos - 1)
            .unwrap_or(self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceSpan {
    start: SourcePosition,
    end: SourcePosition,
}

impl SourceSpan {
    #[inline]
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start: SourcePosition(start),
            end: SourcePosition(end),
        }
    }

    #[inline]
    pub fn start(&self) -> SourcePosition {
        self.start
    }

    #[inline]
    pub fn end(&self) -> SourcePosition {
        self.end
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.end.0 - self.start.0
    }

    #[inline]
    pub fn resolve<'a>(&self, input: &'a str) -> Option<&'a str> {
        input.get(self.start.0..self.end.0)
    }
}
