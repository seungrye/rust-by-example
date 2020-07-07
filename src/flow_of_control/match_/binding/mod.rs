fn age() -> u32 {
    return 15;
}

pub fn main() {
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I am not born yet I guess"),
        n@1..=12 => println!("I am child of age {:?}", n),
        n@13..=19 => println!("I am teen of age {:?}", n),
        n => println!("I am an old person of age {:?}", n),
    }

    match some_number() {
        Some(n@42) => println!("The Answer: {}", n),
        Some(n) => println!("Not interesting.. {}", n),
        _ => (),  // None
    }
}

fn some_number() -> Option<u32> {
    return Some(42);
}