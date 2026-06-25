use crate::parser::ir::*;

// 1) prim types to C types 5-arm table.
fn prim_to_c (prim: &PrimType) -> String { 
	match prim {
		PrimType::Bool => bool,
		PrimType::I32 => int32_t,
		PrimType::I64 => int64_t,
		PrimType::U32 => uint32_t,
		PrimType::F64 => double,
	}
}
// 2) #includes
// emit nothing for extern type blocks.
// in addition to any header that declares the custom struct types and
// external helpers used by the program.
fn include_includes(program: &Program) -> String {

	// reuse most logic from analyser (custom types declared fn)
	let mut types_to_check: Vec<&Type> = Vec::new();
	let function = &program.function;

	// fields
    for extern_type in &program.externtypes {
        for field in &extern_type.fields {
            types_to_check.push(&field.typealt);
        }
    }

    // parameters
    if let Some(params) = &function.params {
        for param in &params.params {
            types_to_check.push(&param.typealt);
        }
    }

    // rettype
    match &function.rettype {
        RetType::typealt(typealt) => {
            types_to_check.push(typealt);
        }

        RetType::Void => {}
    }
    // locals
    for (_, typealt) in &function.locals {
        types_to_check.push(typealt);
    }

    // cast op
    for block in &function.blocks {
        for stmt in &block.stmt {
            if let Rhs::Cast(_, target_type) = &stmt.rhs {
                types_to_check.push(target_type);
            }
        }
    }

    // now check every collected type:
    // the custom type might be wrapped in one or more ptr layers,
    // so unwrap until we reach the type the ptr points at
    let mut needs_stdint = false;
    let mut needs_stdbool = false; 
    for typealt in types_to_check {
        let mut type_to_check = typealt;
        while let Type::Ptr(pointer_type) = type_to_check {
            type_to_check = pointer_type;
        }

        match type_to_check {
        	PrimType::I32 | PrimTpye::I64 | PrimType::U32 => needs_stdint = true,
        	PrimType::Bool => needs_stdbool = true,
        	_ => {} // no header 
        }
    }

    // print the includes
    let mut to_print = String::new();
    if needs_stdint == true {
    	to_print.push_str('#include <stdint.h>\n");
    }
    if needs_stdbool == true {
    	to_print.push_str('#include <stdbool.h>\n");
    }
    to_print
}

// 3) :: -> _
fn pathsepto_(path : &Path) -> String {
	let mut catenated_idents = String::new();
	let mut lone_idents = Vec::new();

	for ident in &path.ident {
		lone_idents.push(ident);
	}

	for ident in lone_idents {
		if catenated_idents.is_empty() == false {
			catenated_idents.push('_');
		}
		catenated_idents.push_str(&ident.string);
	}
	catenated_idents
}
// 4) pointer operations become ordinary C address. field access. dereference, and assignment operations + any other rhs conversion to C
// one match over stmt.rhs, all 10 arms. dest prefix "x = " (or "" for store).
//   use->y  const->lit  cast->(T)y  un->-y/!y  bin->a op b
//   addr_of->&y  load->*p  store->*p=v  member_ptr->&p->f  call->F(args)
// helpers: binop_to_c, literal_to_c (Null->NULL, float may need ".0").

fn convert_ops(stmt: &Stmt) -> String {
	for stmt in &block.stmt {

		// lhs prefix 
		let lhs = match &stmt.local {
			Some(local) => format!("{}", local.ident.string),
			None => format!(""), // case for call and store 
		};

		match &stmt.rhs {

			Rhs::Use(local) => format!("{lhs} = {};", local.ident.string),

			Rhs::Const(literal) => {
				let value = match literal {
					Literal::IntegerLiteral(int) => int.to_string(),
					Literal::FloatLiteral(float) => float.to_string(),
					Literal::True => "true".to_string(),
					Literal::False => "false".to_string(),
					Literal::Null => "NULL".to_string(),
				};

				format!("{lhs} = {value};")

			}

			Rhs::Cast(local, typealt) => format!("{lhs} = ({typealt}){}", local.ident.string),

			Rhs::Un(unop, local) => 

			Rhs::Bin(binop, local_left, local_right) =>
			
			Rhs::Call(path, args) =>
			
			// ptr operations ========================
			
			Rhs::Addr_of(local) =>
			
			Rhs::Member_ptr(local, ident) =>

			Rhs::Load(local) =>

			Rhs::Store(local_one, local_two) =>

		}

		}
	}
}


// ============= label based lowering ===========

// 5) function shell (generate_c driver)
// includes -> "ret name(params) {" -> local decls -> "goto entry;" -> blocks -> "}"
// ret/params/decls all via type_to_c. no symbol table, no Result.

// 6) basic blocks
// per block: "bbN:" -> statements (section 4) -> terminator.
//   jump->goto  cjump->if(c) goto t; else goto f;  return->return x;/return;
// label_to_c must match between labels and gotos.