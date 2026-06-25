// https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form
// above is so I can remember thought process when I inevitably forget how I did it in 2 days :) 
#[derive(Debug, Clone)]
pub struct Program {
	pub externtypes: Vec<ExternType>,  //custom type that the function references must be declared at the top of the file
	pub function: Function, //one function per program
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExternType {
	pub path: Path,
	pub fields: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
	pub ident: Ident,
	pub typealt: Type, // type exists in rust already so typealt is used 
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
	pub path: Path,
	pub params: Option<Params>, // [] is optional for EBNF
	pub rettype: RetType,
	pub locals: Vec<(Local, Type)>,
	pub entry: Label, // label for the entry block
	pub blocks: Vec<Block>, // entry block + zero or more additional blocks (hence vec)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Params {
	pub params: Vec<Param>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
	pub local: Local,
	pub typealt: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RetType { // function either returns type or nothing
	Void,
	typealt(Type)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
	pub label: Label, // when Label matches entry, it is the entry block
	pub stmt: Vec<Stmt>,
	pub term: Term,	// every block ends with exactly one terminator
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stmt {
	pub local: Option<Local>,//lhs
	pub rhs: Rhs,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Rhs {
	Use(Local),
	Const(Literal),
	Cast(Local, Type),
	Un(UnOp, Local),
	Bin(BinOp, Local, Local),
	Addr_of(Local),
	Member_ptr(Local, Ident), // %ident(string)/ident(string): eg: member_ptr %p, x 
	Load(Local),
	Store(Local, Local),
	Call(Path, Option<Args>),	
}

#[derive(Debug, Clone, PartialEq)]
pub struct Args{
	pub locals: Vec<Local>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Term{
	Jump(Label), // name of block to jump to
	CJump(Local, Label, Label), //local is the condition. Label1 = case true, else label2
	Return(Option<Local>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type{
	PrimType(PrimType),
	Path(Path),// custom type 
	Ptr(Box<Type>), // https://doc.rust-lang.org/book/ch15-01-box.html#enabling-recursive-types-with-boxes
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrimType{
	Bool,
	I32,
	I64,
	U32,
	F64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Path{
	pub ident: Vec<Ident>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Local{
	pub ident: Ident, // eg %p 
}

#[derive(Debug, Clone, PartialEq)]
pub struct Label {
	pub digits: Vec<Digit>, //3 will produce bb3 
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal{
	IntegerLiteral(i64), 
	FloatLiteral(f64),
	True,
	False,
	Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnOp{
	Neg,
	Not,
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct Ident{
	pub string: String, //variable name like x p or is_neg
}

#[derive(Debug, Clone, PartialEq)]
pub struct Digit{
	pub digit: u32,
}