use rand::Rng; // should add 'rand' as dependencies to 'Cargo.toml'

fn how_to_match_use() {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(0, 20);

    println!("Tell me about {}", number);

    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"), // 13 to 19 (include 19)
        _ => println!("Ain't special")
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1
    };

    println!("{} => {}", boolean, binary);
}

pub fn main() {
    how_to_match_use();
}