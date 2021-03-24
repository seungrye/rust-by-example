#[allow(clippy::needless_return)]
fn age() -> u32 {
    return 15;
}

pub fn main() {
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I am not born yet I guess"),
        // match 를 1 ... 12 까지 직접적으로 할 수 있지만, 그러면 정확히 나이를 알 수 없으므로,
        // n 에 1 ... 12 를 바인드 한다. 이제 나이를 알수 있게 되었다.
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

#[allow(clippy::needless_return)]
#[allow(clippy::unnecessary_wraps)]
fn some_number() -> Option<u32> {
    return Some(42);
}