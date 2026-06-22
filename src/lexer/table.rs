
use super::tokens::TokenKind;
// Table: lexer will read table[curr_state][curr_input] = next state
// rows: States in the DFA
#[derive(Copy, Clone, PartialEq)] // might remove partialeq later, adding jic
pub enum State {
	//S0
	StartState,

	//Non accepting states: Require further chars, if halting here raise an error
	ConsumedMinus, // - on its own is invalid, needs > to form Arrow Token
	ConsumedPercent, // % on its own is invalid, needs letters to form Local token
	ConsumedFullstop, // 123. is invalid. needs more numbers to become float token 

	//Sa: Accepting states with option to loop or continue 
	ConsumedLocal, // start -> Consumedpercent-> letter, then letter|digit
	ConsumedIdent, // start -> Letter, then letter|digit 
	ConsumedDigit, // for integer literals
	ConsumedFloat, // start -1-> consumeddigit -.-> consumedfullstop -2-> consumedfloat
	ConsumedColon,
	
	//Sa Still, but are immediate acceptance (), with no option to loop or continue
	ConsumedPathSep, // : then : again. After this
	ConsumedArrow,// - then >
	ConsumedLBrack,
	ConsumedRBrack,
	ConsumedLCurly,
	ConsumedRCurly,
	ConsumedSemiColon,
	ConsumedGreaterThan, // start then >
	ConsumedLessThan,
	ConsumedEquals,
	ConsumedComma,

	// Se (error)
	ErrorState,
}
pub const STATE_COUNT: usize = 21; //consts in SCREAMING_SNAKE_CASE according to rust wiki

// comments were useful, but probably should make an accepting bool for the lexer for if(state in Sa)stack.clear()
pub fn is_accepting (state: State) -> bool {
	use State::*;
	match state {
		StartState | ConsumedMinus | ConsumedPercent | ConsumedFullstop | ErrorState => false,
		_ => true
	}
}

// Create the token from the accepting state 
pub fn state_to_token(state: State) -> Option<TokenKind> {
	use State::*;
	use TokenKind::*;
	Some(match state {
        ConsumedIdent => Identifier,
        ConsumedLocal => Local,
        ConsumedDigit => IntegerLiteral,
        ConsumedFloat => FloatLiteral,
        ConsumedColon => Colon,
        ConsumedPathSep => PathSep,
        ConsumedArrow => Arrow,
        ConsumedLBrack => LBracket,
        ConsumedRBrack => RBracket,
        ConsumedLCurly => LCurly,
        ConsumedRCurly => RCurly,
        ConsumedSemiColon=> SemiColon,
        ConsumedGreaterThan => GreaterThan,
        ConsumedLessThan=> LessThan,
        ConsumedEquals => Equals,
        ConsumedComma => Comma,
        _ => return None, // all non-accepting states
    })
}

// coloumns: Character Categories (edges in the automaton)
#[derive(Copy, Clone, PartialEq)]
pub enum Cat {
	Letter,
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
pub const CAT_COUNT: usize = 18; 

// map characters from input to a Category with match
pub fn char_to_cat(c: Option<char>) -> Cat { // return type: Category 
	use Cat::*; // useful for avoiding prefixes on every state
	match c {
		None => EndOfFile,
		Some(char) => match char {
			'A'..='Z' | 'a'..='z' => Letter,
			'_' => Letter, // underscore is part of variable_names
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

// Table construction ==================================================================
 
// build rows of states for each category, for every state
pub fn build_transition_table() -> [[State; CAT_COUNT]; STATE_COUNT] {
	use State::*;
	use Cat::*;
    // to avoid writing forever, let all cells be Error State 
	let mut t = [[ErrorState; CAT_COUNT]; STATE_COUNT];
	// now add rules : [currentstate][inputchar] = nextstate
	// IDENTIFIER: start -letter-> ConsumedIdent which loosp on letter|digit
    t[StartState as usize]			[Letter as usize] 			= ConsumedIdent;
    t[ConsumedIdent as usize]		[Letter as usize]			= ConsumedIdent;
    t[ConsumedIdent as usize]		[Digit as usize] 			= ConsumedIdent;

    // LOCAL start -> % -> letter -> loop on letter|digit
    t[StartState as usize]			[Percent as usize]        	= ConsumedPercent;
    t[ConsumedPercent as usize]		[Letter as usize]    		= ConsumedLocal;
    t[ConsumedLocal as usize]		[Letter as usize]      		= ConsumedLocal;
    t[ConsumedLocal as usize]		[Digit as usize]       		= ConsumedLocal;

    // INTEGER AND FLOAT
    t[StartState as usize]			[Digit as usize]          	= ConsumedDigit;
    t[ConsumedDigit as usize]		[Digit as usize]       		= ConsumedDigit;
    t[ConsumedDigit as usize]		[Fullstop as usize]    		= ConsumedFullstop;
    t[ConsumedFullstop as usize]	[Digit as usize]    		= ConsumedFloat;
    t[ConsumedFloat as usize]		[Digit as usize]       		= ConsumedFloat;

    // COLON AND PATHSEP: start -:-> Consumed Colon -:-> ConsumedPathSep
    t[StartState as usize]			[Colon as usize]          	= ConsumedColon;
    t[ConsumedColon as usize]		[Colon as usize]       		= ConsumedPathSep;

    // ARROW: start -'-'-> ConsumedMinus -'>'-> ConsumedArrow
    t[StartState as usize]			[Minus as usize]          	= ConsumedMinus;
    t[ConsumedMinus as usize]		[GreaterThan as usize] 		= ConsumedArrow;

    // PUNCTUATION:  immediate acceptance (), with no option to loop or continue
    t[StartState as usize]			[LBrack as usize]         	= ConsumedLBrack;
    t[StartState as usize]			[RBrack as usize]         	= ConsumedRBrack;
    t[StartState as usize]			[LCurly as usize]         	= ConsumedLCurly;
    t[StartState as usize]			[RCurly as usize]         	= ConsumedRCurly;
    t[StartState as usize]			[SemiColon as usize]      	= ConsumedSemiColon;
    t[StartState as usize]			[GreaterThan as usize]    	= ConsumedGreaterThan;
    t[StartState as usize]			[LessThan as usize]       	= ConsumedLessThan;
    t[StartState as usize]			[Equals as usize]         	= ConsumedEquals;
    t[StartState as usize]			[Comma as usize]          	= ConsumedComma;

    return t;
}
