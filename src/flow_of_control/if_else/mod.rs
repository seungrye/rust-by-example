pub fn main() {
    let n = 5;

    #[allow(clippy::comparison_chain)]
    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");
        n * 10 // assign to big_n
    } else {
        println!(", and is a big number, halve the number");
        n / 2 // assign to big_n
    };

    println!("{} => {}", n, big_n);
}