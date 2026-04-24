use derive_more::Display;
use std::collections::VecDeque;

#[derive(Display)]
enum ErrorKind {
	ScannerError,
}

struct AppError {
	message: String,
	kind: ErrorKind,
	line: usize,
}

impl AppError {
	fn new(line: usize, message: &str, kind: ErrorKind) -> AppError {
		AppError {
			message: message.to_string(), kind, line
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

	fn inc_current(&mut self, c: char) {
		self.current += c.len_utf8();
	}

	fn current_char(&self) -> Option<char> {
		self.source[self.current..].chars().next()
	}

	fn next_char(&self) -> Option<char> {
		self.source[self.current+1..].chars().next()
	}

	fn advance(&mut self) -> Option<char> {
		let c = self.current_char();
		if let Some(c) = c {
			self.inc_current(c);
		}
		c
	}

	fn _match(&mut self, ch: char) -> bool {
		if self.is_at_end() {
			return false;
		}

		if let Some(c) = self.current_char() {
			if c != ch {
				return false;
			}
			self.inc_current(c);
		}

		true
	}

	fn peek(&self) -> Option<char> {
		self.current_char()
	}

	fn peek_next(&self) -> Option<char> {
		self.next_char()
	}

	fn string(&mut self)  {
		if let Some(ch) = self.peek() {
			while ch != '\'' && !self.is_at_end() {
				if ch == '\n' {
					self.line += 1;
				}
				self.advance();
			}

			if self.is_at_end() {
				self.new_error("Unterminated string.", self.line);
				return;
			}

			self.advance();

			let value = self.source[self.start+1..self.current-1].to_string();
			self.add_token_literal(TokenType::String, Some(Literal::String(value)));
		}
	}

	fn is_digit(&self, ch: char) -> bool {
		ch >= '0' && ch <= '9'
	}

	fn number(&mut self) {
		while self.peek().is_some_and(|ch| self.is_digit(ch)) {
			self.advance();
		}

		if let Some(ch) = self.peek() {
			if ch == '.' && self.peek_next().is_some_and(|ch| self.is_digit(ch)) {
				self.advance();
				while self.peek().is_some_and(|ch| self.is_digit(ch)) {
					self.advance();
				}
			}
		}

		match self.source[self.start..self.current].to_string().parse::<f64>() {
			Ok(value) => {
				self.add_token_literal(TokenType::Number, Some(Literal::Number(value)));
			}
			Err(_) => {
				self.new_error("Not a number!", self.line);
			}
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
				'!' => {
					let token_type = if self._match('=')  { TokenType::BangEqual } else { TokenType::Bang };
					self.add_token(token_type);
				},
				'=' => {
					let token_type = if self._match('=')  { TokenType::EqualEqual } else { TokenType::Equal };
					self.add_token(token_type);
				},
				'>' => {
					let token_type = if self._match('=')  { TokenType::GreaterEqual } else { TokenType::Greater };
					self.add_token(token_type);
				},
				'<' => {
					let token_type = if self._match('=')  { TokenType::LessEqual } else { TokenType::Less };
					self.add_token(token_type);
				},
				'/' => {
					if self._match('/') {
						while self.peek().is_some_and(|c| c != '\n') && !self.is_at_end() {
							self.advance();
						}
					} else {
						self.add_token(TokenType::Slash);
					}
				},
				' ' | '\r' | '\t' => {},
				'\n' => self.line += 1,
				'\'' => self.string(),
				d if self.is_digit(d) => self.number(),
				_ => self.new_error("Unknown token!", self.line),
			}
		}
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
			println!("{}: {} ({})", e.line, e.message, e.kind);
		}
	}

	fn new_error(&mut self, message: &str, line: usize) {
		self.errors.push_back(AppError::new(line, message, ErrorKind::ScannerError));
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

		result.push(Token::new(TokenType::Eof, String::new(), None, self.line,));

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

enum Literal {
	String(String),
	Number(f64),
}

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
