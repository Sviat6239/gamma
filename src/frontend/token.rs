use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    //this token is for names of variables, functions, labels, etc.
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    //this token is forced to be a number for number variable
    #[regex(r"[0-9]+")]
    Number,

    //this token is forced to be a string for string variable
    #[regex(r#""([^"\\]|\\.)*""#)]
    String,

    //this token is forced to be a terminators like \t, \n, \r, etc.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,

    //this token is forced to assign value to variable
    #[token("=")]
    Equals,

    //this token forced to compare two statements in if "is equal"
    #[token("==")]
    IsEqual,

    #[token("!=")]
    NotEquals,

    #[token(">")]
    Greater,

    #[token(">=")]
    GreaterOrEqual,

    #[token("<")]
    Less,

    #[token("<=")]
    LessOrEqual,

    #[token("-")]
    Minus,

    #[token("+")]
    Plus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("%")]
    Percent,

    #[token("^")]
    Caret,

    #[token("&")]
    Ampersand,

    #[token("|")]
    Pipe,

    #[token("!")]
    Bang,

    #[token("?")]
    Question,

    #[token("~")]
    Tilde,

    #[token(":")]
    Colon,

    #[token(";")]
    Semicolon,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token(",")]
    Comma,

    //key word to declare variables
    #[token("declare")]
    Declare,

    //key word to reserve space for variables without initializing them
    #[token("reserve")]
    Reserve,

    //key word to declare a constant
    #[token("constant")]
    Constant,

    //declare byte (1 byte)
    #[token("db")]
    Db,

    //declare word (2 byte)
    #[token("dw")]
    Dw,

    //declare double word (4 byte)
    #[token("dd")]
    Dd,

    //declare quad word (8 byte)
    #[token("dq")]
    Dq,

    //reserve byte (1 byte)
    #[token("rb")]
    Rb,

    //reserve word (2 byte)
    #[token("rw")]
    Rw,

    //reserve double word (4 byte)
    #[token("rd")]
    Rd,

    //reserve quad word (8 byte)
    #[token("rq")]
    Rq,

    #[token("fn")]
    Function,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[token("while")]
    While,

    #[token("for")]
    For,

    #[token("in")]
    In,

    #[token("label")]
    Label,

    #[token("goto")]
    Goto,

    #[token("return")]
    Return,

    #[token("break")]
    Break,

    #[token("continue")]
    Continue,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("print")]
    Print,

    #[token("call")]
    Call,

    #[token("asm")]
    Asm,

    #[token("end")]
    End,

    #[token("tostr")]
    ToStr,

    #[token("toint")]
    ToInt,

    #[token("println")]
    PrintLn,

}