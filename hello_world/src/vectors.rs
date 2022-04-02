use std::mem::size_of_val as sz;

// vectors are heap allocated, => inorder to get the memory in bytes of a vector, we  
pub fn run(){
    let mut vec: Vec<i32> = vec![1, 2, 3, 4, 5]; // create a vector
    // append an element to the end of the vector
    vec.push(6);
    // prepend an element to the beginning of the vector
    vec.insert(0, 0);
    println!("{:?} is {} bites", vec, sz(&vec));
    let mut vec2: Vec<i32> = vec![100, 200, 300, 400, 500, 600];
    // concatenate two vectors
    vec2.extend(vec.iter().cloned());
    println!("{:?} is {} bites", vec2, sz(&vec2));
    // pop the last element from the vector
    vec2.pop();
    println!("{:?} is {} bites", vec2, sz(&vec2));
    // remove the first element from the vector
    vec2.remove(0);
    println!("{:?} is {} bites", vec2, sz(&vec2));
    let slice= &vec2[0..3];
    println!("{:?} is {} bites", slice, sz(&slice));
    // loop through the vector
    for x in &vec2 {
        println!("{}", x);
    }
    // .iter returns an iterator
    for x in vec2.iter_mut() {
        *x= *x * *x;
    }
    println!("{:?} is {} bites", vec2, sz(&vec2));
}