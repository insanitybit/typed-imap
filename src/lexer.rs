struct Lexer {
    ctrl_chars: Vec<u32>,
    all_chars: Vec<u32>,
    specials: Vec<u8>,
    non_specials: Vec<u8>,
    whitespace: Vec<u8>,
}

impl Lexer {
    fn new() -> () {
        let ctrl_chars: Vec<_> = (0..32).collect();
        let all_chars: Vec<_> = (0..256).collect();
        let specials: Vec<u8> = vec![b'(', b')', b'%', b'\'', b'[', b'"'];
        let non_specials = all_chars.iter().filter(|c| {
            !specials.contains(&(**c as u8)) && !ctrl_chars.contains(*c)
        });
        let whitespace: Vec<u8> = vec![b' ', b'\t', b'\r', b'\n'];
        let blackslash = '\\';
        let open_square = '[';
        let double_quote = '"';
    }
}


// // CTRL_CHARS = frozenset(c for c in range(32))
// // ALL_CHARS = frozenset(c for c in range(256))
// SPECIALS = frozenset(c for c in six.iterbytes(b' ()%"['))
// // NON_SPECIALS = ALL_CHARS - SPECIALS - CTRL_CHARS
// // WHITESPACE = frozenset(c for c in six.iterbytes(b' \t\r\n'))
// //
// // BACKSLASH = ord('\\')
// // OPEN_SQUARE = ord('[')
// // CLOSE_SQUARE = ord(']')
// // DOUBLE_QUOTE = ord('"')
