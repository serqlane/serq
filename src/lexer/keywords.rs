use super::TokenKind;

/// The maximum length of a Serqlane keyword in bytes.
pub const MAX_KEYWORD_LEN: usize = 8;

const fn kw(input: &str) -> [u8; MAX_KEYWORD_LEN] {
    let input = input.as_bytes();
    let mut res = [0; MAX_KEYWORD_LEN];
    debug_assert!(input.len() <= MAX_KEYWORD_LEN);

    let mut i = 0;
    while i < input.len() {
        res[i] = input[i];
        i += 1;
    }

    res
}

/// Checks if `buf` holds a valid keyword and returns its [`TokenKind`].
///
/// Otherwise returns [`TokenKind::Identifier`].
pub fn check_keyword(buf: [u8; MAX_KEYWORD_LEN]) -> TokenKind {
    use TokenKind::*;

    #[inline]
    pub fn match_kw(
        buf: [u8; MAX_KEYWORD_LEN],
        pred: [u8; MAX_KEYWORD_LEN],
        token: TokenKind,
    ) -> TokenKind {
        if buf == pred { token } else { Identifier }
    }

    // This is a small hand-crafted trie that dispatches on the first
    // character of a potential keyword (buf is always non-empty) and
    // then goes from there. Through exploiting the fact that keywords
    // fit into registers, we get string comparison with very efficient
    // cmp instructions.
    match buf[0] {
        b'b' => match_kw(buf, const { kw("break") }, Break),
        b'c' if buf[3] == b's' => match_kw(buf, const { kw("const") }, Const),
        b'c' if buf[3] == b't' => match_kw(buf, const { kw("continue") }, Continue),
        b'e' if buf[1] == b'l' => match_kw(buf, const { kw("else") }, Else),
        b'e' if buf[1] == b'n' => match_kw(buf, const { kw("enum") }, Enum),
        b'f' if buf[1] == b'a' => match_kw(buf, const { kw("false") }, False),
        b'f' if buf[1] == b'n' => Fn,
        b'f' if buf[1] == b'o' => match_kw(buf, const { kw("for") }, For),
        b'i' => match_kw(buf, const { kw("if") }, If),
        b'l' => match_kw(buf, const { kw("let") }, Let),
        b'm' => match_kw(buf, const { kw("mut") }, Mut),
        b'p' => match_kw(buf, const { kw("pub") }, Pub),
        b'r' => match_kw(buf, const { kw("return") }, Return),
        b't' => match_kw(buf, const { kw("true") }, True),
        b'w' => match_kw(buf, const { kw("while") }, While),
        _ => Identifier,
    }
}
