#[allow(clippy::print_literal)]
pub fn main() {
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    println!("{number:>width$}", number = 1, width = 6);

    println!("{number:>0width$}", number = 1, width = 6);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[derive(Debug)]
    struct Structure(i32);
    println!(
        "This Debug derived struct `{:?}` will print...",
        Structure(3)
    );

    let pi:f64 = std::f64::consts::PI;
    println!("Pi is roughly {pi:.3}", pi = pi);
}
