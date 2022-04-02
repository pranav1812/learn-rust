
pub fn run(n:i32){
    // while loop
    let mut count: u8= 0;
    while count<= n as u8 {
        if count % 15 == 0 {
            println!("fizzbuzz: {}", count);
        } else if count % 5 == 0 {
            println!("buzz: {}", count);
        } else if count % 3 == 0 {
            println!("fizz: {}", count);
        }
        count+= 1; 
    }
    // for loop for fizzbuzz 
    for count in 0..n as u8 {
        if count % 15 == 0 {
            println!("fizzbuzz: {}", count);
        } else if count % 5 == 0 {
            println!("buzz: {}", count);
        } else if count % 3 == 0 {
            println!("fizz: {}", count);
        }
    }

    // for loop with jump of 3


}