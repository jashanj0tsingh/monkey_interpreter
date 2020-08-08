use crate::token;
use crate::token::Token;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: String,
}

impl Lexer {
    pub fn new<S: Into<String>> (input: S) -> Self {
        let mut lexer = Self {
            input: input.into(),
            position: 0,
            read_position: 0,
            ch: "\\".into(),
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespaces();

        let tok: Option<Token> = 
            match self.ch.chars().next() {
                Some('=')         => Some(Token::new(token::ASSIGN,       &self.ch)),
                Some(';')         => Some(Token::new(token::SEMICOLON,    &self.ch)),
                Some('(')         => Some(Token::new(token::LPAREN,       &self.ch)),
                Some(')')         => Some(Token::new(token::RPAREN,       &self.ch)),
                Some(',')         => Some(Token::new(token::COMMA,        &self.ch)),
                Some('+')         => Some(Token::new(token::PLUS,         &self.ch)),
                Some('{')         => Some(Token::new(token::LBRACE,       &self.ch)),
                Some('}')         => Some(Token::new(token::RBRACE,       &self.ch)),
                Some('\\')        => Some(Token::new(token::EOF,          &self.ch)),
                Some('a'..='z')   => {
                    let keywords: std::collections::HashMap<&str, &str> = 
                        [("fn", token::FUNCTION), ("let", token::LET)]
                            .iter()
                            .cloned()
                            .collect();
                    let lookup_identifier = |id: &str| -> String {
                        if let Some(ident) = keywords.get(id) {
                            (&ident).to_string()
                        } else {
                            token::IDENT.to_string()
                        }
                    };
                    // type unassigned
                    let mut tok = Token::new(token::ILLEGAL, self.read_identifier());
                    tok.token_type = lookup_identifier(&tok.literal);
                    return Some(tok);
                },
                Some('0'..='9')   => return Some(Token::new(token::INT,   self.read_number())),
                _                 => Some(Token::new(token::ILLEGAL,      &self.ch)),
            };
        self.read_char();
        tok
    }

    /// finds all subsequent characters that are letters and returns a string
    /// slice representing the identifier's value
    fn read_identifier(&mut self) -> &str {
        let position = self.position;
        while let Some('a'..='z') = self.ch.chars().next() {
            self.read_char()
        }
        &self.input[position..self.position]
    }

    /// finds all subsequent characters that are numbers and returns a string
    /// slice representing the number value
    fn read_number(&mut self) -> &str {
        let position = self.position;
        while let Some('0'..='9') = self.ch.chars().next() {
            self.read_char()
        }
        &self.input[position..self.position]
    }

    /// reads the next character
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = "\\".into();
        } else if let Some(next_input) = self.input.get(self.read_position..self.read_position + 1) {   
            self.ch = next_input.into();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespaces(&mut self) {
        while let Some(c) = self.ch.chars().next() {
            if c.is_whitespace() {
                self.read_char();
                println!("Char is {}", &self.ch);
            } else {
                break;
            }
        }
    }
}


#[test]
fn test_next_token() {

    let input = "=+(){},;";
    let tests: [(&str, &str); 9] = [
        (token::ASSIGN,     "="),
        (token::PLUS,       "+"),
        (token::LPAREN,     "("),
        (token::RPAREN,     ")"),
        (token::LBRACE,     "{"),
        (token::RBRACE,     "}"),
        (token::COMMA,      ","),
        (token::SEMICOLON,  ";"),
        (token::EOF,        "\\"),
    ];

    let mut l = Lexer::new(input);

    for (i, (expected_token, expected_literal)) in tests.iter().enumerate() {
        if let Some(tok) = l.next_token() {
            assert_eq!(
                &tok.token_type, expected_token,
                "tests[{}] - tokentype wrong. expected={}, got={}",
                i, expected_token, tok.token_type,
            );
            assert_eq!(
                &tok.literal, expected_literal,
                "tests[{}] - literal wrong. expected={}, got={}",
                i, expected_literal, tok.literal,
            );
        }
    }
}
#[test]
fn test_next_token_2() {
    let input = "
        let five=5;
        let ten=10;
        
        let add = fn(x, y) {
            x + y;
        };
        
        let result = add(five, ten);";
    let tests: [(&str, &str); 37] =
    [
        (token::LET,        "let"),
        (token::IDENT,      "five"),
        (token::ASSIGN,     "="),
        (token::INT,        "5"),
        (token::SEMICOLON,  ";"),
        (token::LET,        "let"),
        (token::IDENT,      "ten"),
        (token::ASSIGN,     "="),
        (token::INT,        "10"),
        (token::SEMICOLON,  ";"),
        (token::LET,        "let"),
        (token::IDENT,      "add"),
        (token::ASSIGN,     "="),
        (token::FUNCTION,   "fn"),
        (token::LPAREN,     "("),
        (token::IDENT,      "x"),
        (token::COMMA,      ","),
        (token::IDENT,      "y"),
        (token::RPAREN,     ")"),
        (token::LBRACE,     "{"),
        (token::IDENT,      "x"),
        (token::PLUS,       "+"),
        (token::IDENT,      "y"),
        (token::SEMICOLON,  ";"),
        (token::RBRACE,     "}"),
        (token::SEMICOLON,  ";"),
        (token::LET,        "let"),
        (token::IDENT,      "result"),
        (token::ASSIGN,     "="),
        (token::IDENT,      "add"),
        (token::LPAREN,     "("),
        (token::IDENT,      "five"),
        (token::COMMA,      ","),
        (token::IDENT,      "ten"),
        (token::RPAREN,     ")"),
        (token::SEMICOLON,  ";"),
        (token::EOF,        "\\"),
    ];

    let mut l = Lexer::new(input);

    for (i, (expected_token, expected_literal)) in tests.iter().enumerate() {
        if let Some(tok) = l.next_token() {
            assert_eq!(
                &tok.token_type, expected_token,
                "tests[{}] - tokentype wrong. expected={}, got={} with value {}",
                i, expected_token, tok.token_type, tok.literal,
            );
            assert_eq!(
                &tok.literal, expected_literal,
                "tests[{}] - literal wrong. expected={}, got={}",
                i, expected_literal, tok.literal,
            );
        }
    }
}