#![warn(clippy::pedantic)]// will remove if(when) this gets annoying, keeping only to act as a guide while I write bad rust

pub struct Token {
	pub token_kind: TokenKind,
	pub token_attribute: TokenAttribute,
}

pub struct TokenAttribute {
	// Location
	pub line: usize,
	pub col: usize,
	// lexeme
	pub lexeme: String,
}

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

