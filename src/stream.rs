use tantivy::tokenizer::{Token, TokenStream};

#[derive(Debug)]
pub struct BpeTokenStream {
    pub internal_tokens: Vec<String>,
    pub index: usize,
    pub offset_from: usize,
    pub token: Token,
}

impl<'a> TokenStream for BpeTokenStream {
    fn advance(&mut self) -> bool {
        if self.index < self.internal_tokens.len() {
            let token = self.internal_tokens.get(self.index).unwrap();

            self.token = Token {
                offset_from: self.offset_from,
                offset_to: self.offset_from + token.len(),
                position: self.index,
                text: token.to_string(),
                position_length: self.internal_tokens.len(),
            };

            self.offset_from += token.len();
            self.index += 1;

            true
        } else {
            false
        }
    }

    fn token(&self) -> &Token {
        &self.token
    }

    fn token_mut(&mut self) -> &mut Token {
        &mut self.token
    }
}