use std::ops::{Index, Range};

/// Describes a byte offset into a piece of source code.
///
/// A source location is 32 bits in size, effectively limiting a singular
/// Serqlane source file to a size of 4GiB. This seems like a reasonable
/// limitation for a compiler and rewards us with less memory overhead.
///
/// Note that [`SourceLocation`]s are not attributed to the source string
/// they reference, which makes it the user's responsibility to track
/// these logical relations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceLocation(u32);

impl SourceLocation {
    /// Constructs a new [`SourceLocation`] from a byte offset.
    pub const fn new(pos: u32) -> Self {
        Self(pos)
    }

    /// Converts the [`SourceLocation`] to 1-based line and column.
    ///
    /// This information is only used in error messages, so we choose
    /// to compute it lazily only when we actually need it.
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

            pos += c.len_utf8() as u32;
        }

        (line, column)
    }
}

/// Spans some bytes in a piece of source code.
///
/// This is used to attribute tokens with their textual form. It has
/// less memory overhead than a substring, which makes it convenient
/// to store and pass around.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceSpan {
    start: SourceLocation,
    end: SourceLocation,
}

impl SourceSpan {
    /// Creates a new [`SourceSpan`] from two byte offsets.
    pub const fn new(start: u32, end: u32) -> Self {
        debug_assert!(start <= end);

        Self {
            start: SourceLocation::new(start),
            end: SourceLocation::new(end),
        }
    }

    /// Gets the start of the span as a [`SourceLocation`] (inclusive).
    pub fn start(self) -> SourceLocation {
        self.start
    }

    /// Gets the end of the span as a [`SourceLocation`] (exclusive).
    pub fn end(self) -> SourceLocation {
        self.end
    }

    /// Gets the byte length of this source span.
    pub fn len(self) -> u32 {
        self.end.0 - self.start.0
    }

    /// Attempts to extract the spanned substring from `input`.
    ///
    /// This may return [`None`] if the span is out of bounds for the
    /// given string, or the span does not describe valid UTF-8 text.
    pub fn text(self, input: &str) -> Option<&str> {
        let range: Range<usize> = self.into();
        input.get(range)
    }
}

impl From<SourceSpan> for Range<u32> {
    fn from(value: SourceSpan) -> Self {
        value.start.0..value.end.0
    }
}

impl From<SourceSpan> for Range<usize> {
    fn from(value: SourceSpan) -> Self {
        value.start.0 as usize..value.end.0 as usize
    }
}

impl From<Range<u32>> for SourceSpan {
    fn from(value: Range<u32>) -> Self {
        Self::new(value.start, value.end)
    }
}

impl Index<SourceSpan> for str {
    type Output = str;

    fn index(&self, index: SourceSpan) -> &Self::Output {
        let range: Range<usize> = index.into();
        &self[range]
    }
}
