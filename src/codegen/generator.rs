use crate::parser::ir::*;

fn type_to_c(typealt: &Type) -> String {
    match typealt {
        Type::PrimType(PrimType::I32) => "int32_t".to_string(),
        Type::PrimType(PrimType::I64) => "int64_t".to_string(),
        Type::PrimType(PrimType::U32) => "uint32_t".to_string(),
        Type::PrimType(PrimType::F64) => "double".to_string(),
        Type::PrimType(PrimType::Bool) => "bool".to_string(),
        Type::Path(path) => pathsepto_(path),
        Type::Ptr(inner) => format!("{} *", type_to_c(inner)),
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
    let mut needs_stddef = false;
    let mut needs_custom = false;

    for typealt in types_to_check {
        let mut type_to_check = typealt;
        while let Type::Ptr(pointer_type) = type_to_check {
            type_to_check = pointer_type;
        }

        match type_to_check {
        	Type::PrimType(PrimType::I32) 
        	| Type::PrimType(PrimType::I64) 
        	| Type::PrimType(PrimType::U32) => needs_stdint = true,
        	
        	Type::PrimType(PrimType::Bool) => needs_stdbool = true,
        	_ => {} // no header 
        }
    }

    for block in &function.blocks {
    	for stmt in &block.stmt {
    		if let Rhs::Const(Literal::Null) = &stmt.rhs {
    			needs_stddef = true;
    		}
    	}
    }

    
    // check 1 for custom types directly
	if program.externtypes.is_empty() == false {
		needs_custom = true;
	}
	// check 2 : if theres a call, check if its not recursive. no? custom eneded
	for block in &program.function.blocks {
	    for stmt in &block.stmt {
	        if let Rhs::Call(path, _) = &stmt.rhs {
	            
	            if path != &program.function.path {
	            	needs_custom = true;
	            }
	        }
	    }
	}

    // print the includes
    let mut to_print = String::new();
    if needs_stdint == true {
    	to_print.push_str("#include <stdint.h>\n");
    }
    if needs_stdbool == true {
    	to_print.push_str("#include <stdbool.h>\n");
    }
    if needs_stddef == true {
        to_print.push_str("#include <stddef.h>\n");
    }
    if needs_custom == true {
    	to_print.push_str("#include \"custom_header.h\"\n");
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

	// lhs prefix 
	let lhs = match &stmt.local {
		Some(local) => format!("{}", local.ident.string),
		None => format!(""), // case for store or call   
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

		Rhs::Cast(local, typealt) => format!("{lhs} = ({}){};",type_to_c(typealt), local.ident.string),

		Rhs::Un(unop, local) => {
			match unop {
				UnOp::Neg => format!("{lhs} = -{};", local.ident.string),
				UnOp::Not => format!("{lhs} = !{};", local.ident.string),
			}
		}

		Rhs::Bin(binop, local_left, local_right) =>{
			match binop {
				BinOp::Add => format!("{lhs} = {} + {};", local_left.ident.string, local_right.ident.string),
				BinOp::Sub => format!("{lhs} = {} - {};", local_left.ident.string, local_right.ident.string),
				BinOp::Mul => format!("{lhs} = {} * {};", local_left.ident.string, local_right.ident.string),
				BinOp::Div => format!("{lhs} = {} / {};", local_left.ident.string, local_right.ident.string),
				BinOp::Mod => format!("{lhs} = {} % {};", local_left.ident.string, local_right.ident.string),
				BinOp::Eq => format!("{lhs} = {} == {};", local_left.ident.string, local_right.ident.string),
				BinOp::Ne => format!("{lhs} = {} != {};", local_left.ident.string, local_right.ident.string),
				BinOp::Lt => format!("{lhs} = {} < {};", local_left.ident.string, local_right.ident.string),
				BinOp::Le => format!("{lhs} = {} <= {};", local_left.ident.string, local_right.ident.string),
				BinOp::Gt => format!("{lhs} = {} > {};", local_left.ident.string, local_right.ident.string),
				BinOp::Ge => format!("{lhs} = {} >= {};", local_left.ident.string, local_right.ident.string),
				BinOp::And => format!("{lhs} = {} && {};", local_left.ident.string, local_right.ident.string),
				BinOp::Or => format!("{lhs} = {} || {};", local_left.ident.string, local_right.ident.string),
			}
		}
		
		Rhs::Call(path, args) => {
			// same idea as the path sep (i might be great at complicating things)
		    let mut catenated_args = String::new();
		    let mut lone_args = Vec::new();
		    if let Some(args) = args {
		        for local in &args.locals {
		            lone_args.push(local);
		        }
		    }
		    for local in lone_args {
		        if catenated_args.is_empty() == false {
		            catenated_args.push_str(", ");
		        }
		        catenated_args.push_str(&local.ident.string);
		    }
		    // call may rarely have an lhs 
		    match &stmt.local {
		    	Some(_) => format!("{lhs} = {}({});", pathsepto_(path), catenated_args),
		    	None => format!("{}({});", pathsepto_(path), catenated_args),
		    }
		}
		
		// ptr operations ========================
		
		Rhs::Addr_of(local) => format!("{lhs} = &{};", local.ident.string),
		
		Rhs::Member_ptr(local, ident) => format!("{lhs} = &{}->{};", local.ident.string, ident.string),

		Rhs::Load(local) => format!("{lhs} = *{};", local.ident.string),

		Rhs::Store(local_one, local_two) => format!("*{} = {};", local_one.ident.string, local_two.ident.string)
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