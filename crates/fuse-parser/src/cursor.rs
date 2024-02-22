use crate::{
    lexer::{Token, TokenKind, TokenReference},
    Parser,
};

impl<'a> Parser<'a> {
    pub(crate) fn cur_token(&self) -> &TokenReference {
        self.lexer.current()
    }

    pub(crate) fn cur_kind(&self) -> TokenKind {
        self.cur_token().kind
    }

    pub(crate) fn consume(&mut self) -> TokenReference {
        let token = self.lexer.consume();
        self.prev_token_end = token.end();
        token
    }

    pub fn consume_if(&mut self, kind: TokenKind) -> Option<TokenReference> {
        if self.at(kind) {
            Some(self.consume())
        } else {
            None
        }
    }

    pub fn at(&self, kind: TokenKind) -> bool {
        self.cur_kind() == kind
    }

    pub fn nth(&mut self, n: u8) -> &TokenReference {
        if n == 0 {
            self.cur_token()
        } else {
            self.lexer.lookahead(n)
        }
    }

    pub fn nth_kind(&mut self, n: u8) -> TokenKind {
        self.nth(n).kind
    }

    pub fn view_token(&self, token: Token) -> &'a str {
        self.lexer.view_token(token)
    }
}
