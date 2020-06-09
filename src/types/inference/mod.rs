pub fn main() {
    let elem = 5u8;

    // at this point, the compiler doesn't know the exact type of 'vec',
    // it just knows that i's a vector of something('Vec<_>')
    let mut vec = Vec::new();

    // now the compiler knows that 'vec' is a vector of 'u8' ('Vec<u8>')
    vec.push(elem);

    println!("{:?}", vec);
}