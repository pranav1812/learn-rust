// Primitive str = Immutable fixed length String, it is stored on the stack
// String = Growable, heap allocated string
pub fn run (){
    let primitive_string= "Rust is awesome!";
    println!("{}", primitive_string);
    let growable= String::from("Rust is awesome!");
    println!("{}", growable);
    let mut growable_mut= String::from("Rust is awesome");
    println!("{}", growable_mut);
    growable_mut.push_str(" and I'm learning Rust!");
    println!("{}", growable_mut);
    let mut gr= String::new();
    println!("{} bytes: initial capacity of gr", gr.capacity());
    gr.push('@');
    println!("{} bytes: capacity after adding 1 character", gr.capacity());
    let x= '\u{1F425}'; // chicken emoji: \u{1F425}
    let y= String::from("Rust is sexy");
    gr.push_str(&y);
    gr.push(x);
    println!("{} {}, {} and does it contain sex {}", gr, gr.len(), gr.capacity(), gr.contains("sex"));
    // difference bw from method and new method is that new method returns a new String whereas from method returns a reference to the String
    // create a string with capacity 16
    let mut with_cap_16= String::with_capacity(16);
    println!("{} bytes: capacity of with_cap_16", with_cap_16.capacity());
    with_cap_16.push('#');
    // assert that the capacity is 16 and length is 1
    assert_eq!(with_cap_16.capacity(), 16);
    assert_eq!(with_cap_16.len(), 1);
    println!("All OK");
}