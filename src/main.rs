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
            let start = span.start();
            println!(
                "{}:{} {token:?} (\"{}\")",
                start.line(&buf),
                start.column(&buf),
                span.resolve(&buf).unwrap()
            );
        }

        buf.clear();
    }
}
