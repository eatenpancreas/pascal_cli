mod programs;
#[cfg(test)]
mod tests;

use std::env;
use crate::programs::{get_programs, select_program};

// input examples:
// 
// HWD // this is the program that is going to run
// location:src/hello_world // this is an argument
// src/hello_world // this is the same argument, but in the order that the commandline expects.
// running the program without any arguments should explain what to do


fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() >= 2 {
        let (_, add_args) = args.split_at(2);
        select_program((&args[0], &args[1], add_args));
    } else {
        println!("Programs: ");
        println!();
        let ps = get_programs();
        for program in ps {
            println!("{}", program.name)
        }
    }
}
