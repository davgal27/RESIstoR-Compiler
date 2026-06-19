#![warn(clippy::pedantic)]// will remove if(when) this gets annoying, keeping only to act as a guide while I write bad rust
use super::tokens::TokenKind;
use super::table::{State, Cat, STATE_COUNT, CAT_COUNT, is_accepting, char_to_cat, build_transition_table, state_to_token,};
// will use a batch token model: Scan the whole input and put into Vec<TokensFromLexer> jlox from CraftingInterpreters
pub fn produce_token(input: &str) ->Vec<(TokenKind)> {

    let mut pos_in_source: usize = 0; 
    let t_table = build_transition_table();
    let mut source: Vec<char> = Vec::new();
    let mut tokens: Vec<TokenKind> = Vec::new();
    
    // for every character in the input
    for char in input.chars() {
        source.push(char);
    }

    // main lexar loop: continue this until we reach the eof
    let mut eof_detected: bool = false;

    while eof_detected == false {

        // skip whitespace
        while pos_in_source < source.len() && matches!(source[pos_in_source], ' ' | '\t' | '\r' | '\n') {
            pos_in_source +=1;
        }

        //detect eof. if not eof, get token
        if pos_in_source >= source.len() {
            eof_detected = true;
            tokens.push(TokenKind::EndOfFile);
        } else {
            let token = next_token(&source, &mut pos_in_source, &t_table);

            match token {
                // if token is found (has (some))
                Some(foundtoken) => tokens.push(foundtoken),
                None => {}
            }

        }


    }
    return tokens;

} 

// take in the source chars, the position in the 
pub fn next_token(source: &Vec<char>, pos: &mut usize, t_table: &[[State; CAT_COUNT]; STATE_COUNT]) -> Option<TokenKind>{
    // algorithm from Lexical Analysis notes

    // 1 - INITIALISATION
    let mut current_char : char;
    let mut cat: Cat;
    let mut state = State::StartState;
    let mut lexeme = Vec::new();
    let mut stack = Vec::new(); // init empty stack
    // bad state will just be an empty stack 

    // 2 - SCANNNING LOOP: while not in Se or at eof
    while state != State::ErrorState && (*pos >= source.len()) == false {
        current_char = source[*pos];
        *pos += 1;
        lexeme.push(current_char);
        if is_accepting(state){
            stack.clear();
        }
        stack.push(state);
        cat = char_to_cat(Some(current_char));
        state = t_table[state as usize][cat as usize];
    }

    // 3 - ROLLBACK LOOP
    while is_accepting(state) == false && stack.is_empty() == false {
        state = stack.pop().unwrap();
        lexeme.pop();
        *pos -=1;
    }

    // 4 - REPORT RESULT
    if is_accepting(state) == true {
        return state_to_token(state);
    } else {
        *pos += 1;
        return None;
    }
}