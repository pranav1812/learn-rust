
enum Moves {
    up,
    down,
    left,
    right
}

fn move_avatar (m: Moves) {
    match m {
        Moves::up => println!("Avatar is moving up"),
        Moves::down => println!("Avatar is moving down"),
        Moves::left => println!("Avatar is moving left"),
        Moves::right => println!("Avatar is moving right"),
    }
}

pub fn run (){
    move_avatar(Moves::up);
    move_avatar(Moves::down);
    move_avatar(Moves::left);
    move_avatar(Moves::right);
}