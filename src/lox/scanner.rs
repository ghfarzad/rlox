pub mod token;

use std::collections::HashMap;

use token::Token;
use token::TokenType;

pub struct ScanError {
    line: i32,
    message: String
}

pub struct Scanner {
    source: String,
    tokens: Vec::<Token>,
    start_index: usize,
    current_index: usize,
    line: i32
}

impl ScanError {
    pub fn new(line: i32, msg: &str) -> ScanError {
        return ScanError{
            line: line,
            message: String::from(msg)
        }
    }

    pub fn get_line(&self) -> i32 {
        return self.line;
    }

    pub fn get_message(&self) -> &String {
        return &self.message;
    }
}

impl Scanner {
    pub fn new(src: &str) -> Scanner {
        return Scanner{
            source: String::from(src),
            tokens: Vec::new(),
            start_index: 0,
            current_index: 0,
            line: 1
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec::<Token>, ScanError> {
        let mut c = self.advance();
        while !c.is_none() {
            let Some(a) = c else { todo!(); };
            let tt = match a {
                '(' => Some(TokenType::LeftParen),
                ')' => Some(TokenType::RightParen),
                '{' => Some(TokenType::LeftBrace),
                '}' => Some(TokenType::RightBrace),
                ',' => Some(TokenType::Comma),
                '.' => Some(TokenType::Dot),
                '-' => Some(TokenType::Minus),
                '+' => Some(TokenType::Plus),
                ';' => Some(TokenType::SemiColon),
                '/' => {
                    match self.advance_if('/') {
                        Some(_c) => {
                            let mut p = self.peek();
                            while !p.is_none() && p != Some('\n') {
                                p = self.advance();
                            }
                            None
                        },
                        None    => {
                            Some(TokenType::Slash)
                        }
                    }
                }
                '*' => Some(TokenType::Star),

                //One or two character token
                '!' => {
                    match self.advance_if('=') {
                        Some(_c) => { Some(TokenType::BangEqual) },
                        None     => { Some(TokenType::Bang) }
                    }
                },
                '=' => {
                    match self.advance_if('=') {
                        Some(_c) => { Some(TokenType::EqualEqual) },
                        None     => { Some(TokenType::Equal) }
                    }
                }
                '>' => {
                    match self.advance_if('=') {
                        Some(_c) => { Some(TokenType::GreaterOrEqual) },
                        None     => { Some(TokenType::Greater) }
                    }
                }
                '<' => {
                    match self.advance_if('=') {
                        Some(_c) => { Some(TokenType::LessOrEqual) },
                        None     => { Some(TokenType::Less) }
                    }
                }
                '"' => {
                    if self.scan_string() {
                        let error_message = "Unterminated string literal";
                        return Err(ScanError::new(0, error_message));
                    }
                    self.advance();
                    // Todo: Get the string literal
                    // Remember to strip off surrounding quotes and also
                    // unscape any \n
                    Some(TokenType::Str)
                }

                ' '  => None,
                '\t' => None,
                '\r' => None,
                '\n' => {
                    self.line += 1;
                    None
                }
                _  => {
                    // Todo: Get the numeric literal
                    if a.is_numeric() {
                        self.scan_number();
                        Some(TokenType::Number)
                    }
                    else if a.is_alphabetic() {
                        self.scan_identifier()
                    }
                    else {
                        let error_message = "Unreecognized token";
                        return Err(ScanError::new(0, error_message));
                    }
                }
            };

            if !tt.is_none() {
                unsafe {
                    let t = Token::new(
                        tt.unwrap(),
                        self.source.get_unchecked(
                            self.start_index..self.current_index
                        ),
                        self.line
                    );
                    self.tokens.push(t);
                }
            }

            self.start_index = self.current_index;
            c = self.advance();
        }

        return Ok(self.tokens.clone());
    }

    fn advance(&mut self) -> Option<char> {
        self.current_index += 1;

        return self.source.as_str().chars().nth(self.current_index - 1);
    }

    fn advance_if(&mut self, expected: char) -> Option<char> {
        if self.peek() == Some(expected) {
            return self.advance()
        }

        return None;
    }

    fn peek(&self) -> Option<char> {
        return self.source.as_str().chars().nth(self.current_index);
    }

    fn peek_next(&self) -> Option<char> {
        return self.source.as_str().chars().nth(self.current_index + 1);
    }

    fn scan_string(&mut self) -> bool {
        while self.peek() != Some('"') {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        return self.peek() == Some('"');
    }

    fn scan_number(&mut self) -> Option<bool> {
        let mut p = self.peek();
        while !p.is_none() {
            if p?.is_numeric() {
                p = self.advance();
            }
            else if p == Some('.') {
                if self.peek_next()?.is_numeric() {
                    p = self.advance();
                }
            }
            else {
                p = self.peek();
                break;
            }
        }

        return Some(p?.is_numeric());
    }

    fn scan_identifier(&mut self) -> Option<TokenType> {
        let keywords = HashMap::from([
            ("And",    TokenType::And),
            ("Class",  TokenType::Class),
            ("Else",   TokenType::Else),
            ("False",  TokenType::False),
            ("Fun",    TokenType::Fun),
            ("For",    TokenType::For),
            ("If",     TokenType::If),
            ("Nil",    TokenType::Nil),
            ("Or",     TokenType::Or),
            ("Print",  TokenType::Print),
            ("Return", TokenType::Return),
            ("Super",  TokenType::Super),
            ("This",   TokenType::This),
            ("True",   TokenType::True),
            ("Var",    TokenType::Var),
            ("While",  TokenType::While)
        ]);

        let mut p = self.peek();
        while !p.is_none() {
            if p?.is_alphabetic() || p?.is_numeric() {
                self.advance();
                p = self.peek();
            }
            else {
                break;
            }
        }

        if !p.is_none() {
            let s = &self.source[self.start_index..self.current_index];
            let t = match keywords.get(s) {
                Some(token_type) => { return Some(token_type.clone()); },
                None => { return Some(TokenType::Identifier) }
            };
        }

        None
    }
}
