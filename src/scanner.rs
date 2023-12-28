mod token;

use token::Token;

pub struct Scanner {
    source: String
}

impl Scanner {
    pub fn new(src: &String) -> Scanner {
        return Scanner{
            source: String::from(src)
        }
    }

    pub fn scan_tokens(&self) -> Token
    {
        return Token{};
    }
}
