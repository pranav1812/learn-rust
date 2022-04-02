
pub fn run(){
    let mut age : u8= 18;
    let is_of_age : bool= age > 18;
    let is_under_age= if age < 22 {true} else {false};
    println!("{} {}", is_of_age, is_under_age);
    age= 22;
    if (age>= 22) || !is_under_age{
        println!("You are old enough to drink");
    }
    else{
        println!("Kids are not allowed to drink");
    }
}