#![warn(clippy::pedantic)]// will remove if(when) this gets annoying, keeping only to act as a guide while I write bad rust
// Table: lexer will read table[row][column] at state to go to next_state

pub enum Input { //character stream
	Char(char),
	EndOfFile,
}
// rows: States in the DFA
pub enum State {
// no idea how to fill these in yet
	
}

// coloumns: Character Categories
pub enum Cat {
	Letter,
	Underscore,
	Digit,
	LBrack,
	RBrack,
	LCurly,
	RCurly,
	Percent,
	Colon,
	SemiColon,
	Whitespace,
	Minus,
	GreaterThan,
	LessThan,
	Equals,
	Comma,
	Fullstop,
	EndOfFile, 
	Invalid, //any character not from the above is not defined by grammar 
}

// map characters from input to a Category with match
pub fn char_to_cat(input: Input) -> Cat { // return type: Category 
	use Cat::*; // useful for avoiding prefixes on every state
	match input {
		Input::EndOfFile => EndOfFile,
		Input::Char(char) => match char {
			'A'..='Z' | 'a'..='z' => Letter,
			'_' => Underscore,
			'0'..='9' => Digit,
			'(' => LBrack,
			')' => RBrack,
			'{' => LCurly,
			'}' => RCurly,
			'%' => Percent,
			':' => Colon,
			';' => SemiColon,
			' ' | '\t' | '\r' | '\n' => Whitespace,
			'-' => Minus,
			'>' => GreaterThan,
			'<' => LessThan,
			'=' => Equals,
			',' => Comma,
			'.' => Fullstop,
			_ => Invalid,	
		},
	}
}
		
