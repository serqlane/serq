mod lexer;
use std::io::{self, BufRead, Write};

use lexer::*;

fn main() {
    let mut buf = String::new();
    let mut stdout = io::stdout().lock();
    let mut stdin = io::stdin().lock();
    loop {
        stdout.write_all(b"> ").unwrap();
        stdout.flush().unwrap();
        stdin.read_line(&mut buf).unwrap();

        let scanner = Scanner::new(&buf);
        for (token, span) in scanner {
            let (line, col) = span.start().as_line_and_column(&buf);
            println!(
                "{line}:{col}: {token:?} (\"{}\")",
                span.resolve(&buf).unwrap()
            );
        }

        buf.clear();
    }
}
