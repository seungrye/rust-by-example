pub fn main() {
    {
        let x = 1u8; // unsigned 8bit
        println!("size of 'x' in bytes: {}", std::mem::size_of_val(&x));
    }
    {
        let y = 2u32; // unsigned 32bit
        println!("size of 'y' in bytes: {}", std::mem::size_of_val(&y));
    }
    {
        let z = 3f32; // float 32bit
        println!("size of 'z' in bytes: {}", std::mem::size_of_val(&z));
    }
    {
        let i = 1;
        println!("size of 'i' in bytes: {}", std::mem::size_of_val(&i));
    }
    {
        let f = 1.0;
        println!("size of 'f' in bytes: {}", std::mem::size_of_val(&f));
    }
}
