#![allow(dead_code)]
pub const EXAMPLE_ASSIGNMENT_1: &str =
"
function Math::abs_diff(%a: i32, %b: i32) -> i32 {
	locals {
		%zero	: i32;
		%d		: i32;
		%is_neg : bool;
		%r		: i32;
	}
	entry bb0;
	bb0:
		%zero   = const 0;
		%d  	= bin sub %a, %b;
		%is_neg = bin lt %d, %zero;
	    cjump %is_neg, bb1, bb2;

	bb1:
		%r = un neg %d;
		return %r;

	bb2:
		return %d;
}
";

pub const EXAMPLE_ASSIGNMENT_2: &str =
"
function Client::sum_abs_diffs(%a: i32, %b: i32,
                               %c: i32, %d: i32) -> i32 {
    locals {
        %dx : i32;
        %dy : i32;
        %r  : i32;
    }
    entry bb0;

    bb0:
        %dx = call Math::abs_diff(%a, %b);
        %dy = call Math::abs_diff(%c, %d);
        %r  = bin add %dx, %dy;
        return %r;
}

"
;

pub const EXAMPLE_ASSIGNMENT_3: &str =
"
extern type Custom::Struct::Point {
    x : i32;
    y : i32;
}
function Geom::point_x(%p: ptr<Custom::Struct::Point>) -> i32 {
	locals {
		%fp : ptr<i32>;
        %v  : i32;
    }
    entry bb0;

    bb0:
        %fp = member_ptr %p, x;
        %v = load %fp;
        return %v;
}

";


// valids ========================================================
pub const CONST_ASSIGNMENT: &str = 
"
function Test::const_only(%a: i32) -> i32 {
    locals { %x : i32; }
    entry bb0;
    bb0:
        %x = const 42;
        return %x;
}
";

pub const BIN_OP: &str = 
"
function Test::add_numbers(%a: i32, %b: i32) -> i32 {
    locals { %r : i32; }
    entry bb0;
    bb0:
        %r = bin add %a, %b;
        return %r;
}
";

pub const FUNCTION_CALL: &str = 
"
function Test::funccall(%a: i32, %b: i32) -> i32 {
    locals { %r : i32; }
    entry bb0;
    bb0:
        %r = call Math::abs_diff(%a, %b);
        return %r;
}
";

pub const MEMBER_PTR: &str = 
"
function Test::member(%p: ptr<Custom::Struct::Point>) -> i32 {
    locals { %x : ptr<i32>; }
    entry bb0;
    bb0:
        %x = member_ptr %p, x;
        return %x;
}
";

pub const CJUMP: &str = 
"
function Test::branch(%a: i32) -> i32 {
    locals { %r : i32; }
    entry bb0;
    bb0:
        cjump %a, bb1, bb2;
    bb1:
        return %a;
    bb2:
        return %a;
}
";

// invalids ================================================================
pub const INVALID_NO_SEMICOLON: &str = 
// no ; on i32 and const 0
"
function Math::abs_diff(%a: i32, %b: i32) -> i32 {
    locals {
        %x : i32
    }
    entry bb0;
    bb0:
        %x = const 0
        return %x;
}
";

pub const INVALID_MISSING_ENTRY: &str = 
"
function Math::abs_diff(%a: i32, %b: i32) -> i32 {
    locals {
        %x : i32;
    }

    bb0:
        %x = const 0;
        return %x;
}
";

pub const INVALID_CHARACTERS : &str = 
"
bla bla nonsense !!! @function $$$
"
;

// semantic invalids ======================================================

pub const SEMANTIC_UNDECLARED_LOCAL: &str =
"
function Test::undeclared_local(%a: i32) -> i32 {
    locals { }
    entry bb0;
    bb0:
        %x = const 1;
        return %a;
}
";

pub const SEMANTIC_DUPLICATE_LABELS: &str =
"
function Test::duplicate_labels(%a: i32) -> i32 {
    locals { }
    entry bb0;
    bb0:
        return %a;
    bb0:
        return %a;
}
";

pub const SEMANTIC_MISSING_ENTRY_BLOCK: &str =
"
function Test::missing_entry_block(%a: i32) -> i32 {
    locals { }
    entry bb9;
    bb0:
        return %a;
}
";

pub const SEMANTIC_MISSING_JUMP_TARGET: &str =
"
function Test::missing_jump_target() -> void {
    locals { }
    entry bb0;
    bb0:
        jump bb9;
}
";

pub const SEMANTIC_CJUMP_NON_BOOL: &str =
"
function Test::bad_cjump(%a: i32) -> i32 {
    locals { }
    entry bb0;
    bb0:
        cjump %a, bb1, bb2;
    bb1:
        return %a;
    bb2:
        return %a;
}
";

pub const SEMANTIC_RETURN_TYPE_MISMATCH: &str =
"
function Test::bad_return(%a: i32) -> i32 {
    locals { %b : bool; }
    entry bb0;
    bb0:
        %b = const true;
        return %b;
}
";

pub const SEMANTIC_BAD_BIN_OPERANDS: &str =
"
function Test::bad_bin(%a: i32, %b: bool) -> i32 {
    locals { %r : i32; }
    entry bb0;
    bb0:
        %r = bin add %a, %b;
        return %r;
}
";

pub const SEMANTIC_LOAD_FROM_NON_POINTER: &str =
"
function Test::bad_load(%p: i32) -> i32 {
    locals { %x : i32; }
    entry bb0;
    bb0:
        %x = load %p;
        return %x;
}
";

pub const SEMANTIC_STORE_WRONG_VALUE_TYPE: &str =
"
function Test::bad_store(%p: ptr<i32>, %b: bool) -> void {
    locals { }
    entry bb0;
    bb0:
        store %p, %b;
        return;
}
";

pub const SEMANTIC_MEMBER_PTR_MISSING_FIELD: &str =
"
extern type Custom::Struct::Point {
    x : i32;
}
function Test::bad_member_ptr(%p: ptr<Custom::Struct::Point>) -> void {
    locals { %fp : ptr<i32>; }
    entry bb0;
    bb0:
        %fp = member_ptr %p, y;
        return;
}
";

pub const SEMANTIC_UNDECLARED_CUSTOM_TYPE: &str =
"
function Test::undeclared_custom_type() -> void {
    locals { %p : ptr<Custom::Nope>; }
    entry bb0;
    bb0:
        return;
}
";