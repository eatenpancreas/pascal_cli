use crate::programs::{Program, programs};



pub const hello_world: Program = Program {
    name: "hwd",
    full_name: "Hello world",
    description: "Hello world is a program that says: Hello world. It takes in no arguments.",
    run,
};

fn run(source: &String, program: &String, raw_add_args: &[String]) {
    println!("Hello world!");
}