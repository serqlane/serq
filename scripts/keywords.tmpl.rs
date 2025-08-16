fn keyword_hash(key: &[u8]) -> usize {
    // A perfect hash function for keyword lookup.
    //
    // Make sure the order of elements in the lookup table
    // is the same as the order in keywords.txt.
    //
    // To generate, run the following in the project root:
    // python scripts/perfect_hash.py scripts/keywords.txt scripts/keywords.tmpl.rs -o std

    static G: &[u8] = &[
        $G
    ];
    const S1: [u8; 8] = *b"$S1";
    const S2: [u8; 8] = *b"$S2";

    #[inline]
    fn hash(key: &[u8], salt: [u8; 8]) -> usize {
        key
            .iter()
            .enumerate()
            .map(|(i, &b)| salt[i % $NS] as usize * b as usize)
            .sum::<usize>()
            % $NG
    }

    ((G[hash(key, S1)] + G[hash(key, S2)]) % $NG) as usize
}
