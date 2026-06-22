use crate::lexer::lexer_core::produce_token;
use crate::parser::parser_core::Parser;
use crate::parser::ir::*;
use super::samples::*;

fn lex(input: &str) -> String {
    let tokens = produce_token(input);
    let mut kinds = Vec::new();
    for token in &tokens {
        kinds.push(token.token_kind.clone());
    }
    format!("{:?}", kinds)
}
 

fn parse(input: &str) -> Result<Program, String> {
    let tokens = produce_token(input);
    let mut parser = Parser::new(tokens);
    parser.parse_program()
}


// ======================LEXER================================

pub const QUICK_LEXER: &[(&str, &str)] = &[
    ("%x = const 0;",
        "[Local, Equals, Const, IntegerLiteral, SemiColon, EndOfFile]"),

    ("ptr<i32> Custom::Struct::Point bb3:",
        "[Ptr, LessThan, I32, GreaterThan, Identifier, PathSep, Identifier, PathSep, Identifier, Label, Colon, EndOfFile]"),
];

#[test]
fn lexer_produces_correct_tokens() {
    for &(input, expected) in QUICK_LEXER {
        assert_eq!(lex(input), expected, "for input:\n{}", input);
    }
}
 

// lexer should not crash only should return errors
#[test]
fn lexer_doesnt_crash_with_invalid_input() {
    let _ = produce_token(INVALID_NO_SEMICOLON);
    let _ = produce_token(INVALID_MISSING_ENTRY);
    let _ = produce_token(INVALID_BAD_SYNTAX);
}

// =========================PARSER=============================
#[test]
fn parser_passes_assignment_examples() {
    assert!(parse(EXAMPLE_ASSIGNMENT_1).is_ok());
    assert!(parse(EXAMPLE_ASSIGNMENT_2).is_ok());
    assert!(parse(EXAMPLE_ASSIGNMENT_3).is_ok());
}

#[test]
fn const_assignment_parses() {
    let p = parse(CONST_ASSIGNMENT).expect("program should parse");
    let block = &p.function.blocks[0];

    assert!(!block.stmt.is_empty());

    let stmt = &block.stmt[0];

    match &stmt.rhs {
        Rhs::Const(_) => {}
        _ => panic!("expected const rhs"),
    }

    // ensure termination exists and is valid
    match &block.term {
        Term::Return(_) => {}
        _ => panic!("expected return terminator"),
    }
}

#[test]
fn binary_operation_parses() {
    let p = parse(BIN_OP).expect("program should parse");
    let stmts = &p.function.blocks[0].stmt;

    assert!(matches!(stmts[0].rhs, Rhs::Bin(_, _, _)));
}

#[test]
fn function_call_parses() {
    let p = parse(FUNCTION_CALL).expect("program should parse");
    let stmts = &p.function.blocks[0].stmt;

    assert!(matches!(stmts[0].rhs, Rhs::Call(_, _)));
}

#[test]
fn member_ptr_parses() {
    let p = parse(MEMBER_PTR).expect("program should parse");
    let stmts = &p.function.blocks[0].stmt;

    assert!(matches!(stmts[0].rhs, Rhs::Member_ptr(_, _)));
}

#[test]
fn cjump_parses() {
    let p = parse(CJUMP).expect("program should parse");
    let term = &p.function.blocks[0].term;

    assert!(matches!(term, Term::CJump(_, _, _)));
}

// invalids =========================================
#[test]
fn no_semicolon_doesnt_parse() {
    let result = parse(INVALID_NO_SEMICOLON);
    assert!(result.is_err());
}

#[test]
fn missing_entry_doesnt_parse() {
    let result = parse(INVALID_MISSING_ENTRY);
    assert!(result.is_err());
}


#[test]
fn bad_syntax_doesnt_parse() {
    let result = parse(INVALID_BAD_SYNTAX);
    assert!(result.is_err());
}