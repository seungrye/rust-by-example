pub fn main() {
    let x = 1u8; // unsigned 8bit
    let y = 2u32; // unsigned 32bit
    let z = 3f32; // float 32bit

    let i = 1;
    let f = 1.0;

    println!("size of 'x' in bytes: {}", std::mem::size_of_val(&x));
    println!("size of 'y' in bytes: {}", std::mem::size_of_val(&y));
    println!("size of 'z' in bytes: {}", std::mem::size_of_val(&z));
    println!("size of 'i' in bytes: {}", std::mem::size_of_val(&i));
    println!("size of 'f' in bytes: {}", std::mem::size_of_val(&f));
}