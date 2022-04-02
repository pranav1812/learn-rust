// this module is for illustrating maps in rust

// map: string -> string
struct Map<K, V> {
    entries: Vec<(K, V)>,
}

pub fn print_map(map: &Map<String, String>) {
    for (key, value) in map.entries.iter() {
        println!("{} -> {}", key, value);
    }
}