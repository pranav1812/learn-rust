use std::mem::size_of_val as sz;
// fixed length + fixed datatype
pub fn run () {
    let mut arr: [i32;5];
    arr= [1, 2, 3, 4, 5];
    println!("{:?}", arr);
    arr[2]= 69; // possible since the array is mutable
    println!("{:?}", arr);
    // arrays are stack based, the size of the array is known at compile time
    // to find the memory in bytes of an array
    println!("Memory of arr is {}", std::mem::size_of_val(&arr));
    // to find the length of an array
    println!("Length of arr is {}", arr.len());

    // slice an array from index 0 to 3
    let slice= &arr[0..3];
    println!("{:?}", slice);

    // find if the array contains 2
    let two= 2;
    let float_two: f32= 2.0;
    println!("Contains 2? {}", arr.contains(&two) && arr.contains(&(float_two as i32)));
    let mut brr: [&str;3];
    brr= ["one", "two", "three"];
    brr[0]= "uno898989";
    println!("{:?} is {} bytes", brr, sz(&brr));
}