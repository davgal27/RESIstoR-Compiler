use crate::lexer::lexer_core::produce_tokens;
use crate::parser::parser_core::Parser;
use crate::parser::ir::*;
use super::samples::*;

fn lex(input: &str) -> Result<String, String> {
    let tokens = produce_tokens(input)?;
    let mut kinds = Vec::new();

    for token in &tokens {
        kinds.push(token.token_kind.clone());
    }

    Ok(format!("{:?}", kinds))
}

fn parse(input: &str) -> Result<Program, String> {
    let tokens = produce_tokens(input)?;
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
        assert_eq!(lex(input).unwrap(), expected, "for input:\n{}", input);
    }
}

#[test]
fn lexer_returns_error_for_bad_characters() {
    let result = produce_tokens(INVALID_BAD_SYNTAX);

    assert!(result.is_err());

    let message = result.unwrap_err();
    assert!(message.contains("Lexical Error"));
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












// JIPPITY TESTS 

// #[test]
// fn lexer_accepts_literals() {
//     assert_eq!(
//         lex("true false null 123 12.34").unwrap(),
//         "[True, False, Null, IntegerLiteral, FloatLiteral, EndOfFile]"
//     );
// }

// #[test]
// fn lexer_accepts_all_basic_punctuation() {
//     assert_eq!(
//         lex("( ) { } -> : ; = , < > ::").unwrap(),
//         "[LBracket, RBracket, LCurly, RCurly, Arrow, Colon, SemiColon, Equals, Comma, LessThan, GreaterThan, PathSep, EndOfFile]"
//     );
// }

// #[test]
// fn lexer_rejects_bare_percent() {
//     let result = produce_tokens("%");
//     assert!(result.is_err());
// }

// #[test]
// fn lexer_rejects_bad_arrow() {
//     let result = produce_tokens("-");
//     assert!(result.is_err());
// }

// #[test]
// fn lexer_rejects_incomplete_float() {
//     let result = produce_tokens("123.");
//     assert!(result.is_err());
// }




// #[test]
// fn void_return_parses() {
//     let input = "
//     function Test::noop() -> void {
//         locals { }
//         entry bb0;
//         bb0:
//             return;
//     }
//     ";

//     let p = parse(input).expect("void function should parse");
//     let term = &p.function.blocks[0].term;

//     assert!(matches!(term, Term::Return(None)));
// }

// #[test]
// fn nested_pointer_type_parses() {
//     let input = "
//     function Test::ptrs(%p: ptr<ptr<i32>>) -> void {
//         locals { }
//         entry bb0;
//         bb0:
//             return;
//     }
//     ";

//     assert!(parse(input).is_ok());
// }

// #[test]
// fn cast_parses() {
//     let input = "
//     function Test::cast_example(%a: i32) -> i64 {
//         locals { %b : i64; }
//         entry bb0;
//         bb0:
//             %b = cast %a to i64;
//             return %b;
//     }
//     ";

//     let p = parse(input).expect("cast should parse");
//     assert!(matches!(p.function.blocks[0].stmt[0].rhs, Rhs::Cast(_, _)));
// }

// #[test]
// fn addr_of_parses() {
//     let input = "
//     function Test::addr(%a: i32) -> void {
//         locals { %p : ptr<i32>; }
//         entry bb0;
//         bb0:
//             %p = addr_of %a;
//             return;
//     }
//     ";

//     let p = parse(input).expect("addr_of should parse");
//     assert!(matches!(p.function.blocks[0].stmt[0].rhs, Rhs::Addr_of(_)));
// }

// #[test]
// fn load_and_store_parse() {
//     let input = "
//     function Test::load_store(%p: ptr<i32>, %v: i32) -> i32 {
//         locals { %x : i32; }
//         entry bb0;
//         bb0:
//             store %p, %v;
//             %x = load %p;
//             return %x;
//     }
//     ";

//     let p = parse(input).expect("load/store should parse");
//     let stmts = &p.function.blocks[0].stmt;

//     assert!(matches!(stmts[0].rhs, Rhs::Store(_, _)));
//     assert!(matches!(stmts[1].rhs, Rhs::Load(_)));
// }

// #[test]
// fn local_copy_parses() {
//     let input = "
//     function Test::copy(%a: i32) -> i32 {
//         locals { %x : i32; }
//         entry bb0;
//         bb0:
//             %x = %a;
//             return %x;
//     }
//     ";

//     let p = parse(input).expect("local copy should parse");
//     assert!(matches!(p.function.blocks[0].stmt[0].rhs, Rhs::Use(_)));
// }

// #[test]
// fn jump_parses() {
//     let input = "
//     function Test::jump_only() -> i32 {
//         locals { %x : i32; }
//         entry bb0;
//         bb0:
//             jump bb1;
//         bb1:
//             %x = const 1;
//             return %x;
//     }
//     ";

//     let p = parse(input).expect("jump should parse");
//     assert!(matches!(p.function.blocks[0].term, Term::Jump(_)));
// }