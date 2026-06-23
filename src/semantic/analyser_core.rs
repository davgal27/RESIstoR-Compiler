use std::collections::HashMap;
use crate::parser::ir::*;
use crate::parser::cfg::Cfg;

struct SymbolTable {
    scope: HashMap<String, Type>,
}

impl SymbolTable {
    fn new() -> SymbolTable {
        SymbolTable {
            scope: Hashmap::new()
        }
    }

    fn insert() -> (&mut self, name: &str, typealt: Type) {
        self.scope.insert(name.to_string(), typealt);
    }
    
    // take local and output its type
    fn lookup(&self, local: &Local) -> Result<Type, string> {
        match self.scope.get(&local.ident.string) {
            Some(typealt) => Ok(typealt.clone()),
            None => Err(format!("%{local.ident.string} is used in your code, but has no type! Consider declaring it."))
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

    check_labels_unique(cfg)?;
    check_entry_existscfg)?;
    check_targets_exist(cfg)?; 
    check_locals_declared(function, &symboltable)?;
    check_cjump_is_bool(function, &symboltable)?; 
    check_returns_match(function, &symboltable)?; 
    check_custom_types_declared(program)?;
    check_statements(program, &symboltable)?; 
 
    Ok(true)
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
                Some(local) => symboltable.lookup(local)?,
                None => {} 
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
// also check that voids dont return anything 
fn check_returns_match(function: &Function, symboltable: &SymbolTable) -> Result<(), String> {
    for block in &function.blocks {
        if let Term::Return(return_local) = &block.term { // return statement
            match &function.rettype { // matches function signature 

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
            Type::Path(path) => {
                let mut declared = false;
                for externtype_block in &program.externtypes {
                    if path == &externtype_block.path {
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

// 8. Check each RHS variant according to what is allowed:

// a. Local copy checks that LHS and RHS have the same type.

// b. Const checks that the literal is compatible with the destination type.

// c. Cast checks that the destination type matches the explicit cast target type.

// d. Unary operations check that neg is used on numeric types and not is used on bool.

// e. Binary operations check arithmetic, modulo, comparison, equality, and boolean operators according to their expected operand/result types.

// f. addr_of checks that the result is ptr<T> where T is the source local type.

// g. load checks that the source is ptr<T> and the destination is T.

// h. store checks that there is no destination, the pointer is ptr<T>, and the stored value is T.

// i. member_ptr checks that the base is ptr<P>, P is declared as an extern type, the field exists on P, and the result is ptr<T> where T is the field type.

// j. call checks that all argument locals are declared and that the destination local is declared if one is used. 
// Since external function signatures are not declared in the ResIR input, call validation is limited to local-use and destination checks rather than full argument/return type checking.

