use crate::errors::ParseError;
use crate::{
    BinOp, Expression, Function, Import, Keyword, ParserDescriptor, ParsingResult, Program, Size,
    Statement, Token, TokenType, Type, Value, Variable,
};
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug)]
pub struct Parser {
    tokens: Peekable<IntoIter<TokenType>>,
    rawtokens: Vec<TokenType>,
    peeked: Vec<TokenType>,
    file: Box<str>
}

impl Parser {
    pub fn new(tokens: Vec<TokenType>, file:Box<str> 
    ) -> Parser {
        let ntoken = tokens.clone();
        Parser {
            tokens: tokens.into_iter().peekable(),
            rawtokens: ntoken,
            peeked: Vec::new(),
            file: file.to_owned()
        }
    }

    pub fn parse(&mut self) -> Result<ParsingResult, ParseError> {
        self.parse_program()
    }

    fn next(&mut self) -> Option<TokenType> {
        if self.peeked.is_empty() {
            self.tokens.next()
        } else {
            self.peeked.pop()
        }
    }

    fn peek(&mut self) -> Option<Token> {
        if let Some(token) = self.next() {
            self.push(Some(token.clone()));
            Some(token.token)
        } else {
            None
        }
    }

    fn peek_tt(&mut self) -> Option<TokenType> {
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

    fn push(&mut self, token: Option<TokenType>) {
        if let Some(s) = token {
            self.peeked.push(s);
        }
    }

    fn has_more(&mut self) -> bool {
        !self.peeked.is_empty() || self.tokens.peek().is_some()
    }

    fn next_token(&mut self) -> TokenType {
        self.next().expect("failed to parse")
    }

    fn match_token(&mut self, token: Token) -> Result<Token, ParseError> {
        let t= self.next_token();
        let line = t.linenum;
        match t.token {
            ref t if t == &token => Ok(t.to_owned()),
            other => Err(ParseError::UnexpectedToken {
                expected: ParserDescriptor::Token(token),
                filename: self.file.clone(),
                received: other,
                linenum: line,
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
        let line = token.linenum;
        match token.token {
            Token::Keyword(ref k) if k == keyword => Ok(()),
            other => Err(ParseError::UnexpectedToken {
                expected: ParserDescriptor::Newline,
                filename: self.file.clone(),
                received: other,
                linenum: line
            }),
        }
    }

    fn match_identifier(&mut self) -> Result<String, ParseError> {
        let token = self.next_token();
        let line = token.linenum;
        match token.token {
            Token::Identifier(n) => Ok(n),
            other => Err(ParseError::UnexpectedToken {
                expected: ParserDescriptor::AnyIdentifier,
                filename: self.file.clone(),
                received: other,
                linenum: line,
            }),
        }
    }
}

impl Parser {
    fn parse_program(&mut self) -> Result<ParsingResult, ParseError> {
        self.main_parser()
    }

    fn main_parser(&mut self) -> Result<ParsingResult, ParseError> {
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

        Ok(Ok((
            Program {
                imports,
                func: functions,
                globals,
            },
            self.rawtokens.clone(),
        )))
    }

    fn parse_import_statement(&mut self) -> Result<Import, ParseError> {
        if let Some(Token::Keyword(Keyword::Use)) = self.peek() {
            self.next();
        }
        let mut imports = Vec::new();
        while let Token::Identifier(name) = self.next().unwrap().token {
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
        let ne = self.next();
        match ne {
            Some(TokenType {
                token: Token::Keyword(Keyword::Const),
                val: _,
                linenum: line,
            }) => self.parse_declare(Size::Byte, "&str"),
            other => {
                self.push(other);
                Ok(Statement::Exp(self.parse_expression()?))
            }
        }
    }

    fn parse_function(&mut self) -> Result<Function, ParseError> {
        self.match_keyword(&Keyword::Func)?;

        let is_async = match self.peek() {
            Some(Token::Keyword(Keyword::Async)) => {
                self.next();
                true
            }
            _ => false,
        };
        let name = self.match_identifier()?;
        self.match_token(Token::OpenParen)?;
        let arguments: Vec<Variable> = match self.peek() {
            Some(Token::CloseParen) => Vec::new(),
            _ => self.parse_arguments(&name)?,
        };

        self.match_token(Token::CloseParen)?;
        let return_type = match self.peek().unwrap() {
            Token::Colon => {
                self.match_token(Token::Colon)?;
                self.parse_return(&name)?
            }
            _ => Type::Void,
        };
        self.match_token(Token::OpenBrace)?;

        let mut statements = Vec::new();

        while self.peek_token(Token::CloseBrace).is_err() {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        self.match_token(Token::CloseBrace)?;

        Ok(Function {
            is_async,
            name,
            return_type,
            arguments,
            statements,
        })
    }

    fn parse_return(&mut self, fnname: &str) -> Result<Type, ParseError> {
        let typ = match self.peek_tt() {
            Some(TokenType{token: Token::Keyword(Keyword::Bool), val: _, linenum: _}) => Ok(Type::Bool),
            Some(TokenType{token: Token::Keyword(Keyword::MLstr), val: _, linenum: _}) => Ok(Type::Mlstr),
            Some(TokenType{token: Token::Keyword(Keyword::Int), val: _, linenum: _}) => Ok(Type::Int),
            Some(TokenType{token: Token::Keyword(Keyword::String), val: _, linenum: _}) => Ok(Type::Str),
            Some(TokenType{token: Token::Keyword(Keyword::Void), val: _, linenum: _}) => Ok(Type::Void),
            other => Err(ParseError::AbsentReturnType{
                filename: self.file.clone(),
                fnname: fnname.into(),
                linenum: other.unwrap().linenum,
            }),
        };

        if typ.is_ok() {
            self.next();
        }
        typ
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        let ne = self.next();
        match ne {
            Some(TokenType {
                token: Token::Keyword(Keyword::Int),
                val: _,
                linenum: line,
            }) => self.parse_declare(Size::Int, "int"),
            Some(TokenType {
                token: Token::Keyword(Keyword::Let),
                val: _,
                linenum: line,
            }) => self.parse_declare(Size::Byte, ""),
            Some(TokenType {
                token: Token::Keyword(Keyword::Bool),
                val: _,
                linenum: line,
            }) => self.parse_declare(Size::Byte, "bool"),
            Some(TokenType {
                token: Token::Keyword(Keyword::Const),
                val: _,
                linenum: line,
            }) => self.parse_declare(Size::Byte, ""),
            Some(TokenType {
                token: Token::Keyword(Keyword::String),
                val: _,
                linenum: line,
            }) => self.parse_declare(Size::Byte, "str"),
            Some(TokenType {
                token: Token::Keyword(Keyword::Return),
                val: _,
                linenum: line,
            }) => Ok(Statement::Return(self.parse_expression()?)),
            Some(TokenType {
                token: Token::Keyword(Keyword::If),
                val: _,
                linenum: line,
            }) => self.parse_if_statement(),
            Some(TokenType {
                token: Token::Keyword(Keyword::While),
                val: _,
                linenum: line,
            }) => self.parse_while_statement(),
            Some(TokenType {
                token: Token::OpenBrace,
                val: _,
                linenum: line,
            }) => self.parse_compond_statement(),
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
            TokenType{token: Token::OpenParen,linenum: l, val: _} => {
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
                received: received.token,
                filename: self.file.clone(),
                linenum: received.linenum 
            }),
        }
    }

    fn parse_while_statement(&mut self) -> Result<Statement, ParseError> {
        let token = self.next_token(); 
        match token{
            TokenType{token: Token::OpenParen, val: _, linenum: _} => {
                let condition = self.parse_expression()?;
                self.match_token(Token::CloseParen)?;
                Ok(Statement::While(
                    condition,
                    Box::new(self.parse_statement()?),
                ))
            }
            received => Err(ParseError::UnexpectedToken {
                expected: ParserDescriptor::Token(Token::OpenParen),
                filename: self.file.clone(),
                received: received.token,
                linenum: received.linenum
            }),
        }
    }

    fn parse_declare(&mut self, size: Size, t: &str) -> Result<Statement, ParseError> {
        match (self.next_token(), self.peek()) {
            (TokenType{token: Token::Identifier(name), val: _, linenum: _}, Some(Token::Assign)) => {
                self.drop(1);
                let exp = self.parse_expression()?;
                Ok(Statement::Declare(
                    Variable {
                        name,
                        size,
                        t: t.to_string(),
                    },
                    Some(exp),
                ))
            }
            other => Err(ParseError::UnassignedVariable{
                linenum: other.0.linenum,
                filename: self.file.clone()
            }),
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
            (
                Some(TokenType {
                    token: Token::Identifier(name),
                    val: _,
                    linenum: line,
                }),
                Some(TokenType {
                    token: Token::Assign,
                    val: _,
                    linenum: line1,
                }),
            ) => {
                let exp = self.parse_expression()?;
                Ok(Expression::Assign(name, Box::new(exp)))
            }
            (
                Some(TokenType {
                    token: Token::Identifier(name),
                    val: _,
                    linenum: line,
                }),
                Some(TokenType {
                    token: Token::AssignAdd,
                    val: _,
                    linenum: line1,
                }),
            ) => self.parse_assign_op(BinOp::Addition, &name),
            (
                Some(TokenType {
                    token: Token::Identifier(name),
                    val: _,
                    linenum: line,
                }),
                Some(TokenType {
                    token: Token::AssignSub,
                    val: _,
                    linenum: line1,
                }),
            ) => self.parse_assign_op(BinOp::Subtraction, &name),
            (
                Some(TokenType {
                    token: Token::Identifier(name),
                    val: _,
                    linenum: line,
                }),
                Some(TokenType {
                    token: Token::AssignMul,
                    val: _,
                    linenum: line1,
                }),
            ) => self.parse_assign_op(BinOp::Multiplication, &name),
            (
                Some(TokenType {
                    token: Token::Identifier(name),
                    val: _,
                    linenum: line,
                }),
                Some(TokenType {
                    token: Token::AssignDiv,
                    val: _,
                    linenum: line1,
                }),
            ) => self.parse_assign_op(BinOp::Division, &name),
            (
                Some(TokenType {
                    token: Token::Identifier(name),
                    val: _,
                    linenum: line,
                }),
                Some(TokenType {
                    token: Token::AssignMod,
                    val: _,
                    linenum: line1,
                }),
            ) => self.parse_assign_op(BinOp::Modulus, &name),
            (
                Some(TokenType {
                    token: Token::Identifier(name),
                    val: _,
                    linenum: line,
                }),
                Some(TokenType {
                    token: Token::AssignBitLeft,
                    val: _,
                    linenum: line1,
                }),
            ) => self.parse_assign_op(BinOp::BitwiseLeft, &name),
            (
                Some(TokenType {
                    token: Token::Identifier(name),
                    val: _,
                    linenum: line,
                }),
                Some(TokenType {
                    token: Token::AssignBitRight,
                    val: _,
                    linenum: line1,
                }),
            ) => self.parse_assign_op(BinOp::BitwiseRight, &name),
            (
                Some(TokenType {
                    token: Token::Identifier(name),
                    val: _,
                    linenum: line,
                }),
                Some(TokenType {
                    token: Token::AssignAnd,
                    val: _,
                    linenum: line1,
                }),
            ) => self.parse_assign_op(BinOp::BitwiseAnd, &name),
            (
                Some(TokenType {
                    token: Token::Identifier(name),
                    val: _,
                    linenum: line,
                }),
                Some(TokenType {
                    token: Token::AssignOr,
                    val: _,
                    linenum: line1,
                }),
            ) => self.parse_assign_op(BinOp::BitwiseOr, &name),
            (
                Some(TokenType {
                    token: Token::Identifier(name),
                    val: _,
                    linenum: line,
                }),
                Some(TokenType {
                    token: Token::AssignXor,
                    val: _,
                    linenum: line1,
                }),
            ) => self.parse_assign_op(BinOp::BitwiseXor, &name),
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
            (
                Some(TokenType {
                    token: Token::Literal(Value::Char(c)),
                    val: _,
                    linenum: line,
                }),
                _,
            ) => Ok(Expression::Char(c.chars().as_str().parse().unwrap())),
            (
                Some(TokenType {
                    token: Token::Keyword(Keyword::True),
                    val: _,
                    linenum: line,
                }),
                _,
            ) => Ok(Expression::Bool(true)),
            (
                Some(TokenType {
                    token: Token::Keyword(Keyword::False),
                    val: _,
                    linenum: line,
                }),
                _,
            ) => Ok(Expression::Bool(false)),
            (
                Some(TokenType {
                    token: Token::Literal(Value::Int(num)),
                    val: _,
                    linenum: line,
                }),
                _,
            ) => Ok(Expression::Int(num)),
            (
                Some(TokenType {
                    token: Token::Literal(Value::MLStr(num)),
                    val: _,
                    linenum: line,
                }),
                _,
            ) => Ok(Expression::MLStr(num)),
            (
                Some(TokenType {
                    token: Token::Identifier(name),
                    val: _,
                    linenum: line,
                }),
                Some(Token::Increment),
            ) => self.parse_inc_op(BinOp::Addition, &name, true),
            (
                Some(TokenType {
                    token: Token::Identifier(name),
                    val: _,
                    linenum: line,
                }),
                Some(Token::Decrement),
            ) => self.parse_inc_op(BinOp::Subtraction, &name, true),
            (
                Some(TokenType {
                    token: Token::Increment,
                    val: _,
                    linenum: line,
                }),
                Some(Token::Identifier(name)),
            ) => self.parse_inc_op(BinOp::Addition, &name, false),
            (
                Some(TokenType {
                    token: Token::Decrement,
                    val: _,
                    linenum: line,
                }),
                Some(Token::Identifier(name)),
            ) => self.parse_inc_op(BinOp::Subtraction, &name, false),
            (
                Some(TokenType {
                    token: Token::OpenParen,
                    val: _,
                    linenum: line,
                }),
                _,
            ) => {
                let exp = self.parse_expression()?;
                self.match_token(Token::CloseParen)?;
                Ok(exp)
            }
            (
                Some(TokenType {
                    token: Token::Identifier(name),
                    val: _,
                    linenum: line,
                }),
                _,
            ) => match self.peek() {
                Some(Token::OpenParen) => Ok(Expression::FunctionCall(
                    name,
                    self.parse_function_arguments()?,
                )),
                _ => Ok(Expression::Variable(name)),
            },
            (
                Some(
                    op
                    @
                    TokenType {
                        token: Token::Negation,
                        val: _,
                        linenum: _,
                    },
                ),
                _,
            )
            | (
                Some(
                    op
                    @
                    TokenType {
                        token: Token::LogicalNeg,
                        val: _,
                        linenum: _,
                    },
                ),
                _,
            )
            | (
                Some(
                    op
                    @
                    TokenType {
                        token: Token::BitComp,
                        val: _,
                        linenum: _,
                    },
                ),
                _,
            ) => {
                let factor = self.parse_factor()?;
                Ok(Expression::UnOp(op.token.into(), Box::new(factor)))
            }
            (
                Some(TokenType {
                    token: Token::BitwiseAnd,
                    val: _,
                    linenum: line,
                }),
                _,
            ) => match self.next() {
                Some(TokenType {
                    token: Token::Identifier(name),
                    val: _,
                    linenum: line,
                }) => Ok(Expression::VariableRef(name)),
                Some(received) => Err(ParseError::UnexpectedToken {
                    expected: ParserDescriptor::AnyVariable,
                    received: received.token,
                    filename: self.file.clone(),
                    linenum: received.linenum
                }),
                _ => Err(ParseError::Unknown),
            },
            other => Err(ParseError::UnassignedVariable{
                linenum: other.0.unwrap().linenum,
                filename: self.file.clone(),
            }),
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

    fn parse_arguments(&mut self, fnname: &str) -> Result<Vec<Variable>, ParseError> {
        let mut arguments = Vec::new();
        while self.peek_token(Token::CloseParen).is_err() {
            let name = self.match_identifier()?;
            self.match_token(Token::Colon)?;
            let t = match self.peek() {
                Some(Token::Keyword(Keyword::Int)) => "int",
                Some(Token::Keyword(Keyword::String)) => "str",
                Some(Token::Keyword(Keyword::MLstr)) => "mlstr",
                Some(Token::Keyword(Keyword::Bool)) => "bool",
                _ => panic!("Function {} has wrong argument type", name),
            };
            let size = match self.next() {
                Some(TokenType {
                    token: Token::Keyword(Keyword::Int),
                    val: _,
                    linenum: line,
                }) => Ok(Size::Int),
                Some(TokenType {
                    token: Token::Keyword(Keyword::String),
                    val: _,
                    linenum: line,
                }) => Ok(Size::Byte),
                Some(TokenType {
                    token: Token::Keyword(Keyword::MLstr),
                    val: _,
                    linenum: line,
                }) => Ok(Size::Byte),
                Some(TokenType {
                    token: Token::Keyword(Keyword::Bool),
                    val: _,
                    linenum: line,
                }) => Ok(Size::Byte),
                other => Err(ParseError::UnexpectedType {
                    expected: Type::Int,
                    received: Type::Char,
                    filename: self.file.clone(),
                    linenum: other.unwrap().linenum
                }),
            }?;
            arguments.push(Variable {
                name,
                size,
                t: t.to_string(),
            });
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
                    let op = self.next().unwrap().token.into();
                    let next_term = next(self);
                    term = Expression::BinOp(op, Box::new(term), Box::new(next_term?))
                }
                _ => break,
            }
        }
        Ok(term)
    }
}
