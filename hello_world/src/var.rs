
pub fn run() {
    let name= "Rust"; // variables in rust are immutable by default
    let mut desc= "Nice"; // mut = mutable
    println!("{} is {}", name, desc);
    desc= "Cool";
    println!("{} is {}", name, desc);
    // define an intteger variable: rank
    let rank: i32 = 1; 
    // define a float variable: weight
    let weight: f32 = 15.6;
    println!("rank*weight = {}", rank as f32*weight);
    // constant
    const MAX_POINTS: u32 = 100;
    println!("Max points: {}", MAX_POINTS);
    // define multiple variables toether
    let (x, y, z)= (1, 2, 3);
    println!("x={}, y={}, z={}", x, y, z);
    // Primitive Types:
    // Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
    // Floats: f32, f64
    // Booleans: bool
    // Characters: char
    // Tuples: Tuple structs
    // Arrays: Array structs
    // Enums: Enum structs
    let is_greater: bool= 10.0 > 5.0;
    let emoji: char= '\u{1F600}';
    println!("Is 10.0 greater than 5.0? {} {}", is_greater, emoji);
    println!("Max of i32: {}", std::i32::MAX);
    println!("Max of u64: {}", std::u32::MAX);
    let arr= [1, 2, 3];
    println!("arr[0] = {}", arr[0]);
    // declare an array brr if i32 of length 3;
    let brr: [i32;3];
    brr= [1, 2, 3]; // Arrays are fixed in size
    println!("brr = {:?}", (brr));

}