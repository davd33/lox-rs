use std::collections::VecDeque;
use derive_more::Display;

#[derive(Display)]
enum ErrorKind {
	ScannerError,
}

struct AppError {
	message: String,
	kind: ErrorKind,
}

impl AppError {
	fn new(message: &str, kind: ErrorKind) -> AppError {
		AppError {
			message: message.to_string(), kind,
		}
	}
}

struct Scanner {
	source: String,
	current: usize,
	start: usize,
	line: usize,
	tokens: Vec<Token>,
	errors: VecDeque<AppError>
}

impl Scanner {
	fn new(source: String) -> Self {
		Scanner {
			source,
			current: 0,
			start: 0,
			line: 1,
			tokens: vec![],
			errors: VecDeque::new(),
		}
	}

	fn has_errors(&self) -> bool {
		!self.errors.is_empty()
	}

	fn advance(&mut self) -> Option<char> {
		let c = self.source[self.current..].chars().next();
		if let Some(c) = c {
			self.current += c.len_utf8();
		}
		c
	}

	fn add_token(&mut self, _type: TokenType) {
		self.add_token_literal(_type, None);
	}

	fn add_token_literal(&mut self, _type: TokenType, literal: Option<Literal>) {
		self.tokens.push(
			Token::new(_type,
			           self.source[self.start..self.current].to_string(),
			           literal,
			           self.line));
	}

	fn print_errors(&mut self) {
		while let Some(e) = self.errors.pop_front() {
			println!("{} ({})", e.message, e.kind);
		}
	}

	fn scan_token(&mut self) {
		if let Some(c) = self.advance() {
			match c {
				'(' => self.add_token(TokenType::LeftParen),
				')' => self.add_token(TokenType::RightParen),
				'{' => self.add_token(TokenType::LeftBrace),
				'}' => self.add_token(TokenType::RightBrace),
				',' => self.add_token(TokenType::Comma),
				'.' => self.add_token(TokenType::Dot),
				'-' => self.add_token(TokenType::Minus),
				'+' => self.add_token(TokenType::Plus),
				';' => self.add_token(TokenType::Semicolon),
				'*' => self.add_token(TokenType::Star),
				_ => self.errors.push_back(AppError::new("Unknown token!", ErrorKind::ScannerError)),
			}
		}
	}

	fn is_at_end(&self) -> bool {
		self.current >= self.source.chars().count()
	}

	pub fn scan_tokens(&mut self) -> Vec<Token> {
		let mut result = vec![];

		while !self.is_at_end() {
			self.start = self.current;
			self.scan_token();
		}

		result.push(Token::new(TokenType::Eof, String::new(), None, 0,));

		self.print_errors();

		result
	}
}

#[derive(Display)]
enum TokenType {
	// Single-character tokens.
	LeftParen, RightParen, LeftBrace, RightBrace,
	Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

	// One or two character tokens.
	Bang, BangEqual,
	Equal, EqualEqual,
	Greater, GreaterEqual,
	Less, LessEqual,

	// Literals.
	Identifier, String, Number,

	// Keywords.
	And, Class, Else, False, Fun, For, If, Nil, Or,
	Print, Return, Super, This, True, Var, While,

	Eof,
}

struct Literal;

#[derive(Display)]
#[display("{line:} {lexeme} ({_type})")]
struct Token {
	_type: TokenType,
	lexeme: String,
	literal: Option<Literal>,
	line: usize,
}

impl Token {
	pub fn new(_type: TokenType,
	           lexeme: String,
	           literal: Option<Literal>,
	           line: usize) -> Self {

		Token {
			_type,
			lexeme,
			literal,
			line,
		}
	}
}

fn run(source: String) {
	let mut scanner = Scanner::new(source);
	let tokens = scanner.scan_tokens();

	for t in tokens.iter() {
		println!("{t}");
	}
}

fn main() {
	run("print 'hello world'".to_string());
}
