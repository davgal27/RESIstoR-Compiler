#![warn(clippy::pedantic)]// will remove if(when) this gets annoying, keeping only to act as a guide while I write bad rust

// https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form
// above is so I can remember thought process when I inevitably forget how I did it in 2 days :) 

pub struct Program {
	pub externtype: Vec<ExternType>, 
	pub function: Function,
}
pub struct ExternType {
	pub path: Path,
	pub field: Vec<Field>,
}
pub struct Field {
	pub ident: Ident,
	pub typealt: Type, // type exists in rust already so typealt is used 
}
pub struct Function {
	pub path: Path,
	pub params: Option<Params>, // [] is optional for EBNF
	pub rettype: RetType,
	pub locals: Vec<(Local, Type)>,
	pub entry: Label,
	pub block: Vec<(Block)>,
}
pub struct Params {
	pub params: Vec<(Param)>,
}
pub struct Param {
	pub param: (Local, Type),
}
pub enum RetType {
	Void,
	typealt(Type)
}
pub struct Block {
	pub label: Label,
	pub stmt: Vec<Stmt>,
	pub term: Term,	
}
pub struct Stmt {
	pub local: Option<Local>,
	pub rhs: Rhs,
}
pub enum Rhs {
	Use(Local),
	Const(Literal),
	Cast(Local, Type),
	Un(UnOp, Local),
	Bin(BinOp, Local, Local),
	Addr_of(Local),
	Member_ptr(Local, Ident),
	Load(Local),
	Store(Local, Local),
	Call(Path, Option<Args>),	
}
pub struct Args{
	pub locals: Vec<(Local)>,
}
pub enum Term{
	Jump(Label),
	CJump(Local, Label, Label),
	Return(Option<Local>),
}
pub enum Type{
	PrimType(PrimType),
	Path(Path),
	Ptr(Box<Type>), // https://doc.rust-lang.org/book/ch15-01-box.html#enabling-recursive-types-with-boxes
}
pub enum PrimType{
	Bool,
	I32,
	I64,
	U32,
	F64,
}
pub struct Path{
	pub ident: Vec<(Ident)>,
}
pub struct Local{
	pub ident: Ident,
}
pub struct Label {
	pub digit: Vec<(Digit)>,
}
pub enum Literal{
	IntegerLiteral(i64), 
	FloatLiteral(f64),
	True,
	False,
	Null,
}
pub enum UnOp{
	Neg,
	Not,
}
pub enum BinOp{
	Add,
	Sub,
	Mul,
	Div,
	Mod,
	Eq,
	Ne,
	Lt,
	Le,
	Gt,
	Ge,
	And,
	Or,
}
pub struct Ident{
	pub string: String,
}
pub struct Digit{
	pub digit: u8,
}


