/// 字句
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// 識別子
    Identifier(String),
    /// 数値リテラル
    Number(f64),
    /// 文字列リテラル
    String(String),

    /// (
    LParen,
    /// )
    RParen,
    /// {
    LBrace,
    /// }
    RBrace,

    /// 演算子
    Operator(Operator),

    /// 予約語
    Reserved(Reserved),

    /// 改行
    NewLine,
}

/// 演算子
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Mul,
    /// /
    Div,
    /// %
    Mod,
    /// ==
    Equal,
    /// ===
    ObjectEqual,
    /// !=
    NotEqual,
    /// >
    GreaterThan,
    /// >=
    GreaterThanEqual,
    /// <
    LessThan,
    /// <=
    LessThanEqual,
    /// &&
    LogicalAnd,
    /// ||
    LogicalOr,
    /// !
    Not,
    /// &
    BitAnd,
    /// |
    BitOr,
    /// =
    Assign,
    /// +=
    AddAssign,
    /// -=
    SubAssign,
    /// *=
    MulAssign,
    /// /=
    DivAssign,
    /// %=
    ModAssign,
}

impl From<&str> for Operator {
    fn from(s: &str) -> Self {
        match s {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            "*" => Operator::Mul,
            "/" => Operator::Div,
            "%" => Operator::Mod,
            "==" => Operator::Equal,
            "===" => Operator::ObjectEqual,
            "!=" => Operator::NotEqual,
            ">" => Operator::GreaterThan,
            ">=" => Operator::GreaterThanEqual,
            "<" => Operator::LessThan,
            "<=" => Operator::LessThanEqual,
            "&&" => Operator::LogicalAnd,
            "||" => Operator::LogicalOr,
            "!" => Operator::Not,
            "&" => Operator::BitAnd,
            "|" => Operator::BitOr,
            "=" => Operator::Assign,
            "+=" => Operator::AddAssign,
            "-=" => Operator::SubAssign,
            "*=" => Operator::MulAssign,
            "/=" => Operator::DivAssign,
            "%=" => Operator::ModAssign,
            _ => panic!("{} is not operator", s),
        }
    }
    
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Reserved {
    /// print文
    Print,

    // return
    Return,

    // typeof
    Typeof,

    // if
    If,

    // else
    Else,

    // for
    For,

    // while
    While,

    // break
    Break,

    // continue
    Continue,

    // function
    Fn,
}

/// 字句解析器
#[derive(Debug)]
pub struct Lexer {
    /// 文字の配列
    pub tokens: Vec<char>,

    /// 現在解析中の文字の位置
    position: usize,

    /// 現在解析中の文字
    current: Option<char>,
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
        let first = input.first().cloned();
        Lexer {
            tokens: input,
            position: 0,
            current: first,
        }
    }

    /// トークンを1つ返す
    pub fn token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let token = self.number()
                .or_else(|| self.new_line())
                .or_else(|| self.paren())
                .or_else(|| self.reserved()) 
                .or_else(|| self.operator())
                .or_else(|| self.string_literal())
                .or_else(|| self.identifier());
        self.next();

        // dbg!(token.clone());

        token
    }

    /// 空白をスキップする
    fn skip_whitespace(&mut self) {
        while self.current.is_some() && is_space(self.current.unwrap()) {
            self.next();
        }
    }

    fn new_line(&mut self) -> Option<Token> {
        if self.current? == '\n' {
            Some(Token::NewLine)
        } else {
            None
        }
    }

    /// 予約語を読み込む
    fn reserved(&mut self) -> Option<Token> {
        match self.current? {
            'p' => self.check_string_with_space("print").then_some(Token::Reserved(Reserved::Print)),
            'r' => self.check_string_with_space("return").then_some(Token::Reserved(Reserved::Return)),
            'i' => self.check_string_with_space("if").then_some(Token::Reserved(Reserved::If)),
            'e' => self.check_string("else").then_some(Token::Reserved(Reserved::Else)),
            'f' => self.check_string_with_space("for").then_some(Token::Reserved(Reserved::For))
                .or_else(|| self.check_string_with_space("fn").then_some(Token::Reserved(Reserved::Fn))),
            't' => self.check_string_with_space("typeof").then_some(Token::Reserved(Reserved::Typeof)),
            _ => None,
        }
    }

    fn check_string_with_space(&mut self, s: &str) -> bool {
        let s_with_space = s.to_owned() + " ";
        self.check_string(&s_with_space)
    }

    fn check_string(&mut self, s: &str) -> bool {
        for (i, char) in s.chars().enumerate() {
            if self.tokens.get(self.position + i) != Some(&char) {
                return false;
            }
        }

        self.position += s.len() - 1;

        true
    }

    /// 数字を読み込む
    fn number(&mut self) -> Option<Token> {
        let mut number_chars = vec![self.current?];

        while self.peek().is_some() && is_part_of_number(self.peek()?) {
            self.next();
            number_chars.push(self.current?);
        }

        String::from_iter(number_chars)
            .parse::<f64>()
            .ok()
            .map(Token::Number)
    }

    /// 括弧を読み込む
    fn paren(&mut self) -> Option<Token> {
        match self.current? {
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            '{' => Some(Token::LBrace),
            '}' => Some(Token::RBrace),
            _ => None,
        }
    }

    /// 演算子を読み込む
    fn operator(&mut self) -> Option<Token> {
        match self.current? {
            '+' => self.tokenize_operator(&["+=", "+"]),
            '-' => self.tokenize_operator(&["-=", "-"]),
            '*' => self.tokenize_operator(&["*=", "*"]),
            '/' => self.tokenize_operator(&["/=", "/"]),
            '%' => self.tokenize_operator(&["%=", "%"]),
            '=' => self.tokenize_operator(&["===", "==", "="]),
            '>' => self.tokenize_operator(&[">=", ">"]),
            '<' => self.tokenize_operator(&["<=", "<"]),
            '&' => self.tokenize_operator(&["&&", "&"]),
            '|' => self.tokenize_operator(&["||", "|"]),
            '!' => self.tokenize_operator(&["!=", "!"]),
            _ => None,
        }
    }

    /// 演算子の候補を受け取り，トークンを返す
    fn tokenize_operator(&mut self, candidates: &[&'static str]) -> Option<Token> {
        for candidate in candidates {
            if self.check_string(candidate) {
                return Some(Token::Operator(Operator::from(*candidate)));
            }
        }

        None
    }

    /// 識別子を読み込む
    fn identifier(&mut self) -> Option<Token> {
        let mut identifier_chars = vec![self.current?];

        while self.peek().is_some() && !self.peek().unwrap().is_whitespace() {
            self.next();
            identifier_chars.push(self.current?);
        }

        Some(Token::Identifier(String::from_iter(identifier_chars)))
    }

    /// 文字列リテラルを読み込む
    fn string_literal(&mut self) -> Option<Token> {
        if self.current? != '"' {
            return None;
        }

        let mut string_chars = vec![];

        while self.peek().is_some() && self.peek() != Some(&'"') {
            self.next();
            string_chars.push(self.current?);
        }

        self.next();

        Some(Token::String(String::from_iter(string_chars)))
    }

    /// positionを進め，
    /// currentを更新する
    pub fn next(&mut self) {
        self.position += 1;
        self.current = self.tokens.get(self.position).cloned();
    }

    /// 現在解析中の文字の次の文字
    pub fn peek(&self) -> Option<&char> {
        self.tokens.get(self.position + 1)
    }

    /// 次の文字が期待している文字かどうか
    pub fn is_peeking(&self, c: &char) -> bool {
        self.peek() == Some(c)
    }
}

/// 数字かどうか
fn is_part_of_number(c: &char) -> bool {
    c.is_ascii_digit() || *c == '.'
}

fn is_space(c: char) -> bool {
    c == ' ' || c == '\t'
}