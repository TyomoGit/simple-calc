use std::rc::Rc;

use crate::token::Lexer;
use crate::token::Token;
use crate::token::Reserved;
use crate::token::Operator;

#[derive(Debug, Clone)]
pub enum Statement {
    Return(Box<Expr>),
    Print(Box<Expr>),
    Expr(Box<Expr>),
    Block(Vec<Statement>),
    If {
        condition: Box<Expr>,
        block: Box<Statement>,
        else_block: Option<Box<Statement>>,
    },
}

#[derive(Debug, Clone)]
pub struct ReferenceType<T> {
    pub value: Rc<T>
}

/// 式
#[derive(Debug, Clone)]
pub enum Expr {
    /// 識別子
    Identifier(String),

    /// 数字
    Number(f64),

    /// 文字列
    String(ReferenceType<String>),

    /// 前置演算子
    PrefixExpr {
        operator: Operator,
        right: Box<Expr>,
    },

    /// 中置演算子
    InfixExpr {
        left: Box<Expr>,
        operator: Operator,
        right: Box<Expr>,
    },

    /// 後置演算子
    PostfixExpr {
        left: Box<Expr>,
        operator: Operator,
    },
}

impl From<&Token> for Operator {
    fn from(value: &Token) -> Self {
        if let Token::Operator(operator) = value {
            operator.clone()
        } else {
            panic!("invalid operator");
        }
    }
}

/// 演算子の優先度
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Precedence {
    /// 最低
    Lowest,
    /// 代入と複合代入
    Assign,
    /// ||
    LogicalOr,
    /// &&
    LogicalAnd,
    /// |
    BitOr,
    /// &
    BitAnd,
    /// ==, !=
    Equality,
    /// <, >, <=, >=
    Compare,
    /// +, -
    Sum,
    /// *, /
    Product,
    /// 前置演算子
    Prefix,
    ///後置演算子
    Postfix,
}

impl From<&Token> for Precedence {
    /// トークンの優先度を返す
    fn from(value: &Token) -> Self {
        let Token::Operator(operator) = value else {
            return Precedence::Lowest;
        };

        match operator {
            Operator::Assign | Operator::AddAssign | Operator::SubAssign | Operator::MulAssign | Operator::DivAssign | Operator::ModAssign => Precedence::Assign,
            Operator::BitOr => Precedence::BitOr,
            Operator::BitAnd => Precedence::BitAnd,
            Operator::LogicalOr => Precedence::LogicalOr,
            Operator::LogicalAnd => Precedence::LogicalAnd,
            Operator::Equal | Operator::NotEqual => Precedence::Equality,
            Operator::GreaterThan | Operator::GreaterThanEqual | Operator::LessThan | Operator::LessThanEqual | Operator::ObjectEqual => Precedence::Compare,
            Operator::Plus | Operator::Minus => Precedence::Sum,
            Operator::Div | Operator::Mul | Operator::Mod => Precedence::Product,
            Operator::Not => Precedence::Prefix,

        }
    }
}

/// 構文解析器
pub struct Parser {
    /// 字句解析器
    lexer: Lexer,
    /// 現在のトークン
    current: Option<Token>,
    /// 次のトークン
    peek: Option<Token>,
}

/// 関連関数
impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.token();
        let peek = lexer.token();

        Parser {
            lexer,
            current,
            peek,
        }
    }
}

/// インスタンスメソッド
impl Parser {
    pub fn next(&mut self) {
        self.current = self.peek.clone();
        self.peek = self.lexer.token();
    }

    /// 解析を開始する
    pub fn parse(&mut self) -> Option<Vec<Statement>> {
        let mut statements = Vec::new();
        
        while self.current.is_some() {
            let statement = self.parse_statement()?;
            statements.push(*statement);

            self.skip_newline_eof();

            self.next();
        }

        Some(statements)
    }

    fn skip_newline_eof(&mut self) {
        while self.is_peek(&Token::NewLine) || self.peeking_eof() {
            self.next();
            if self.current.is_none() { break; }
        }
    }

    pub fn parse_statement(&mut self) -> Option<Box<Statement>> {
        match self.current.as_ref()? {
            Token::Reserved(Reserved::Print) => self.parse_print_statement(),
            Token::Reserved(Reserved::Return) => self.parse_return_statement(),
            Token::Reserved(Reserved::If) => self.parse_if_statement(),
            _ => self.parse_expr(Precedence::Lowest).map(|expr| Box::new(Statement::Expr(expr))),
        }
    }

    fn parse_block(&mut self) -> Option<Box<Statement>> {
        if self.current.as_ref()? != &Token::LBrace { return None; }

        self.skip_newline_eof();
        self.next();

        let mut statements = Vec::new();
        while *self.current.as_ref()? != Token::RBrace && !self.is_peek(&Token::RBrace) && !self.peeking_eof() {
            let statement = self.parse_statement()?;

            statements.push(*statement);

            self.skip_newline_eof();
            self.next();
        }

        Some(Box::new(Statement::Block(statements)))
    }

    /// 式を解析する
    pub fn parse_expr(&mut self, precedence: Precedence) -> Option<Box<Expr>> {
        let mut left = self.parse_prefix()?;

        while precedence < self.peeking_precedence() {
            self.next();
            left = self.parse_postfix(left.clone())
                .or_else(|| self.parse_infix(left))?;
        }

        // println!("{:?}", left);

        Some(left)
    }

    fn parse_print_statement(&mut self) -> Option<Box<Statement>> {
        if self.current.as_ref()? != &Token::Reserved(Reserved::Print) { return None; }
        self.next();

        let expression = self.parse_expr(Precedence::Lowest);

        if self.is_peek(&Token::NewLine) || self.peeking_eof() || self.is_peek(&Token::RBrace) {
            expression.map(|expr| Box::new(Statement::Print(expr)))
        } else {
            None
        }
    }

    fn parse_return_statement(&mut self) -> Option<Box<Statement>> {
        if self.current.as_ref()? != &Token::Reserved(Reserved::Return) { return None; }

        self.next();
        let expression = self.parse_expr(Precedence::Lowest);

        if self.is_peek(&Token::NewLine) || self.peeking_eof() {
            expression.map(|expr| Box::new(Statement::Return(expr)))
        } else {
            None
        }
    }

    fn parse_if_statement(&mut self) -> Option<Box<Statement>> {
        if self.current.as_ref()? != &Token::Reserved(Reserved::If) { return None; }

        self.next();

        let condition = self.parse_expr(Precedence::Lowest);

        self.next();

        let block = self.parse_block()?;

        let mut else_block: Option<Box<Statement>> = None;

        if self.is_peek(&Token::Reserved(Reserved::Else)) {
            self.next();
            else_block = self.parse_block();
        }

        Some(Box::new(Statement::If {
            condition: condition?,
            block,
            else_block,
        }))
    }

    /// 前置演算子式，識別子，数字を解析する
    pub fn parse_prefix(&mut self) -> Option<Box<Expr>> {
        match self.current.as_ref()? {
            Token::Operator(Operator::Plus) | Token::Operator(Operator::Minus) | Token::Operator(Operator::Not) => self.parse_prefix_expr(),
            Token::Identifier(name) => {
                Some(Box::new(Expr::Identifier(name.clone())))
            }
            Token::Number(_) => self.parse_number(),
            Token::String(_) => self.parse_string(),
            Token::LParen => self.parse_grouped_expr(),
            _ => None,
        }
    }

    /// 前置演算子式を解析する
    pub fn parse_prefix_expr(&mut self) -> Option<Box<Expr>> {
        match self.current.as_ref()? {
            Token::Operator(_) => (),
            _ => return None,
        };

        let operator = Operator::from(self.current.as_ref()?);
        self.next();

        let number = self.parse_expr(Precedence::Prefix);

        match operator {
            Operator::Plus | Operator::Minus | Operator::Not => Some(Box::new(Expr::PrefixExpr {
                operator,
                right: number?,
            })),
            _ => None,
        }
    }

    /// 数字を解析する
    pub fn parse_number(&mut self) -> Option<Box<Expr>> {
        if let Some(Token::Number(n)) = self.current {
             Some(Box::new(Expr::Number(n)))
        } else {
            None
        }
    }

    /// 文字列を解析する
    pub fn parse_string(&mut self) -> Option<Box<Expr>> {
        if let Some(Token::String(s)) = self.current.as_ref() {
             Some(Box::new(Expr::String(
                    ReferenceType {
                        value: s.clone().into(),
                    }
             )))
        } else {
            None
        }
    }

    /// 括弧で囲まれた式を解析する
    pub fn parse_grouped_expr(&mut self) -> Option<Box<Expr>> {
        self.next();
        let expression = self.parse_expr(Precedence::Lowest);

        if self.is_peek(&Token::RParen) {
            self.next();
            expression
        } else {
            None
        }
    }

    /// 後置演算子式を解析する
    pub fn parse_postfix(&mut self, _left: Box<Expr>) -> Option<Box<Expr>> {
        let token = self.current.as_ref()?;
        let _operator = Operator::from(token);

        // ここに追加していく
        
        // match operator {
        //     _ => None,
        // }
        None
    }

    /// 中置演算子式の場合に式を解析する
    pub fn parse_infix(&mut self, left: Box<Expr>) -> Option<Box<Expr>> {
        let token = self.current.as_ref()?;
        let Token::Operator(operator) = token else {
            return Some(left);
        };

        match operator {
            Operator::Plus | Operator::Minus | Operator::Mul | Operator::Div | Operator::Mod
            | Operator::Equal | Operator::NotEqual
            | Operator::GreaterThan | Operator::GreaterThanEqual | Operator::LessThan | Operator::LessThanEqual | Operator::ObjectEqual
            | Operator::LogicalAnd | Operator::LogicalOr
            | Operator::Assign | Operator::AddAssign | Operator::SubAssign | Operator::MulAssign | Operator::DivAssign | Operator::ModAssign
            | Operator::BitAnd | Operator::BitOr => self.parse_infix_expr(left),
            _ => Some(left),
        }
    }

    /// 中置演算子式を解析する
    pub fn parse_infix_expr(&mut self, left: Box<Expr>) -> Option<Box<Expr>> {
        let operator = Operator::from(self.current.as_ref()?);
        let precedence = Precedence::from(self.current.as_ref()?);

        self.next();

        let right = self.parse_expr(precedence)?;

        Some(Box::new(Expr::InfixExpr {
            left,
            operator,
            right,
        }))
    }

    /// 次のトークンの優先度を返す
    pub fn peeking_precedence(&self) -> Precedence {
        let token = &self.peek;

        if token.is_none() {
            Precedence::Lowest
        } else {
            Precedence::from(token.as_ref().unwrap())
        }
    }

    /// 次のトークンが引数のトークンと同じかどうかを返す
    pub fn is_peek(&self, token: &Token) -> bool {
        if self.peek.is_none() {
            false
        } else {
            self.peek.as_ref().unwrap() == token
        }
    }

    pub fn peeking_eof(&self) -> bool {
        self.peek.is_none()
    }
}