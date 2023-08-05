extern crate nom;

use repl::Repl;

mod vm;
mod instruction;
mod repl;
mod assembler;

fn main() {
    let mut repl = Repl::new();
    repl.run();
}

