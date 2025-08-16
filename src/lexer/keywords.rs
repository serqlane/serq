use super::Token;

pub const MAX_KEYWORD_LEN: usize = 8;

// This might look like a weird way of storing keywords, but
// it enables us to do string comparisons with a single cmp
// instruction thanks to keywords being no longer than 8 bytes.
static KEYWORDS: [([u8; 8], Token); 15] = [
    (*b"break\0\0\0", Token::Break),
    (*b"const\0\0\0", Token::Const),
    (*b"continue", Token::Continue),
    (*b"else\0\0\0\0", Token::Else),
    (*b"enum\0\0\0\0", Token::Enum),
    (*b"false\0\0\0", Token::False),
    (*b"for\0\0\0\0\0", Token::For),
    (*b"fn\0\0\0\0\0\0", Token::Fn),
    (*b"if\0\0\0\0\0\0", Token::If),
    (*b"let\0\0\0\0\0", Token::Let),
    (*b"mut\0\0\0\0\0", Token::Mut),
    (*b"pub\0\0\0\0\0", Token::Pub),
    (*b"return\0\0", Token::Return),
    (*b"true\0\0\0\0", Token::True),
    (*b"while\0\0\0", Token::While),
];

fn keyword_hash(key: &[u8]) -> usize {
    // A perfect hash function for keyword lookup.
    //
    // Make sure the order of elements in the lookup table
    // is the same as the order in keywords.txt.
    //
    // To generate, run the following in the project root:
    // ./scripts/perfect_hash.py scripts/keywords.txt scripts/keywords.tmpl.rs -o std

    static G: &[u8] = &[
        0, 2, 0, 31, 0, 0, 19, 0, 0, 0, 26, 0, 0, 19, 0, 9, 0, 0, 3, 10, 0, 0, 25, 0, 14, 0, 7, 0,
        0, 17, 9, 6,
    ];
    const S1: [u8; 8] = *b"kpOWHt9r";
    const S2: [u8; 8] = *b"Two9MDXf";

    fn hash(key: &[u8], salt: [u8; 8]) -> usize {
        key.iter()
            .enumerate()
            .map(|(i, &b)| salt[i % 8] as usize * b as usize)
            .sum::<usize>()
            % 32
    }

    ((G[hash(key, S1)] + G[hash(key, S2)]) % 32) as usize
}

pub fn is_keyword(key: [u8; 8], cursor: usize) -> Option<Token> {
    let hash = keyword_hash(&key[..(cursor & 7)]);
    if hash < KEYWORDS.len() {
        let (kw, token) = KEYWORDS[hash];
        if key == kw {
            return Some(token);
        }
    }

    None
}
