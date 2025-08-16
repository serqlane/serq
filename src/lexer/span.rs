#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourcePosition(usize);

impl SourcePosition {
    pub fn as_line_and_column(self, input: &str) -> (u32, u32) {
        let mut line = 1;
        let mut column = 1;
        let mut pos = 0;
        for c in input.chars() {
            if pos >= self.0 {
                break;
            }

            if c == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }

            pos += c.len_utf8();
        }

        (line, column)
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
