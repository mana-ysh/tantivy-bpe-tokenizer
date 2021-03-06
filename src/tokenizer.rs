use crate::stream::BpeTokenStream;
use tantivy::tokenizer::{BoxTokenStream, Tokenizer};
use tokenizers::Tokenizer as HuggingfaceTokenizer;

#[derive(Clone, Debug)]
pub struct BpeTokenizer {
    hf_tokenizer: HuggingfaceTokenizer
}

impl BpeTokenizer {
    pub fn new_from_file(json_path: &str) -> Self {
        BpeTokenizer {
            hf_tokenizer: HuggingfaceTokenizer::from_file(json_path).unwrap()
        }
    }
}

impl Tokenizer for BpeTokenizer {
    fn token_stream<'a>(&self, text: &'a str) -> BoxTokenStream<'a> {
        // NOTE: consider [UNK].
        let output = self.hf_tokenizer.encode(text, true).unwrap();
        let internal_tokens: Vec<String> = output.get_tokens()
            .iter()
            .zip(output.get_offsets().iter())
            .map(|(token, offset)| {
                if token == "[UNK]" {
                    text[offset.0..offset.1].to_string()
                 } else {
                    token.to_string()
                }
            })
            .collect();

        BoxTokenStream::from(
            BpeTokenStream {
                internal_tokens: internal_tokens,
                token: Default::default(),
                index: 0,
                offset_from: 0,
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;
    use tantivy::tokenizer::{Token, Tokenizer};

    #[test]
    fn test_tokenizer() {
        let mut json_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        json_path.push("tests/resources/example.json");
        let tokenizer = BpeTokenizer::new_from_file(&json_path.into_os_string().into_string().unwrap());
        
        // English
        let mut token_stream = tokenizer.token_stream("This is a pen");
        let mut tokens: Vec<Token> = vec![];
        token_stream.process(&mut |token: &Token| tokens.push(token.clone()));

        assert_eq!(tokens.len(), 4);

        assert_eq!(tokens[0].text, "This");
        assert_eq!(tokens[0].offset_from, 0);
        assert_eq!(tokens[0].offset_to, 4);
        assert_eq!(tokens[0].position, 0);

        assert_eq!(tokens[1].text, "is");
        assert_eq!(tokens[1].offset_from, 4);
        assert_eq!(tokens[1].offset_to, 6);
        assert_eq!(tokens[1].position, 1);
        
        assert_eq!(tokens[2].text, "a");
        assert_eq!(tokens[2].offset_from, 6);
        assert_eq!(tokens[2].offset_to, 7);
        assert_eq!(tokens[2].position, 2);

        assert_eq!(tokens[3].text, "pen");
        assert_eq!(tokens[3].offset_from, 7);
        assert_eq!(tokens[3].offset_to, 10);
        assert_eq!(tokens[3].position, 3);

        // UNK case (Japanese)
        let mut token_stream = tokenizer.token_stream("?????????");
        let mut tokens: Vec<Token> = vec![];
        token_stream.process(&mut |token: &Token| tokens.push(token.clone()));

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].text, "???");
        assert_eq!(tokens[0].offset_from, 0);
        assert_eq!(tokens[0].offset_to, 3);
        assert_eq!(tokens[0].position, 0);

        assert_eq!(tokens[1].text, "???");
        assert_eq!(tokens[1].offset_from, 3);
        assert_eq!(tokens[1].offset_to, 6);
        assert_eq!(tokens[1].position, 1);

        assert_eq!(tokens[2].text, "???");
        assert_eq!(tokens[2].offset_from, 6);
        assert_eq!(tokens[2].offset_to, 9);
        assert_eq!(tokens[2].position, 2);
    }
}