#![warn(clippy::pedantic)]// will remove if(when) this gets annoying, keeping only to act as a guide while I write bad rust
use std::fmt; // for printing nicelsy
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
	pub token_kind: TokenKind,
	pub token_attribute: TokenAttribute,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenAttribute {
	// Location
	pub line: usize,
	pub col: usize,
	// lexeme: word in a program
	pub lexeme: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
	// Keywords: only one possible lexeme
	Function,
	Locals,
	Entry,
	Const,
	Cast,
	Un,
	Bin,
	Addr_of,
	Member_ptr,
	Load,
	Store,
	Call,
	Jump,
	CJump,
	Return,
	Void,
	Ptr,
	True,
	False,
	Null,
	To,
	Extern,
	Type,
	// Primitive type names (still keywords)
	Bool,
	I32,
	I64,
	U32,
	F64,
	
	// Identifiers
	Identifier,
	
	// Local names
	Local, // % Ident
	
	// Labels
	Label, // bb N 
		
	// Literals
	IntegerLiteral,
	FloatLiteral,
	
	// Punctuation
	LBracket, // (
	RBracket, // )
	LCurly, // {
	RCurly, // }
	Arrow,
	Colon,
	SemiColon,
	Equals,
	Comma,
	LessThan, // <
	GreaterThan, //>
	
	// Path Separators
	PathSep,

	//eof
	EndOfFile,
	
}

// =============DISPLAY FOR TESTING=============== 
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token Kind: {:<12} Lexeme: {:<25} Line: {:<3} Col: {:<3}",
            format!("{:?}", self.token_kind),
            self.token_attribute.lexeme,
            self.token_attribute.line,
            self.token_attribute.col
        )
    }
}