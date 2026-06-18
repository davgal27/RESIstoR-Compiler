#![warn(clippy::pedantic)]// will remove if(when) this gets annoying, keeping only to act as a guide while I write bad rust
use super::tokens::TokenKind
use super::table::{State, Cat}
// Probably will use a batch token model: Scan the whole input and put into Vec<TokensFromLexer> jlox vs clox in Crafting interpreters. (jlox = batch, clox = stream)
pub struct Lexer {
	source: Vec<char>,
	table:
}
// 1) Implement a method which, when reading the characters, puts them in a Vector called ReadLexemes, and ReadTokens

// 2) Implement a method which skips whitespace when reading, and immediately calls next_token

// 3) End of file detector to stop calling next_token.... If EOF -> return token

/* 4) Helper function to know if consumed all source code :
private boolean isAtEnd() {
return current >= source.length();
} */

pub fn next_token() {
    // algorithm from Lexical Analysis notes

    // 1 - INITIALISATION
    let state = State.StartState;
    let lexeme = [];
    let stack = [bad];

    // 2 - SCANNNING LOOP
    while (state != State.ErrorState) {
        
    }


}