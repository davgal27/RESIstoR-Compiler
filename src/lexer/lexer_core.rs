#![warn(clippy::pedantic)]// will remove if(when) this gets annoying, keeping only to act as a guide while I write bad rust
use crate::tokens::TokenKind

pub struct Lexer {
	source: Vec<char>,
	start: usize,
	pos: usize,
	line: usize,
	col: usize,
	table:
}


Setup
new() is called
    -> source = ['%','d',' ','=', 'b','i','n']
    -> start = 0, current = 0, line = 1, col = 1
    -> table = build_transition_table()
tokenise() is called — outer loop
loop:
    skip_whitespace()   // consume any leading whitespace
    call next_token()
    push result to Vec<Token>
    if EOF -> break
First call to next_token() — scanning %d
Section 1 — Initialisation:
stack = [ErrorState]    // sentinel at bottom
lexeme = ""
state = StartState
start = 0
snapshot line/col
Section 2 — Scan loop:
peek() -> '%'  -> Input::Char('%')
char_to_cat('%') -> Cat::Percent
TRANSITION_TABLE[StartState][Percent] -> ConsumedPercent
state is NOT accepting -> don't clear stack
stack.push(ConsumedPercent, pos=1)
advance() -> current=1, col=2
lexeme = "%"

peek() -> 'd' -> Input::Char('d')
char_to_cat('d') -> Cat::Letter
TRANSITION_TABLE[ConsumedPercent][Letter] -> ConsumedLocal
state is NOT accepting -> don't clear stack
stack.push(ConsumedLocal, pos=2)
advance() -> current=2, col=3
lexeme = "%d"

peek() -> ' ' -> Input::Char(' ')
char_to_cat(' ') -> Cat::Whitespace
TRANSITION_TABLE[ConsumedLocal][Whitespace] -> ErrorState
ErrorState -> break out of scan loop
Section 3 — Rollback loop:
current state = ConsumedLocal
state_to_token(ConsumedLocal) = Some(Local) -> accepting, stop rollback immediately
no rollback needed
Section 4 — Final:
state_to_token(ConsumedLocal) -> Some(Local)
lexeme = source[0..2] = "%d"
reclassify(Local, "%d") -> Local (no change needed)
make_token(Local, "%d", line=1, col=1)
return Ok(Token)
Back in tokenise() — push Token(Local, "%d") to Vec
Second call to next_token() — scanning  
Section 1:
skip_whitespace() -> consumes ' ', current=3, col=4
stack = [ErrorState]
state = StartState
start = 3
Section 2 — Scan loop:
peek() -> '=' -> Input::Char('=')
char_to_cat('=') -> Cat::Equals
TRANSITION_TABLE[StartState][Equals] -> ConsumedEquals
state IS accepting -> stack.clear(), stack = [ErrorState]
stack.push(ConsumedEquals, pos=4)
advance() -> current=4, col=5
lexeme = "="

peek() -> ' ' -> Input::Char(' ')
char_to_cat(' ') -> Cat::Whitespace
TRANSITION_TABLE[ConsumedEquals][Whitespace] -> ErrorState
break
Section 3 — Rollback:
state = ConsumedEquals
state_to_token(ConsumedEquals) = Some(Equals) -> accepting, no rollback
Section 4 — Final:
lexeme = "="
make_token(Equals, "=", line=1, col=4)
return Ok(Token)
Third call to next_token() — scanning bin
Section 1:
skip_whitespace() -> consumes ' ', current=5
stack = [ErrorState]
state = StartState
start = 5
Section 2 — Scan loop:
peek() -> 'b' -> Letter
TRANSITION_TABLE[StartState][Letter] -> ConsumedIdent
NOT accepting -> don't clear
stack.push(ConsumedIdent, pos=6)
advance(), lexeme = "b"

peek() -> 'i' -> Letter
TRANSITION_TABLE[ConsumedIdent][Letter] -> ConsumedIdent
IS accepting -> stack.clear(), stack=[ErrorState]
stack.push(ConsumedIdent, pos=7)
advance(), lexeme = "bi"

peek() -> 'n' -> Letter
TRANSITION_TABLE[ConsumedIdent][Letter] -> ConsumedIdent
IS accepting -> stack.clear()
stack.push(ConsumedIdent, pos=8)
advance(), lexeme = "bin"

peek() -> EOF -> Input::EndOfFile
char_to_cat(EOF) -> Cat::EndOfFile
TRANSITION_TABLE[ConsumedIdent][EndOfFile] -> ErrorState
break
Section 3 — Rollback:
state = ConsumedIdent
state_to_token(ConsumedIdent) = Some(Identifier) -> accepting, no rollback
Section 4 — Final:
lexeme = "bin"
state_to_token(ConsumedIdent) -> Some(Identifier)
reclassify(Identifier, "bin") -> Bin   // keyword lookup
make_token(Bin, "bin", line=1, col=6)
return Ok(Token)
tokenise() sees EOF, breaks, returns:
Vec<Token> = [
    Token(Local,      "%d",  line=1, col=1),
    Token(Equals,     "=",   line=1, col=4),
    Token(Bin,        "bin", line=1, col=6),
]
Every function called, in order:
tokenise()
    skip_whitespace()
    next_token()
        peek()
        char_to_cat()
        TRANSITION_TABLE[state][cat]
        state_to_token()     // to check if accepting (stack clear)
        advance()
        -- rollback if needed --
        state_to_token()     // final emit
        reclassify()
        make_token()


//////// OR 

next_token + 
while not EOF:
    skip whitespace

    state ← start
    last_accept_state ← none
    last_accept_pos ← none
    start_pos ← current_pos

    while M can transition on next character:
        read a
        state ← δ(state, a)

        if state is accepting:
            last_accept_state ← state
            last_accept_pos ← current_pos

    if last_accept_state == none:
        error("lexical error at start_pos")

    retract input to last_accept_pos
    emit token(type(last_accept_state), lexeme[start_pos:last_accept_pos])


IMPLEMENT traverse_table function ,
