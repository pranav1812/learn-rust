
// traditional struct
struct Color{
    red: u8,
    green: u8,
    blue: u8
}

struct Col (u8, u8, u8);

pub struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    pub fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
    pub fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    pub fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }
}

pub fn run () {
    let mut c = Color{red: 255, green: 0, blue: 0};
    let mut c2 = Col(0, 240, 56);

    c.red = 200;
    println!("{}, {}", c.red, c.green);
    c.red= 0;
    println!("{}", c.red);
    println!("{}, {}", c2.0, c2.1); 
}