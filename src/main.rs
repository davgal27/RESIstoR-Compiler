#![warn(clippy::pedantic)] // will keep this while i write subpar rust, though I might remove the pedantic if this becomes too annoying. 
pub mod parser;
pub mod lexer;
pub mod semantic; 

#[cfg(test)]
mod test;
mod samples;

fn main() {
    println!("compiler running ...");
}