pub fn main() {
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            print!("fb ");
        } else if n % 3 == 0 {
            print!("f ");
        } else if n % 5 == 0 {
            print!("b ");
        } else {
            print!("{} ", n);
        }

        n += 1;
    }
    println!("");
}