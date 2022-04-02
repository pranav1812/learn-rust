// mod print;
// mod var;
// mod string;
// mod arrays;
// mod vectors;
// mod conditionals;
// mod loops;
mod pointers;
mod structs;
mod enums;
mod cli;

fn main() {
    println!("Hello, world!");
    // print::print_something("Qwerty 69");
    // var::run();
    // string::run();
    // arrays::run();
    // vectors::run();
    // conditionals::run();
    // loops::run(100);

    // closures: these are like lambdas in python
    // syntax: |param1: param1 type, param2: type| -> return type {some one line operation} 
    let add_one = |x: i32| -> i32 { x + 1 };    
    println!("{}", add_one(1));

    //
    pointers::run();
    structs::run();
    let mut p= structs::Person::new("John", "V");
    println!("{}", p.get_full_name());
    p.set_last_name("Vikings");
    println!("{}", p.get_full_name());
    enums::run();
    cli::run();
}
