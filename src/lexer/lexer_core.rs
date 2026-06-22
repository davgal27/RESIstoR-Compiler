
use super::tokens::{TokenKind, TokenAttribute, Token};
use super::table::{State, Cat, STATE_COUNT, CAT_COUNT, is_accepting, char_to_cat, build_transition_table, state_to_token,};
// will use a batch token model: Scan the whole input and put into Vec<TokensFromLexer> jlox from CraftingInterpreters
pub fn produce_token(input: &str) -> Vec<Token> {

    let mut pos_in_source: usize = 0; 
    let t_table = build_transition_table();
    let mut source: Vec<char> = Vec::new();
    let mut tokens: Vec<Token> = Vec::new();
    let mut line: usize = 1;
    let mut col: usize = 1;
    
    // for every character in the input
    for char in input.chars() {
        source.push(char);
    }

    // main lexar loop: continue this until we reach the eof
    let mut eof_detected: bool = false;

    while eof_detected == false {

        // skip whitespace
        while pos_in_source < source.len() && matches!(source[pos_in_source], ' ' | '\t' | '\r' | '\n') {
            // detect new line
            if source[pos_in_source] == '\n' {
                line += 1;
                col = 1; // reset coloumn back to the start
            } else {
                col += 1; 
            }

            pos_in_source +=1;
        }

        //detect eof. if not eof, get token
        if pos_in_source >= source.len() {
            eof_detected = true;
            tokens.push(Token {
                token_kind: TokenKind::EndOfFile,
                token_attribute: TokenAttribute {
                    line,
                    col, 
                    lexeme: String::new() 
                },
            });
        } else {
            // remember where this token starts so that attribute points at first char
            let start_col = col;
            let token = next_token(&source, &mut pos_in_source, &t_table);

            match token {
                // if token is found (has (some))
                Some((found_kind, found_lexeme)) => {
                    // move col the length of the word
                    col += found_lexeme.chars().count();
                    let kind = which_kind(found_kind, &found_lexeme);
                    tokens.push(Token {
                        token_kind: kind,
                        token_attribute: TokenAttribute {
                            line,
                            col: start_col,
                            lexeme: found_lexeme, 
                        },
                    });
                }
                // lexical error: next_token stepped over an offendngi token
                None => {
                    let offending_char = source[pos_in_source -1];
                    eprintln!("Lexical error: Unexpected Character '{offending_char}' at line {line}, col {start_col}");
                    col +=1;
                }
            }

        }


    }
    return tokens;

}

//reserved words listed in the grammar are not valid Idents
fn classify_lexeme(lexeme: &str) -> TokenKind {
    use TokenKind::*;
    match lexeme {
        "function" => Function,
        "locals" => Locals, 
        "entry" => Entry,
        "const" => Const, 
        "cast" => Cast, 
        "un" => Un, 
        "bin" => Bin,
        "addr_of" => Addr_of, 
        "member_ptr" => Member_ptr,
        "load" => Load, 
        "store" => Store, 
        "call" => Call,
        "jump" => Jump, 
        "cjump" => CJump, 
        "return" => Return,
        "void" => Void, 
        "ptr" => Ptr, 
        "true" => True, 
        "false" => False,
        "null" => Null, 
        "to" => To, 
        "extern" => Extern, 
        "type" => Type,
        "bool" => Bool, 
        "i32" => I32, 
        "i64" => I64, 
        "u32" => U32, 
        "f64" => F64,
        _=> Identifier,
        // will check for labels in the parser 
    }
}

// identifier from DFA can be a keyword or a label too, so we need to check forthat
fn which_kind(kind: TokenKind, lexeme: &str) -> TokenKind {
    // if not identifier, leave as is
    if kind != TokenKind::Identifier {
        return kind; 
    }
    // is it a keyword?
    if classify_lexeme(lexeme) != TokenKind::Identifier {
        return classify_lexeme(lexeme);
    }
    // is it a label?
    if is_label(lexeme) {
        return TokenKind::Label;
    }
    // if none of the above, its an identifier 
    TokenKind::Identifier
}

fn is_label(lexeme: &str) -> bool {
    // does the lexeme start with bb? no? not a label
    let Some(rest_of_lexeme) = lexeme.strip_prefix("bb") else {
        return false;
    };

    // lexeme starts with bb. Is the part after bb all numbers? 
    if rest_of_lexeme.is_empty() == false 
        && rest_of_lexeme.chars().all(|c| c.is_ascii_digit()) == true {
        return true;
    }
    false
}


// take in the source chars, the position in the 
fn next_token(source: &Vec<char>, pos: &mut usize, t_table: &[[State; CAT_COUNT]; STATE_COUNT]) -> Option<(TokenKind, String)>{
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
        let lexeme_string: String = lexeme.iter().collect();
        match state_to_token(state) {
            Some(found_kind) => return Some((found_kind, lexeme_string)),
            None => return None,
        }
    } else {

        *pos += 1;
        return None;

    }
}