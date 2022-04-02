
pub fn run (){

    // With non-primitive types, if you want to copy another variable, you need to use & sign, else first variable will be lost
    let vec1: Vec<i32> = vec![1,2,3];
    let vec2= &vec1;
    println!("{:?} \n{:?}", vec1, vec2);
    
}