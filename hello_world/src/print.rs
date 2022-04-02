// create a public function print_something which accepts a string as a parameter and prints it to the console.

pub fn print_something(s: &str) {
    println!("{} {}", s, "!");
    println!("{name} is {trait}", name = "Rust", trait = "Awesome");
    println!("{1} {0}" , "Hello", "World");
    println!("{:?}", (1, "Sixty Nine", 3)); // {:?} is a special format string that prints the value in a way that is easy to read.
    println!("10 in binary is {:b}, hexadecimal is {:x}, and in ocatal is {:o}", 10, 10, 10);
}