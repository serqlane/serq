use std::fs;

mod ast;

mod diagnostic;

mod lexer;

mod parser;
use parser::*;

fn main() {
    let x = fs::read_to_string("x.serq").expect("Cannot find 'x.serq'");
    let mut parser = Parser::new(&x);
    println!("{:?}", parser.parse());
}
