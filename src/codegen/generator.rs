use crate::parser::ir::*;

pub fn generate_c(program: &Program, header_name: &str, unreachable_blocks: &Vec<Block>) -> String {
	let function = &program.function; 
	let mut c = String::new();// holds whole program

	// INCLUDES============================================
	let includes = include_includes(program, header_name);

	// FUNCTION SIGNATURE: rettype, name (params) ===========
	let rettype = match &function.rettype{
		RetType::Void => format!("void"),
		RetType::typealt(t) => type_to_c(t),
	};

	let function_name = pathsepto_(&function.path);
	
	// never have I ever regretted how i strucutred the IR more than right now 
	let mut params_string = String::new();
	if let Some(params) = &function.params {
		for p in &params.params {
			if params_string.is_empty() == false {
				params_string.push_str(", ");
			}
			match &p.typealt {
			    Type::Ptr(_) => {
			        params_string.push_str(&format!("{}{}", type_to_c(&p.typealt), p.local.ident.string));
			    }
			    _ => {
			        params_string.push_str(&format!("{} {}", type_to_c(&p.typealt), p.local.ident.string));
			    }
			}
		}
	}
	if params_string.is_empty() == true {
		params_string.push_str("void");
	}

	// LOCALS =================================================== 
	//(I couldnt get the grouping of locals to work when pointers got in the mix)
	let mut locals_string = String::new();
	for (local, typealt) in &function.locals {
		match typealt {
	    Type::Ptr(_) => {
		        locals_string.push_str(&format!("	{}{};\n", type_to_c(typealt), local.ident.string));
		    }
		    _ => {
		        locals_string.push_str(&format!("	{} {};\n", type_to_c(typealt), local.ident.string));
		    }
		}
	}

	//ENTRY ===================================================
	let mut entry_label= String::from("bb");
	for digit in &function.entry.digits {
		entry_label.push_str(&digit.digit.to_string());
	}
	let entry_string = format!("	goto {};", entry_label);

	// BLOCKS: label, statments, terminator =======================
	let mut blocks_string = String::new();

	for block in &function.blocks {
		// analysis consumption
		let mut is_unreachable = false;
		for unreachable in unreachable_blocks {
			if unreachable.label == block.label {
				is_unreachable = true;
			}
		}
		if is_unreachable == true {
			continue; //ignore it 
		}

		let mut label = String::from("bb");
		for digit in &block.label.digits {
			label.push_str(&digit.digit.to_string());
		}
		let label_string = format!("\n{}:\n", label);
		blocks_string.push_str(&label_string);

		for stmt in &block.stmt {
			let stmt_string = format!("    {}\n", convert_ops(stmt));
			blocks_string.push_str(&stmt_string);
		}

		let final_string = format!("	{}\n", convert_terminators(&block.term));
		blocks_string.push_str(&final_string);
	}

	c.push_str(&includes);
	c.push('\n');
	c.push_str(&format!("{} {}({}) {{\n", rettype, function_name, params_string));
	c.push_str(&locals_string);
	c.push('\n');
	c.push_str(&entry_string);
	c.push('\n');
	c.push_str(&blocks_string);
	c.push_str("}\n");

	// if function_name != "main" { // this was done to keep the compile command exactly the same as in spec
    // 	c.push('\n');
    // 	c.push_str("int main(void) {\n");
    // 	c.push_str("    return 0;\n");
    // 	c.push_str("}\n");
	// }
	c 
}

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
fn include_includes(program: &Program, header_name: &str) -> String {

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

        	Type::Path(_) => needs_custom = true,

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
    	to_print.push_str(&format!("#include \"{}\"\n", header_name));
    }
    to_print
}

// :: -> _
fn pathsepto_(path : &Path) -> String {
	// scuffed way of doing this probably
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

// put the externtype you didint print into the header file custom_header.file 

fn convert_terminators(term: &Term) -> String {
	match term {
		Term::Jump(label) => {
			let mut target= String::from("bb");
			for digit in &label.digits {
				target.push_str(&digit.digit.to_string());
			}
			format!("goto {target};")
		}

		Term::CJump(condition, label_one, label_two) => {
			let mut target_one= String::from("bb");
			for digit in &label_one.digits {
				target_one.push_str(&digit.digit.to_string());
			}
			let mut target_two= String::from("bb");
			for digit in &label_two.digits {
				target_two.push_str(&digit.digit.to_string());
			}

			format!("if ({}) goto {}; else goto {};", condition.ident.string, target_one, target_two)
		}

		Term::Return(local) => match local {
			Some(local) => format!("return {};", local.ident.string),
			None => format!("return;"),
		},
	}

}