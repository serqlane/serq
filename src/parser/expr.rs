use super::Parser;
use crate::{
    ast::expr::{Expression, Literal},
    lexer::TokenKind,
};

// The following implements a simple Pratt parsing system.
//
// We handle precedence by assigning binding power to every operator:
// expr:   A + B * C
// power:    1   2
//
// Thus, the expression gets folded as `A + (B * C)` since * has higher
// binding power than +.
//
// To handle associativity on top, we give operators slightly asymmetric
// binding powers on the left and right side:
// expr:   A   +   B   +   C
// power:    1   2   1   2
//
// The expression gets folded as `(A + B) + C` since the first + binds
// the B more tightly than the second one.
//
// Prefix and suffix operators only have right or left binding power,
// respectively. See the respective `*_binding_power` functions, which
// encode this information at the type level for easy referencing.

// -, !, ~, *, &
fn prefix_binding_power(op: TokenKind) -> ((), u8) {
    use TokenKind::*;
    match op {
        Minus | Bang | Tilde | Star | And => ((), 23),
        _ => unreachable!(),
    }
}

// [, (
fn postfix_binding_power(op: TokenKind) -> Option<(u8, ())> {
    use TokenKind::*;
    match op {
        LeftBracket | LeftParen => Some((25, ())),
        _ => None,
    }
}

// +, -, *, /, %, <<, >>, <, <=, >, >=, ==, !=, &, |, ^, &&, ||, =, +=, *=, -=, /=, %=, <<=, >>=, &=, |=, ^=, .
fn infix_binding_power(op: TokenKind) -> Option<(u8, u8)> {
    use TokenKind::*;
    match op {
        Star | Slash | Percent => Some((19, 20)),
        Plus | Minus => Some((17, 18)),
        Shl | Shr => Some((15, 16)),
        And => Some((13, 14)),
        Caret => Some((11, 12)),
        Or => Some((9, 10)),
        EqEq | BangEq | Lt | LtEq | Gt | GtEq => Some((7, 8)),
        AndAnd => Some((5, 6)),
        OrOr => Some((3, 4)),
        Eq | PlusEq | MinusEq | StarEq | SlashEq | PercentEq | AndEq | OrEq | CaretEq | ShlEq
        | ShrEq => Some((2, 1)),
        _ => None,
    }
}

impl<'src> Parser<'src> {
    pub fn expression(&mut self) -> Expression {
        self.expression_(0)
    }

    fn expression_(&mut self, mbp: u8) -> Expression {
        use TokenKind::*;

        let token = self.next().unwrap();
        let text = self.text(token.span());

        let mut lhs = match token.kind() {
            // TODO: Identifiers, strings, ...
            Number => parse_number(text),
            b @ (True | False) => Expression::Literal(Literal::Bool(b == True)),
            LeftParen => {
                let expr = self.expression_(0);
                self.expect(TokenKind::RightParen);
                expr
            }
            op @ (Minus | Bang | Tilde | Star | And) => {
                let ((), rbp) = prefix_binding_power(op);
                let rhs = self.expression_(rbp);
                Expression::prefix_operator(op, rhs)
            }
            _ => unimplemented!(),
        };

        loop {
            let op = match self.peek() {
                op @ (Plus | Minus | Star | Slash | Percent | Shl | Shr | And | Or | Caret
                | EqEq | BangEq | Lt | LtEq | Gt | GtEq | AndAnd | OrOr | Eq | PlusEq
                | MinusEq | StarEq | SlashEq | PercentEq | ShlEq | ShrEq | AndEq | OrEq
                | CaretEq | LeftBracket | LeftParen | RightParen | RightBracket) => op,
                TokenKind::Semicolon => break,
                op => panic!("{op:?}"), // Syntax error.
            };

            if let Some((lbp, ())) = postfix_binding_power(op) {
                if lbp < mbp {
                    break;
                }
                self.next();

                if op == LeftParen {
                    let mut params = Vec::new();
                    while !matches!(self.peek(), RightParen | Eof) {
                        let param = self.expression_(0);
                        if self.peek() != RightParen {
                            self.expect(Comma);
                        }
                        params.push(param);
                    }
                    self.expect(RightParen);
                    lhs = Expression::call(lhs, params);
                } else if op == LeftBracket {
                    let rhs = self.expression_(0);
                    self.expect(RightBracket);
                    lhs = Expression::index(lhs, rhs);
                }

                continue;
            }

            if let Some((lbp, rbp)) = infix_binding_power(op) {
                if lbp < mbp {
                    break;
                }

                self.next();
                let rhs = self.expression_(rbp);

                lhs = Expression::infix_operator(lhs, op, rhs);
                continue;
            }

            break;
        }

        lhs
    }
}

fn parse_number(src: &str) -> Expression {
    // TODO: Handle more number formats and errors.
    src.parse::<u64>()
        .map(Literal::Integer)
        .map(Expression::Literal)
        .unwrap()
}
