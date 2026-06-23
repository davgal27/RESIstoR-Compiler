use super::ir::*;
use crate::lexer::tokens::{Token, TokenKind}; 

pub struct Parser {
	tokens: Vec<Token>, // batch of tokens from lexer 
	current: usize, // index of next token to consume 
}

impl Parser {

	pub fn new(tokens: Vec<Token>) -> Parser {
		Parser {
			tokens,
			current: 0,
		}
	}

	// =================HELPER FUNCTIONS========================
	/* 
	Most of these helpers are inspired from the "jlox" 
	implementation from Crafting Interpreters,
	part of which demonstrates the construction of a recursive descent
	parser of a simple language in java.
	*/ 

	fn yell_error(&self, message: &str) -> String {
		let t = self.peek();
		format!("Syntax Error: {} at Line: {}, Col: {}",
			message,
			t.token_attribute.line,
			t.token_attribute.col,
			)
	}

    // look but dont consume 
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    // for when alternatives are possible (enums) 
    fn match_token(&mut self, kind: TokenKind) -> bool {
        if self.check_tokenkind(kind) {
            self.advance();
            return true;
        }
        false 
    }
 
    // what did we just consume?
    fn get_previous_token(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
 
    fn is_at_end(&self) -> bool {
        self.peek().token_kind == TokenKind::EndOfFile
    }
 
    // is the current token this kind?
    fn check_tokenkind(&self, kind: TokenKind) -> bool {
        if self.is_at_end() {
        	return false; 
        }
        self.peek().token_kind == kind
    }
 
    // consume the current token and return it
    fn advance(&mut self) -> Token {
        if !self.is_at_end() { self.current += 1; }
        self.get_previous_token().clone()
    }

    // if the current token is what we expect: consume and return
    // else error
    fn consume_token(&mut self, kind: TokenKind, message: &str) -> Result<Token, String> {
        if self.check_tokenkind(kind) {
            return Ok(self.advance());
        }

        Err(self.yell_error(message))
    }

	// ========================Parse x ===============================
	// parse x => returns Result<x, String>.
	// ? operator: if err return, 

	pub fn parse_program(&mut self) -> Result<Program, String> {

		let mut externtypes = Vec::new();
		while self.check_tokenkind(TokenKind::Extern) {
			externtypes.push(self.parse_externtype()?); //? operator helps the code not be a bajillion lines long
		}

		let function = self.parse_function()?;

		if self.is_at_end() == false{
			return Err(self.yell_error("nothing should be written after function"))
		} 
		// okay if everything succeeds from the ?. If a ? fails then error
		Ok(Program {
			externtypes,
			function,
		})
	}

	fn parse_externtype(&mut self) -> Result<ExternType, String> {

		self.consume_token(TokenKind::Extern, "expected 'extern'")?;
		self.consume_token(TokenKind::Type, "expected 'type' after 'extern'")?;

		let path = self.parse_path()?;

		self.consume_token(TokenKind::LCurly, "expected {")?;

		let mut fields = Vec::new();
		while self.check_tokenkind(TokenKind::Identifier) {
			fields.push(self.parse_field()?);
			self.consume_token(TokenKind::SemiColon, "expected ;")?;
		}

		self.consume_token(TokenKind::RCurly, "expected }")?;

		Ok(ExternType {
			path,
			fields, 
		})
	}

	fn parse_field(&mut self) -> Result<Field, String> {
		let ident = self.parse_ident()?;

		self.consume_token(TokenKind::Colon, "expected :")?;

		let typealt = self.parse_type()?;

		Ok(Field {
			ident,
			typealt
		})
	}

	fn parse_function(&mut self) -> Result<Function, String> {
		self.consume_token(TokenKind::Function, "expected 'function'")?;
		
		let path = self.parse_path()?;

		self.consume_token(TokenKind::LBracket, "expected (")?;

		//optional
		// check if there is a ) right after (, then param list empty
		let rbrack_present = self.check_tokenkind(TokenKind::RBracket);
		let mut params = None;
		if rbrack_present == false {
			params = Some(self.parse_params()?); 
		}

		self.consume_token(TokenKind::RBracket, "expected )")?;

		self.consume_token(TokenKind::Arrow, "expected Arrow")?;

		let rettype = self.parse_rettype()?;

		self.consume_token(TokenKind::LCurly, "expected {")?;

		self.consume_token(TokenKind::Locals, "expected Locals")?;

		self.consume_token(TokenKind::LCurly, "expected {")?;

		let mut locals = Vec::new(); 
		while self.check_tokenkind(TokenKind::Local) {
			let local = self.parse_local()?;

			self.consume_token(TokenKind::Colon, "expected : ")?;

			let typealt = self.parse_type()?;

			self.consume_token(TokenKind::SemiColon, "expected ; after specifying type!")?;
			locals.push((local, typealt)); // psh pair of local and type 
		}

		self.consume_token(TokenKind::RCurly, "expected } ")?;

		self.consume_token(TokenKind::Entry, "expected 'entry' ")?;

		let entry = self.parse_label()?;

		self.consume_token(TokenKind::SemiColon, "expected ; after entry label")?;

		let mut blocks = Vec::new();
		while self.check_tokenkind(TokenKind::Label) {
			blocks.push(self.parse_block()?);
		}

		self.consume_token(TokenKind::RCurly, "expected }")?; 

		Ok(Function {
			path,
			params,
			rettype,
			locals,
			entry,
			blocks
		})
	}

	fn parse_params (&mut self) -> Result<Params, String> {
		let mut params = Vec::new(); 
		params.push(self.parse_param()?);

		while self.check_tokenkind(TokenKind::Comma) {
			self.consume_token(TokenKind::Comma, "expected ,")?;
			params.push(self.parse_param()?);
		}

		Ok(Params {
			params,
		})

	}

    fn parse_param(&mut self) -> Result<Param, String> {
    	let local = self.parse_local()?;

    	self.consume_token(TokenKind::Colon, "expected : between identifier and type")?;

    	let typealt = self.parse_type()?;

    	Ok(Param{
    		local,
    		typealt,
    	})
    }


	fn parse_rettype(&mut self) -> Result<RetType, String> {
	    let is_void = self.match_token(TokenKind::Void);
	    if is_void == true {
	        return Ok(RetType::Void);
	    }
	    let typealt = self.parse_type()?;
	    Ok(RetType::typealt(typealt))
	}

	fn parse_block(&mut self) -> Result<Block, String> {
		let label = self.parse_label()?;

		self.consume_token(TokenKind::Colon, "expected  :")?;

		let mut stmt = Vec::new();
		//statement if it is none of the terminators 
		while self.check_tokenkind(TokenKind::Jump) == false && self.check_tokenkind(TokenKind::CJump) == false &&
			self.check_tokenkind(TokenKind::Return) == false && self.is_at_end() == false {
				stmt.push(self.parse_stmt()?);
				self.consume_token(TokenKind::SemiColon, "unterminated statement... perhaps you forgot a ;?")?;
			}

		let term = self.parse_term()?;

		self.consume_token(TokenKind::SemiColon, "expected ; after a block terminator!")?;

		Ok(Block{
			label,
			stmt,
			term
		})
	}

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
       	// if i understood well this is the reason the assignment stated ALMOST always one token of lookahead 
   		// check if an equal is there after Local
        let checkpoint = self.current;
        self.advance();
        let equals_present = self.check_tokenkind(TokenKind::Equals);
        self.current = checkpoint;

        let mut local = None;
        if equals_present == true {
        	local = Some(self.parse_local()?);
        	self.consume_token(TokenKind::Equals, "Expected = ")?;
        }

        let rhs = self.parse_rhs()?;

        Ok(Stmt{
        	local,
        	rhs
        })

    }

    fn parse_rhs(&mut self) -> Result<Rhs, String> {

	    if self.check_tokenkind(TokenKind::Local) {
	        let local = self.parse_local()?;

	        return Ok(Rhs::Use(local));
	    }

	    if self.check_tokenkind(TokenKind::Const) {
	        self.consume_token(TokenKind::Const, "expected 'const'")?;

	        let literal = self.parse_literal()?;

	        return Ok(Rhs::Const(literal));
	    }

	    if self.check_tokenkind(TokenKind::Cast) {
	        self.consume_token(TokenKind::Cast, "expected 'cast'")?;

	        let local = self.parse_local()?;

	        self.consume_token(TokenKind::To, "expected 'to'")?;

	        let typealt = self.parse_type()?;

	        return Ok(Rhs::Cast(local, typealt));
	    }

	    if self.check_tokenkind(TokenKind::Un) {
	        self.consume_token(TokenKind::Un, "expected 'un'")?;

	        let unop = self.parse_unop()?;

	        let local = self.parse_local()?;

	        return Ok(Rhs::Un(unop, local));
	    }

	    if self.check_tokenkind(TokenKind::Bin) {
	        self.consume_token(TokenKind::Bin, "expected 'bin'")?;

	        let binop = self.parse_binop()?;

	        let local_one = self.parse_local()?;

	        self.consume_token(TokenKind::Comma, "expected ','")?;

	        let local_two = self.parse_local()?;

	        return Ok(Rhs::Bin(binop, local_one, local_two));
	    }

	    if self.check_tokenkind(TokenKind::Addr_of) {
	        self.consume_token(TokenKind::Addr_of, "expected 'addr_of'")?;

	        let local = self.parse_local()?;

	        return Ok(Rhs::Addr_of(local));
	    }

	    if self.check_tokenkind(TokenKind::Member_ptr) {
	        self.consume_token(TokenKind::Member_ptr, "expected 'member_ptr'")?;

	        let local = self.parse_local()?;

	        self.consume_token(TokenKind::Comma, "expected ','")?;

	        let ident = self.parse_ident()?;

	        return Ok(Rhs::Member_ptr(local, ident));
	    }

	    if self.check_tokenkind(TokenKind::Load) {
	        self.consume_token(TokenKind::Load, "expected 'load'")?;

	        let local = self.parse_local()?;

	        return Ok(Rhs::Load(local));
	    }

	    if self.check_tokenkind(TokenKind::Store) {
	        self.consume_token(TokenKind::Store, "expected 'store'")?;

	        let local_one = self.parse_local()?;

	        self.consume_token(TokenKind::Comma, "expected ','")?;

	        let local_two = self.parse_local()?;

	        return Ok(Rhs::Store(local_one, local_two));
	    }

		if self.check_tokenkind(TokenKind::Call) {
		    self.consume_token(TokenKind::Call, "expected 'call'")?;

		    let path = self.parse_path()?;
		    
		    self.consume_token(TokenKind::LBracket, "expected '('")?;
		    
		    let rbrack_present = self.check_tokenkind(TokenKind::RBracket);
		    let mut args = None;
		    if rbrack_present == false {
		        args = Some(self.parse_args()?);
		    }
		    
		    self.consume_token(TokenKind::RBracket, "expected ')'")?;
		    
		    return Ok(Rhs::Call(path, args));
		}

	    Err(self.yell_error("expected a statement"))
	}

	fn parse_args(&mut self) -> Result<Args, String> {
	    let mut locals = Vec::new();
	    let local_one = self.parse_local()?;
	    locals.push(local_one);

	    while self.check_tokenkind(TokenKind::Comma) {
	        self.consume_token(TokenKind::Comma, "expected ,")?;

	        let local_rest = self.parse_local()?;
	        locals.push(local_rest);
	    }

	    Ok(Args {
	    	locals 
	 	})
	}

	fn parse_term(&mut self) -> Result<Term, String> {

	    if self.check_tokenkind(TokenKind::Jump) {
	        self.consume_token(TokenKind::Jump, "expected jump")?;

	        let label = self.parse_label()?;

	        return Ok(Term::Jump(label));
	    }
	    
	    if self.check_tokenkind(TokenKind::CJump) {
	        self.consume_token(TokenKind::CJump, "expected cjump")?;

	        let local = self.parse_local()?;	        
	        
	        self.consume_token(TokenKind::Comma, "expected ,")?;	        
	        
	        let label_one = self.parse_label()?;	        
	        
	        self.consume_token(TokenKind::Comma, "expected ,")?;
	        
			let label_two = self.parse_label()?;
	        
	        return Ok(Term::CJump(local, label_one, label_two));
	    }

	    if self.check_tokenkind(TokenKind::Return) {
	        self.consume_token(TokenKind::Return, "expected return")?;

	        let local_present = self.check_tokenkind(TokenKind::Local);
	        let mut local = None;
	        if local_present == true {
	            local = Some(self.parse_local()?);
	        }

	        return Ok(Term::Return(local));
	    }

	    Err(self.yell_error("expected a terminator (jump, cjump, or return)"))
	}

	fn parse_type(&mut self) -> Result<Type, String> {

	    if self.check_tokenkind(TokenKind::Bool) || self.check_tokenkind(TokenKind::I32)
	    || self.check_tokenkind(TokenKind::I64) || self.check_tokenkind(TokenKind::U32)
	    || self.check_tokenkind(TokenKind::F64) {
	        let primtype = self.parse_primtype()?;

	        return Ok(Type::PrimType(primtype));
	    }

	    if self.check_tokenkind(TokenKind::Identifier) {
	        let path = self.parse_path()?;
	        return Ok(Type::Path(path));
	    }

	    if self.check_tokenkind(TokenKind::Ptr) {
	        self.consume_token(TokenKind::Ptr, "expected ptr")?;

	        self.consume_token(TokenKind::LessThan, "expected <")?;

	        let typealt = self.parse_type()?;

	        self.consume_token(TokenKind::GreaterThan, "expected >")?;

	        return Ok(Type::Ptr(Box::new(typealt)));
	    }

	    Err(self.yell_error("expected a type"))
	}

	fn parse_primtype(&mut self) -> Result<PrimType, String> {
	    if self.check_tokenkind(TokenKind::Bool) {
	        self.consume_token(TokenKind::Bool, "expected bool")?;
	        return Ok(PrimType::Bool);
	    }
	    if self.check_tokenkind(TokenKind::I32) {
	        self.consume_token(TokenKind::I32, "expected i32")?;
	        return Ok(PrimType::I32);
	    }
	    if self.check_tokenkind(TokenKind::I64) {
	        self.consume_token(TokenKind::I64, "expected i64")?;
	        return Ok(PrimType::I64);
	    }
	    if self.check_tokenkind(TokenKind::U32) {
	        self.consume_token(TokenKind::U32, "expected u32")?;
	        return Ok(PrimType::U32);
	    }
	    if self.check_tokenkind(TokenKind::F64) {
	        self.consume_token(TokenKind::F64, "expected f64")?;
	        return Ok(PrimType::F64);
	    }

	    Err(self.yell_error("expected a primitive type"))
	}

	fn parse_path(&mut self) -> Result<Path, String> {
	    let mut ident = Vec::new();

	    let ident_one = self.parse_ident()?;
	    ident.push(ident_one);

	    while self.check_tokenkind(TokenKind::PathSep) {
	        self.consume_token(TokenKind::PathSep, "expected ::")?;

	        let ident_rest = self.parse_ident()?;
	        ident.push(ident_rest);
	    }

	    Ok(Path {
	    	ident 
	 	})
	}

	fn parse_local(&mut self) -> Result<Local, String> {
	    let local = self.consume_token(TokenKind::Local, "expected a local (%name)")?;
	    let lexeme = local.token_attribute.lexeme[1..].to_string(); // remmove the % 
	    let ident = Ident {string: lexeme};

	    Ok(Local {
	    	ident 
	    })
	}

	fn parse_label(&mut self) -> Result<Label, String> {
	    let label = self.consume_token(TokenKind::Label, "expected a label (bbN)")?;

	    let mut digits = Vec::new();
	    for chars in label.token_attribute.lexeme[2..].chars() {
	        let digit = self.parse_digit(chars)?;
	        digits.push(digit);
	    }

	    Ok(Label {
	    	digits
	    })
	}

	fn parse_literal(&mut self) -> Result<Literal, String> {
	    if self.check_tokenkind(TokenKind::IntegerLiteral) {
	        let int = self.consume_token(TokenKind::IntegerLiteral, "expected an integer")?;

	        let parsed_int = int.token_attribute.lexeme.parse::<i64>();//parse converts to other type
	        if parsed_int.is_err() {
	            return Err(self.yell_error("invalid integer literal"));
	        }
	        let int_val = parsed_int.unwrap();

	        return Ok(Literal::IntegerLiteral(int_val));
	    }

	    if self.check_tokenkind(TokenKind::FloatLiteral) {
	        let float = self.consume_token(TokenKind::FloatLiteral, "expected a float")?;

	        let parsed_float = float.token_attribute.lexeme.parse::<f64>();

	        if parsed_float.is_err() {
	            return Err(self.yell_error("invalid float literal"));
	        }
	        let float_val = parsed_float.unwrap();

	        return Ok(Literal::FloatLiteral(float_val));
	    }

	    if self.check_tokenkind(TokenKind::True) {
	        self.consume_token(TokenKind::True, "expected 'true'")?;
	        return Ok(Literal::True);
	    }
	    if self.check_tokenkind(TokenKind::False) {
	        self.consume_token(TokenKind::False, "expected 'false'")?;
	        return Ok(Literal::False);
	    }
	    if self.check_tokenkind(TokenKind::Null) {
	        self.consume_token(TokenKind::Null, "expected 'null'")?;
	        return Ok(Literal::Null);
	    }

	    Err(self.yell_error("expected a literal"))
	}

	fn parse_unop(&mut self) -> Result<UnOp, String> {
	    let t = self.consume_token(TokenKind::Identifier, "expected a unary operator ")?;
	    match t.token_attribute.lexeme.as_str() {
	        "neg" => Ok(UnOp::Neg),
	        "not" => Ok(UnOp::Not),
	        other => Err(format!(
	            "Syntax Error: unknown unary operator '{}' at Line: {}, Col: {}",
	            other, t.token_attribute.line, t.token_attribute.col
	        )),
	    }
	}

	fn parse_binop(&mut self) -> Result<BinOp, String> {
	    let t = self.consume_token(TokenKind::Identifier, "expected a binary operator")?;
	    match t.token_attribute.lexeme.as_str() {
	        "add" => Ok(BinOp::Add),
	        "sub" => Ok(BinOp::Sub),
	        "mul" => Ok(BinOp::Mul),
	        "div" => Ok(BinOp::Div),
	        "mod" => Ok(BinOp::Mod),
	        "eq"  => Ok(BinOp::Eq),
	        "ne"  => Ok(BinOp::Ne),
	        "lt"  => Ok(BinOp::Lt),
	        "le"  => Ok(BinOp::Le),
	        "gt"  => Ok(BinOp::Gt),
	        "ge"  => Ok(BinOp::Ge),
	        "and" => Ok(BinOp::And),
	        "or"  => Ok(BinOp::Or),
	        other => Err(format!(
	            "Syntax Error: unknown binary operator '{}' at Line: {}, Col: {}",
	            other, t.token_attribute.line, t.token_attribute.col
	        )),
	    }
	}

	fn parse_ident(&mut self) -> Result<Ident, String> {
	    let t = self.consume_token(TokenKind::Identifier, "expected an identifier")?;
	    let string = t.token_attribute.lexeme;
	    Ok(Ident {
	    	string
	    })
	}

	fn parse_digit(&self, c: char) -> Result<Digit, String> {
	    let d = c.to_digit(10);
	    if d.is_none() {
	        return Err(self.yell_error(&format!("invalid digit '{}' in label", c)));
	    }
	    let digit = d.unwrap();
	    Ok(Digit{
	    	digit
	    })
	}
}