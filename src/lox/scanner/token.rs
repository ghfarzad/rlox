use core::fmt;

#[derive(Clone)]
pub enum TokenType {
    //Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    //One or tow character token
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,

    //Literals
    Identifier,
    Str,
    Number,

    //Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", "FOO").ok();
        Ok(())
    }
}

#[derive(Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    //literal: Object,
    line: i32
}

impl Token {
    pub fn new( token_type: TokenType, lexeme: &str, /*literal: &Object,*/ line: i32 ) -> Token {
        return Token {
            token_type: token_type,
            lexeme: String::from(lexeme),
            //literal: literal,
            line: line
        };
    }

    pub fn to_string(&self) -> String
    {
        //format!( "{} {} {}", self.tokenType, self.lexeme, self.literal)
        format!( "{} {}", self.token_type, self.lexeme)
    }
}
