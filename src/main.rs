use std::io::{self, BufRead, Write};

mod ast;

mod diagnostic;

mod lexer;
use lexer::*;

mod parser;
use parser::*;

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

        let mut parser = Parser::new(&buf);
        println!("{:?}", parser.expression());

        buf.clear();
    }
}
