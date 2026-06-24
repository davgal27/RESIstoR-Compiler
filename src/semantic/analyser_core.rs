use std::collections::HashMap;
use crate::parser::ir::*;
use crate::parser::cfg::Cfg;

struct SymbolTable {
    scope: HashMap<String, Type>,
}

impl SymbolTable {
    fn new() -> SymbolTable {
        SymbolTable {
            scope: HashMap::new()
        }
    }

    fn insert(&mut self, name: &str, typealt: Type) {
        self.scope.insert(name.to_string(), typealt);
    }
    
    // take local and output its type
    fn lookup(&self, local: &Local) -> Result<Type, String> {
        match self.scope.get(&local.ident.string) {
            Some(typealt) => Ok(typealt.clone()),
            None => Err(format!("%{} is used in your code, but has no type! Consider declering it.", local.ident.string))
        }
    }
}

pub fn analyse(program: &Program, cfg: &Cfg) -> Result<bool, String> {
    let function = &program.function;

    let mut symboltable = SymbolTable::new(); 
    if let Some(params) = &function.params {
        for param in &params.params {
            symboltable.insert(&param.local.ident.string, param.typealt.clone());
        }
    }
    for (local, typealt) in &function.locals {
        symboltable.insert(&local.ident.string, typealt.clone());
    }

    check_locals_declared(function, &symboltable)?;
    check_labels_unique(cfg)?;
    check_entry_exists(cfg)?;
    check_targets_exist(cfg)?;
    check_statements(program, &symboltable)?; 
    check_returns_match(function, &symboltable)?; 
    check_cjump_is_bool(function, &symboltable)?; 
    check_custom_types_declared(program)?;
 
    Ok(true)
}

/*places to check:
1) LHS of the statement
2) RHS (from the statement) cases where there are locals: use, cast, un, bin, addrof, memberptr, load, store, call(from args)
3) Terminators: cjump and return can have locals
*/
fn check_locals_declared(function: &Function, symboltable: &SymbolTable) -> Result<(), String> {
    for block in &function.blocks{


        for stmt in &block.stmt {
            //1: LHS
            if let Some(lhs_local) = &stmt.local {
                symboltable.lookup(lhs_local)?;
            }

            // 2: RHS
            match &stmt.rhs {
                Rhs::Use(local) => {symboltable.lookup(local)?;}
                Rhs::Cast(local,_) => {symboltable.lookup(local)?;}
                Rhs::Un(_, local) => {symboltable.lookup(local)?;}
                Rhs::Bin(_,local_one, local_two) => {
                    symboltable.lookup(local_one)?;
                    symboltable.lookup(local_two)?;
                }
                Rhs::Addr_of(local) => {symboltable.lookup(local)?;}
                Rhs::Member_ptr(local, _) => {symboltable.lookup(local)?;}
                Rhs::Load(local) => {symboltable.lookup(local)?;}
                Rhs::Store(local_one, local_two) => {
                    symboltable.lookup(local_one)?;
                    symboltable.lookup(local_two)?;
                }
                Rhs::Call(_,args) => match args {
                    Some(args) => {
                        for local in &args.locals{
                            symboltable.lookup(local)?;
                        }
                    }
                    None => {}
                }
                Rhs::Const(_) => {}
            }
        }

        // 3: Terminators 
        match &block.term {
            Term::Jump(_) => {}
            Term::CJump(local,_,_) => {symboltable.lookup(local)?;}
            Term::Return(local) => match local {
                Some(local) => {symboltable.lookup(local)?; }
                None => {} 
            }
        }
    }

    Ok(())    
}

// i could have done this without the cfg, but since I did it before might as well use it 
fn check_labels_unique(cfg: &Cfg) -> Result<(), String> {
    let mut checked_labels: Vec<Label> = Vec::new();

    //build label -> check if its been checked or not 
    for block in &cfg.blocks {
        let mut label = String::from("bb");
        for digit in &block.label.digits {
            let str_from_digit = digit.digit.to_string();
            label.push_str(&str_from_digit);
        }


        if checked_labels.contains(&block.label) == true {
            return Err(format!("duplicate block label {label}"));
        } else {
            checked_labels.push(block.label.clone());
        }
    }

    Ok(())
}

fn check_entry_exists(cfg: &Cfg) -> Result<(), String> {

    let mut found_entry = false; 
    
    for block in &cfg.blocks {
        if block.label == cfg.entry {
            found_entry = true;
        }
    }

    if found_entry == false {
        return Err(format!("Entry block does not exist!"));
    }

    Ok(())
}

fn check_targets_exist(cfg: &Cfg) -> Result<(), String> {

    for (_,target) in &cfg.edges {
        let mut target_exists = false;

        for block in &cfg.blocks {
            if block.label == *target {
                target_exists = true;
            }
        }

        if target_exists == false {
            return Err(format!("Jump or Cjump targets dont exist!"));
        }
    }

    Ok(())
}

// check that LHS = RHS 
// except: Store has no LHS, Const is checking that its compatible not equal, and call doesnt produce an RHS type
// for the above will handle case by case
fn check_statements(program: &Program, symboltable: &SymbolTable) -> Result<(), String> {
    for block in &program.function.blocks {
        for stmt in &block.stmt {

            // gather LHS type. optional because store no LHS 
            let lhs_type: Option<Type> = match &stmt.local {
                Some(local) => Some(symboltable.lookup(local)?),
                None => None,
            };

            // gather RHS type
            let rhs_type: Type = match &stmt.rhs {

                Rhs::Use(local) => symboltable.lookup(local)?,

                Rhs::Const(literal) => {
                    let dest_type = match &lhs_type {
                        Some(lhs_type) => lhs_type,
                        None => return Err("const needs a destination local".to_string()),
                    };
                    let compatible = match literal {
                        Literal::IntegerLiteral(_) => {
                            match dest_type {
                                Type::PrimType(PrimType::I32) => true,
                                Type::PrimType(PrimType::I64) => true,
                                Type::PrimType(PrimType::U32) => true,
                                _ => false,
                            }
                        }
                        Literal::FloatLiteral(_) => {
                            match dest_type {
                                Type::PrimType(PrimType::F64) => true,
                                _ => false,
                            }
                        }
                        Literal::True | Literal::False => {
                            match dest_type {
                                Type::PrimType(PrimType::Bool) => true,
                                _ => false,
                            }
                        }
                        Literal::Null => {
                            match dest_type {
                                Type::Ptr(_) => true,
                                _ => false,
                            }
                        }
                    };
                    if compatible == false {
                        return Err(format!("literal {literal:?} is not compatible with {dest_type:?}"));
                    }
                    continue; // dont check lhs = rhs at the bottom 
                }

                Rhs::Cast(_, typealt) => typealt.clone(),

                Rhs::Un(unop, local) => {
                    let operand_type = symboltable.lookup(local)?;

                    match unop {
                        UnOp::Neg => {
                            let numeric = match operand_type {
                                Type::PrimType(PrimType::I32) => true,
                                Type::PrimType(PrimType::I64) => true,
                                Type::PrimType(PrimType::U32) => true,
                                Type::PrimType(PrimType::F64) => true,
                                _ => false,
                            };

                            if numeric == false {
                                return Err(format!("neg expects a numeric type,but got {operand_type:?}"));
                            }

                            operand_type
                        }

                        UnOp::Not => {
                            if (operand_type == Type::PrimType(PrimType::Bool)) == false {
                                return Err(format!("not expects bool, but got {operand_type:?}"));
                            }

                            Type::PrimType(PrimType::Bool)
                        }
                    }
                }

                Rhs::Bin(binop, left, right) => {
                    let left_type = symboltable.lookup(left)?;
                    let right_type = symboltable.lookup(right)?;
                    match binop {

                        // numeric operators 
                        BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => {
                            let numeric = match left_type {
                                Type::PrimType(PrimType::I32) => true,
                                Type::PrimType(PrimType::I64) => true,
                                Type::PrimType(PrimType::U32) => true,
                                Type::PrimType(PrimType::F64) => true,
                                _ => false,
                            };
                            if numeric == true && left_type == right_type {
                                left_type
                            } else {
                                return Err(format!("{binop:?} expects matching numeric operands, but got {left_type:?} and {right_type:?}"));
                            }
                        }
                        // also numeric but no float (integer)
                        BinOp::Mod => {
                            let integer = match left_type {
                                Type::PrimType(PrimType::I32) => true,
                                Type::PrimType(PrimType::I64) => true,
                                Type::PrimType(PrimType::U32) => true,
                                _ => false,
                            };
                            if integer == true && left_type == right_type {
                                left_type
                            } else {
                                return Err(format!("{binop:?} expects matching integer operands for mod, but got {left_type:?} and {right_type:?}"));
                            }
                        }

                        // no specification, just as long as types are equal 
                        BinOp::Eq | BinOp::Ne => {
                            if left_type == right_type {
                                Type::PrimType(PrimType::Bool)
                            } else {
                                return Err(format!("{binop:?} expects matching operands, but got {left_type:?} and {right_type:?}"));
                            }
                        }

                        // numeric, but return T or F 
                        BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => {
                            let numeric = match left_type {
                                Type::PrimType(PrimType::I32) => true,
                                Type::PrimType(PrimType::I64) => true,
                                Type::PrimType(PrimType::U32) => true,
                                Type::PrimType(PrimType::F64) => true,
                                _ => false,
                            };

                            if numeric == true && left_type == right_type {
                                Type::PrimType(PrimType::Bool)
                            } else {
                                return Err(format!("{binop:?} expects matching numeric operands, but got {left_type:?} and {right_type:?}"));
                            }
                        }

                        // boolean operators(also t / f )
                        BinOp::And | BinOp::Or => {
                            let left_bool = left_type == Type::PrimType(PrimType::Bool);
                            let right_bool = right_type == Type::PrimType(PrimType::Bool);
                            if left_bool == true && right_bool == true {
                                Type::PrimType(PrimType::Bool)
                            } else {
                                return Err(format!("{binop:?} expects bool operands, but got {left_type:?} and {right_type:?}"));
                            }
                        }
                    }
                }

                // addr_of: rhs type is ptr<Type>
                Rhs::Addr_of(local) => {
                    let local_type = symboltable.lookup(local)?;
                    Type::Ptr(Box::new(local_type))
                }

                // member_ptr:
                // 1. operand must have type ptr<P>
                // 2. P is declared with extern block, 
                // 3. field exists, 
                // 4. rhs type is ptr<fieldtype>
                Rhs::Member_ptr(operand, field) => {

                    // 1
                    let operand_type = symboltable.lookup(operand)?;
                    // check for ptr<>
                    let pointed_at = match operand_type {
                        Type::Ptr(inner) => *inner,
                        other => return Err(format!("member_ptr expects a pointer, got {other:?}")),
                    };
                    // check it points to a path 
                    let path = match pointed_at {
                        Type::Path(path) => path,
                        other => return Err(format!("member_ptr expects ptr<Struct>, got ptr<{other:?}>")),
                    };

                    // 2:  (also check which extern block) 
                    let mut declared_struct: Option<&ExternType> = None;
                    for extern_type_block in &program.externtypes {
                        if extern_type_block.path == path {
                            declared_struct = Some(extern_type_block);
                        }
                    }
                    let declared_struct = match declared_struct {
                        Some(extern_type_block) => extern_type_block,
                        None => return Err(format!("{path:?} is not declared as an extern type")),
                    };

                    // 3
                    let mut field_type: Option<Type> = None;
                    for declared_field in &declared_struct.fields {
                        if declared_field.ident.string == field.string {
                            field_type = Some(declared_field.typealt.clone());
                        }
                    }
                    let field_type = match field_type {
                        Some(typealt) => typealt,
                        None => return Err(format!("field {} does not exist on {path:?}", field.string)),
                    };

                    // 4
                    Type::Ptr(Box::new(field_type))
                }
                
                // load: source is ptr<Type>, rhs type is T
                Rhs::Load(local) => {
                    let local_type = symboltable.lookup(local)?;
                    match local_type {
                        Type::Ptr(inner_type) => *inner_type,
                        _ => return Err(format!("load expects a pointer, but got {local_type:?}")),
                    }
                }

                Rhs::Store(ptr_local, val_local) => {
                    if lhs_type.is_some() {
                        return Err(format!("store does not have a destination local"));
                    }

                    let ptr_type = symboltable.lookup(ptr_local)?;
                    let val_type = symboltable.lookup(val_local)?;

                    match ptr_type {
                        Type::Ptr(inner_type) => {
                            if *inner_type != val_type {
                                return Err(format!(
                                    "store cannot put {val_type:?} into pointer {inner_type:?}"
                                ));
                            }

                            continue;
                        }

                        _ => {
                            return Err(format!("store destination must be a pointer, got {ptr_type:?}"));
                        }
                    }
                }

                // no rhs 
                Rhs::Call(_, _) => continue,
            };

            //unpack lhs
            let lhs_type = match lhs_type {
                Some(lhs_type) => lhs_type,
                None => return Err("statement is missing a destination local".to_string()),
            };

            // finally, check if the lhs = the rhs 
            if lhs_type != rhs_type {
                return Err(format!("LHS type {lhs_type:?} does not match RHS type {rhs_type:?}"));
            }
        }
    }

    Ok(())
}

// also check that voids dont return anything 
fn check_returns_match(function: &Function, symboltable: &SymbolTable) -> Result<(), String> {
    for block in &function.blocks {
        if let Term::Return(return_local) = &block.term { // if terminator a return statement, check it
            match &function.rettype { // match the terminator local with function return type

                RetType::typealt(function_type) => {
                    match return_local {
                        Some(local) => { 
                            let return_type = symboltable.lookup(local)?;

                            if return_type != *function_type {
                                return Err(format!("return type is incorrect. received {return_type:?} instead of {function_type:?}"));
                            }
                        }
                        None => {
                            return Err(format!("Function which was supposed to return a value returned none."));
                        }
                    }
                }

                RetType::Void => {
                    if return_local.is_some() {
                        return Err(format!("A void function returned a value!"));
                    }
                }
            }
        }
    }
    Ok(())
}

fn check_cjump_is_bool(function: &Function, symboltable: &SymbolTable) -> Result<(), String> {
    for block in &function.blocks {
        if let Term::CJump(condition,_ ,_) = &block.term {
            let condition_type = symboltable.lookup(condition)?;

            if condition_type != Type::PrimType(PrimType::Bool) {
                return Err(format!("Cjump's condition type must be a boolean. Got {condition_type:?} instead."));
            }
        }
    }
    Ok(())
}


/*places to check: where types can be delcared
1) Fields
2) Paarameters (in functoins)
3) Rettype (in functions)
4) Locals (in functions)
5) Cast operator 
*/
fn check_custom_types_declared(program: &Program) -> Result<(), String> {

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
    for typealt in types_to_check {
        let mut type_to_check = typealt;
        while let Type::Ptr(pointer_type) = type_to_check {
            type_to_check = pointer_type;
        }

        // path holds custom type
        // if the typetc is a path, see that it matches  
        match type_to_check {
            Type::Path(path) => { // is it custom?
                let mut declared = false;
                for externtype_block in &program.externtypes {
                    if path == &externtype_block.path { // okay its custom, is it in the extern block?
                        declared = true; 
                    }
                }
                if declared == false {
                    return Err(format!("custom type {path:?} is referenced but not declared"));
                }
            }
            _ => {} 
        }
    }
    Ok(())
}