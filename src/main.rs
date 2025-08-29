use std::io::{self, BufRead, Write};

mod diagnostic;

mod lexer;
use lexer::*;

fn main() {
    let mut buf = String::new();
    let mut stdout = io::stdout().lock();
    let mut stdin = io::stdin().lock();
    loop {
        stdout.write_all(b"> ").unwrap();
        stdout.flush().unwrap();
        stdin.read_line(&mut buf).unwrap();

        let lexer = Lexer::new(&buf);
        for token in lexer {
            let (line, col) = token.span().start().as_line_and_column(&buf);
            println!(
                "{line}:{col}: {:?} (\"{}\")",
                token.kind(),
                &buf.as_str()[token.span()]
            );
        }

        buf.clear();
    }
}
