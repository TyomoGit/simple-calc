/// 字句
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// 識別子
    Identifier(String),
    /// 数字
    Number(f64),
    /// (
    LParen,
    /// )
    RParen,

    /// 演算子
    Operator(Operator),

    /// セミコロン
    Semicolon,

    // return
    Return,
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

        let token = if is_part_of_number(&self.current?) {
            self.number()
        } else {
            self.operator()
                .or_else(|| self.identifier())
        };
        self.next();

        // println!("{:?}, ", token);

        token
    }

    /// 空白をスキップする
    fn skip_whitespace(&mut self) {
        while self.current.is_some() && self.current.unwrap().is_whitespace() {
            self.next();
        }
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

    /// 演算子を読み込む
    fn operator(&mut self) -> Option<Token> {
        match self.current? {
            '+' => self.tokenize_operator('=', Token::Operator(Operator::AddAssign), Token::Operator(Operator::Plus)),
            '-' => self.tokenize_operator('=', Token::Operator(Operator::SubAssign), Token::Operator(Operator::Minus)),
            '*' => self.tokenize_operator('=', Token::Operator(Operator::MulAssign), Token::Operator(Operator::Mul)),
            '/' => self.tokenize_operator('=', Token::Operator(Operator::DivAssign), Token::Operator(Operator::Div)),
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            '%' => self.tokenize_operator('=', Token::Operator(Operator::ModAssign), Token::Operator(Operator::Mod)),
            '=' => self.tokenize_operator('=', Token::Operator(Operator::Equal), Token::Operator(Operator::Assign)),
            '>' => self.tokenize_operator('=', Token::Operator(Operator::GreaterThanEqual), Token::Operator(Operator::GreaterThan)),
            '<' => self.tokenize_operator('=', Token::Operator(Operator::LessThanEqual), Token::Operator(Operator::LessThan)),
            '&' => self.tokenize_operator('&', Token::Operator(Operator::LogicalAnd), Token::Operator(Operator::BitAnd)),
            '|' => self.tokenize_operator('|', Token::Operator(Operator::LogicalOr), Token::Operator(Operator::BitOr)),
            '!' => self.tokenize_operator('=', Token::Operator(Operator::NotEqual), Token::Operator(Operator::Not)),
            _ => None,
        }
    }

    /// 2文字以下の演算子を読み込む
    fn tokenize_operator(&mut self, if_peek: char, matched: Token, not_matched: Token) -> Option<Token> {
        if self.is_peeking(&if_peek) {
            self.next();
            Some(matched)
        } else {
            Some(not_matched)
        }
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