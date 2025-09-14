use super::Parser;
use crate::{
    ast::stmt::{Function, FunctionArg, Statement},
    lexer::TokenKind,
};

impl<'src> Parser<'src> {
    pub(super) fn statement(&mut self) -> Statement {
        if self.at(TokenKind::Let) || self.at(TokenKind::Mut) {
            let kw = self.next().unwrap();
            let ident = self.ident();
            self.eat(TokenKind::Eq);
            let expr = self.expression();
            Statement::Variable {
                ident,
                expr,
                mutable: kw.kind() == TokenKind::Mut,
            }
        } else if let Some(item) = self.item() {
            Statement::Item(item)
        } else {
            Statement::Expression(self.expression())
        }
    }

    pub(super) fn function(&mut self) -> Function {
        self.eat(TokenKind::Fn);
        let name = self.ident();

        let mut args = Vec::new();
        self.eat(TokenKind::LeftParen);
        while !self.at(TokenKind::RightParen) && !self.eof() {
            let name = self.ident();
            self.eat(TokenKind::Colon);
            let typ = self.ident();
            if !self.at(TokenKind::RightParen) {
                self.eat(TokenKind::Comma);
            }
            args.push(FunctionArg { name, typ });
        }
        self.eat(TokenKind::RightParen);

        let ret = if self.at(TokenKind::Colon) {
            self.next();
            Some(self.ident())
        } else {
            None
        };

        self.eat(TokenKind::LeftBrace);
        let block = self.block();

        Function {
            name,
            args: args.into_boxed_slice(),
            ret,
            block,
        }
    }
}
