use crate::errors::ParseError;
use crate::{
    BinOp, Expression, Function, Import, Keyword, ParserDescriptor, Program, Size, Statement,
    Token, Type, Value, Variable,
};
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug)]
pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
    peeked: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens.into_iter().peekable(),
            peeked: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        self.parse_program()
    }

    fn next(&mut self) -> Option<Token> {
        if self.peeked.is_empty() {
            self.tokens.next()
        } else {
            self.peeked.pop()
        }
    }

    fn peek(&mut self) -> Option<Token> {
        if let Some(token) = self.next() {
            self.push(Some(token.clone()));
            Some(token)
        } else {
            None
        }
    }

    fn drop(&mut self, count: usize) {
        for _ in 0..count {
            self.next();
        }
    }

    fn push(&mut self, token: Option<Token>) {
        if let Some(t) = token {
            self.peeked.push(t);
        }
    }

    fn has_more(&mut self) -> bool {
        !self.peeked.is_empty() || self.tokens.peek().is_some()
    }

    fn next_token(&mut self) -> Token {
        self.next().expect("failed to parse")
    }

    fn match_token(&mut self, token: Token) -> Result<Token, ParseError> {
        match self.next_token() {
            ref t if t == &token => Ok(token),
            other => Err(ParseError::UnexpectedToken {
                expected: ParserDescriptor::AnyIdentifier,
                received: other,
            }),
        }
    }

    fn peek_token(&mut self, token: Token) -> Result<Token, ParseError> {
        match self.peek() {
            Some(ref t) if t == &token => Ok(token),
            _ => Err(ParseError::AbsentToken {
                expected: ParserDescriptor::Token(token),
            }),
        }
    }

    fn match_keyword(&mut self, keyword: &Keyword) -> Result<(), ParseError> {
        let token = self.next_token();
        match token {
            Token::Keyword(ref k) if k == keyword => Ok(()),
            other => Err(ParseError::UnexpectedToken {
                expected: ParserDescriptor::Newline,
                received: other,
            }),
        }
    }

    fn match_identifier(&mut self) -> Result<String, ParseError> {
        match self.next_token() {
            Token::Identifier(n) => Ok(n),
            other => Err(ParseError::UnexpectedToken {
                expected: ParserDescriptor::AnyIdentifier,
                received: other,
            }),
        }
    }
}

impl Parser {
    fn parse_program(&mut self) -> Result<Program, ParseError> {
        self.main_parser()
    }

    fn main_parser(&mut self) -> Result<Program, ParseError> {
        let mut functions = Vec::new();
        let mut imports = Vec::new();
        let mut globals = Vec::new();

        while self.has_more() {
            if self.peek().unwrap() == Token::Keyword(Keyword::Use) {
                imports.push(self.parse_import_statement()?);
            } else if self.peek().unwrap() == Token::Keyword(Keyword::Func) {
                functions.push(self.parse_function()?);
            } else {
                globals.push(self.parse_global_vars()?);
            }
        }

        Ok(Program {
            imports,
            func: functions,
            globals: globals.into_iter().collect(),
        })
    }

    fn parse_import_statement(&mut self) -> Result<Import, ParseError> {
        if let Some(Token::Keyword(Keyword::Use)) = self.peek() {
            self.next();
        }
        let mut imports = Vec::new();
        while let Token::Identifier(name) = self.next().unwrap() {
            imports.push(name);
            if let Some(Token::Keyword(Keyword::Func)) = self.peek() {
                break;
            }
            if let Some(Token::Keyword(Keyword::Const)) = self.peek() {
                break;
            }
            if let Some(Token::Keyword(Keyword::Use)) = self.peek() {
                break;
            }
            if let Some(Token::DoubleColon) = self.peek() {
                self.next();
            }
        }
        Ok(Import { name: imports })
    }

    fn parse_global_vars(&mut self) -> Result<Statement, ParseError> {
        match self.next() {
            Some(Token::Keyword(Keyword::Const)) => self.parse_declare(Size::Byte),
            other => {
                self.push(other);
                Ok(Statement::Exp(self.parse_expression()?))
            }
        }
    }

    fn parse_function(&mut self) -> Result<Function, ParseError> {
        self.match_keyword(&Keyword::Func)?;
        let mut is_async = false;
        let return_type: Type;
        if let Some(Token::Keyword(Keyword::Async)) = self.peek() {
            self.next();
            is_async = true;
        }
        // panic!("{:#?}", self.peek());
        let name = self.match_identifier()?;
        self.match_token(Token::OpenParen)?;
        let arguments: Vec<Variable> = match self.peek() {
            Some(Token::CloseParen) => Vec::new(),
            _ => self
                .parse_arguments()
                .expect("Failed to parse function arguments"),
        };

        self.match_token(Token::CloseParen)?;
        if self.peek().unwrap() == Token::Colon {
            self.match_token(Token::Colon)?;
            return_type = self.parse_return()?;
        } else {
            return_type = Type::Void;
        }
        self.match_token(Token::OpenBrace)?;

        let mut statements = vec![];

        while self.peek_token(Token::CloseBrace).is_err() {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        self.match_token(Token::CloseBrace)?;

        Ok(Function {
            is_async,
            name,
            arguments,
            return_type,
            statements,
        })
    }

    fn parse_return(&mut self) -> Result<Type, ParseError> {
        if let Some(Token::Keyword(Keyword::Bool)) = self.peek() {
            self.next();
            return Ok(Type::Bool);
        } else if let Some(Token::Keyword(Keyword::MLstr)) = self.peek() {
            self.next();
            return Ok(Type::Mlstr);
        } else if let Some(Token::Keyword(Keyword::Int)) = self.peek() {
            self.next();
            return Ok(Type::Int);
        } else if let Some(Token::Keyword(Keyword::String)) = self.peek() {
            self.next();
            return Ok(Type::Str);
        } else if let Some(Token::Keyword(Keyword::Void)) = self.peek() {
            self.next();
            return Ok(Type::Void);
        }
        Err(ParseError::AbsentReturnType)
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.next() {
            Some(Token::Keyword(Keyword::Int)) => self.parse_declare(Size::Int),
            Some(Token::Keyword(Keyword::Let)) => self.parse_declare(Size::Byte),
            Some(Token::Keyword(Keyword::Bool)) => self.parse_declare(Size::Byte),
            Some(Token::Keyword(Keyword::Const)) => self.parse_declare(Size::Byte),
            Some(Token::Keyword(Keyword::String)) => self.parse_declare(Size::Byte),
            Some(Token::Keyword(Keyword::Return)) => {
                Ok(Statement::Return(self.parse_expression()?))
            }
            Some(Token::Keyword(Keyword::If)) => self.parse_if_statement(),
            Some(Token::Keyword(Keyword::While)) => self.parse_while_statement(),
            Some(Token::OpenBrace) => self.parse_compond_statement(),
            other => {
                self.push(other);
                Ok(Statement::Exp(self.parse_expression()?))
            }
        }
    }

    fn parse_compond_statement(&mut self) -> Result<Statement, ParseError> {
        let mut statements = vec![];
        while self.peek_token(Token::CloseBrace).is_err() {
            statements.push(self.parse_statement()?);
        }
        self.drop(1);
        Ok(Statement::Compound(statements))
    }

    fn parse_if_statement(&mut self) -> Result<Statement, ParseError> {
        match self.next_token() {
            Token::OpenParen => {
                let condition = self.parse_expression()?;
                self.match_token(Token::CloseParen)?;
                let if_body = self.parse_statement()?;
                match self.peek() {
                    Some(Token::Keyword(Keyword::Else)) => {
                        self.drop(1);
                        let else_body = self.parse_statement()?;
                        Ok(Statement::If(
                            condition,
                            Box::new(if_body),
                            Some(Box::new(else_body)),
                        ))
                    }
                    _ => Ok(Statement::If(condition, Box::new(if_body), None)),
                }
            }
            received => Err(ParseError::UnexpectedToken {
                expected: ParserDescriptor::Token(Token::OpenParen),
                received,
            }),
        }
    }

    fn parse_while_statement(&mut self) -> Result<Statement, ParseError> {
        match self.next_token() {
            Token::OpenParen => {
                let condition = self.parse_expression()?;
                self.match_token(Token::CloseParen)?;
                Ok(Statement::While(
                    condition,
                    Box::new(self.parse_statement()?),
                ))
            }
            received => Err(ParseError::UnexpectedToken {
                expected: ParserDescriptor::Token(Token::OpenParen),
                received,
            }),
        }
    }

    fn parse_declare(&mut self, size: Size) -> Result<Statement, ParseError> {
        match (self.next_token(), self.peek()) {
            (Token::Identifier(name), Some(Token::Assign)) => {
                self.drop(1);
                let exp = self.parse_expression()?;
                Ok(Statement::Declare(Variable { name, size }, Some(exp)))
            }
            _ => Err(ParseError::UnassignedVariable),
        }
    }

    fn parse_assign_op(&mut self, bin_op: BinOp, name: &str) -> Result<Expression, ParseError> {
        let exp = Expression::BinOp(
            bin_op,
            Box::new(Expression::Variable(name.to_string())),
            Box::new(self.parse_expression()?),
        );
        Ok(Expression::Assign(name.to_string(), Box::new(exp)))
    }

    fn parse_inc_op(
        &mut self,
        bin_op: BinOp,
        name: &str,
        postfix: bool,
    ) -> Result<Expression, ParseError> {
        self.next();
        let exp = Expression::BinOp(
            bin_op,
            Box::new(Expression::Variable(name.to_string())),
            Box::new(Expression::Int(1)),
        );
        if postfix {
            Ok(Expression::AssignPostfix(name.to_string(), Box::new(exp)))
        } else {
            Ok(Expression::Assign(name.to_string(), Box::new(exp)))
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_comma_expression()
    }

    fn parse_comma_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_gen_experssion(&[Token::Comma], &Parser::parse_assignment_expression)
    }

    fn parse_assignment_expression(&mut self) -> Result<Expression, ParseError> {
        match (self.next(), self.next()) {
            (Some(Token::Identifier(name)), Some(Token::Assign)) => {
                let exp = self.parse_expression()?;
                Ok(Expression::Assign(name, Box::new(exp)))
            }
            (Some(Token::Identifier(name)), Some(Token::AssignAdd)) => {
                self.parse_assign_op(BinOp::Addition, &name)
            }
            (Some(Token::Identifier(name)), Some(Token::AssignSub)) => {
                self.parse_assign_op(BinOp::Subtraction, &name)
            }
            (Some(Token::Identifier(name)), Some(Token::AssignMul)) => {
                self.parse_assign_op(BinOp::Multiplication, &name)
            }
            (Some(Token::Identifier(name)), Some(Token::AssignDiv)) => {
                self.parse_assign_op(BinOp::Division, &name)
            }
            (Some(Token::Identifier(name)), Some(Token::AssignMod)) => {
                self.parse_assign_op(BinOp::Modulus, &name)
            }
            (Some(Token::Identifier(name)), Some(Token::AssignBitLeft)) => {
                self.parse_assign_op(BinOp::BitwiseLeft, &name)
            }
            (Some(Token::Identifier(name)), Some(Token::AssignBitRight)) => {
                self.parse_assign_op(BinOp::BitwiseRight, &name)
            }
            (Some(Token::Identifier(name)), Some(Token::AssignAnd)) => {
                self.parse_assign_op(BinOp::BitwiseAnd, &name)
            }
            (Some(Token::Identifier(name)), Some(Token::AssignOr)) => {
                self.parse_assign_op(BinOp::BitwiseOr, &name)
            }
            (Some(Token::Identifier(name)), Some(Token::AssignXor)) => {
                self.parse_assign_op(BinOp::BitwiseXor, &name)
            }
            (a, b) => {
                self.push(b);
                self.push(a);
                self.parse_conditional_expression()
            }
        }
    }

    fn parse_conditional_expression(&mut self) -> Result<Expression, ParseError> {
        let mut term = self.parse_or_expression()?;
        while let Some(Token::Question) = self.peek() {
            self.next();
            let body = self.parse_expression()?;
            self.match_token(Token::Colon)?;
            let else_body = self.parse_expression()?;
            term = Expression::Ternary(Box::new(term), Box::new(body), Box::new(else_body))
        }
        Ok(term)
    }

    fn parse_or_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_gen_experssion(&[Token::Or], &Parser::parse_logical_and_expression)
    }

    fn parse_logical_and_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_gen_experssion(&[Token::And], &Parser::parse_bitwise_or_expression)
    }

    fn parse_bitwise_or_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_gen_experssion(&[Token::BitwiseOr], &Parser::parse_bitwise_xor_expression)
    }

    fn parse_bitwise_xor_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_gen_experssion(&[Token::BitwiseXor], &Parser::parse_bitwise_and_expression)
    }

    fn parse_bitwise_and_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_gen_experssion(&[Token::BitwiseAnd], &Parser::parse_equality_expression)
    }

    fn parse_equality_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_gen_experssion(
            &[Token::Equal, Token::NotEqual],
            &Parser::parse_relational_expression,
        )
    }

    fn parse_relational_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_gen_experssion(
            &[
                Token::LessThan,
                Token::GreaterThan,
                Token::LessThanOrEqual,
                Token::GreaterThanOrEqual,
            ],
            &Parser::parse_bitshift_expression,
        )
    }

    fn parse_bitshift_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_gen_experssion(
            &[Token::BitwiseLeft, Token::BitwiseRight],
            &Parser::parse_additive_expression,
        )
    }

    fn parse_additive_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_gen_experssion(
            &[Token::Negation, Token::Addition],
            &Parser::parse_multiplicative_expression,
        )
    }

    fn parse_multiplicative_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_gen_experssion(
            &[Token::Multiplication, Token::Division, Token::Modulus],
            &Parser::parse_factor,
        )
    }

    fn parse_factor(&mut self) -> Result<Expression, ParseError> {
        match (self.next(), self.peek()) {
            (Some(Token::Literal(Value::Char(c))), _) => {
                Ok(Expression::Char(c.chars().as_str().parse().unwrap()))
            }
            (Some(Token::Literal(Value::Int(num))), _) => Ok(Expression::Int(num)),
            (Some(Token::Literal(Value::MLStr(num))), _) => Ok(Expression::MLStr(num)),
            (Some(Token::Identifier(name)), Some(Token::Increment)) => {
                self.parse_inc_op(BinOp::Addition, &name, true)
            }
            (Some(Token::Identifier(name)), Some(Token::Decrement)) => {
                self.parse_inc_op(BinOp::Subtraction, &name, true)
            }
            (Some(Token::Increment), Some(Token::Identifier(name))) => {
                self.parse_inc_op(BinOp::Addition, &name, false)
            }
            (Some(Token::Decrement), Some(Token::Identifier(name))) => {
                self.parse_inc_op(BinOp::Subtraction, &name, false)
            }
            (Some(Token::OpenParen), _) => {
                let exp = self.parse_expression()?;
                self.match_token(Token::CloseParen)?;
                Ok(exp)
            }
            (Some(Token::Identifier(name)), _) => match self.peek() {
                Some(Token::OpenParen) => Ok(Expression::FunctionCall(
                    name,
                    self.parse_function_arguments()?,
                )),
                _ => Ok(Expression::Variable(name)),
            },
            (Some(op @ Token::Negation), _)
            | (Some(op @ Token::LogicalNeg), _)
            | (Some(op @ Token::BitComp), _) => {
                let factor = self.parse_factor()?;
                Ok(Expression::UnOp(op.into(), Box::new(factor)))
            }
            (Some(Token::BitwiseAnd), _) => match self.next() {
                Some(Token::Identifier(name)) => Ok(Expression::VariableRef(name)),
                Some(received) => Err(ParseError::UnexpectedToken {
                    expected: ParserDescriptor::AnyVariable,
                    received,
                }),
                _ => Err(ParseError::Unknown),
            },
            _ => Err(ParseError::UnassignedVariable),
        }
    }

    fn parse_function_arguments(&mut self) -> Result<Vec<Expression>, ParseError> {
        let mut arguments = vec![];
        self.next();
        while self.peek_token(Token::CloseParen).is_err() {
            let exp = self.parse_assignment_expression()?;
            arguments.push(exp);
            if let Some(Token::Comma) = self.peek() {
                self.next();
            }
        }
        self.next();
        Ok(arguments)
    }

    fn parse_arguments(&mut self) -> Result<Vec<Variable>, ParseError> {
        let mut arguments = Vec::new();
        while self.peek_token(Token::CloseParen).is_err() {
            let name = self.match_identifier()?;
            self.match_token(Token::Colon)?;
            let size = match self.next() {
                Some(Token::Keyword(Keyword::Int)) => Ok(Size::Int),
                Some(Token::Keyword(Keyword::String)) => Ok(Size::Byte),
                Some(Token::Keyword(Keyword::MLstr)) => Ok(Size::Byte),
                Some(Token::Keyword(Keyword::Bool)) => Ok(Size::Byte),
                _ => Err(ParseError::UnexpectedType {
                    expected: Type::Int,
                    received: Type::Char,
                }),
            }?;
            arguments.push(Variable { name, size });
            if let Some(Token::Comma) = self.peek() {
                self.next();
            }
        }
        Ok(arguments)
    }

    fn parse_gen_experssion<F>(
        &mut self,
        matching: &[Token],
        next: F,
    ) -> Result<Expression, ParseError>
    where
        F: Fn(&mut Parser) -> Result<Expression, ParseError>,
    {
        let mut term = next(self)?;

        loop {
            match self.peek() {
                Some(ref token) if matching.contains(token) => {
                    let op = self.next().unwrap().into();
                    let next_term = next(self);
                    term = Expression::BinOp(op, Box::new(term), Box::new(next_term?))
                }
                _ => break,
            }
        }
        Ok(term)
    }
}
