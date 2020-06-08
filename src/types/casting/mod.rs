pub fn main() {
    let decimal = 65.4321_f32;
    // let integer:u8 = decimal;

    let integer = decimal as u8;
    let character = integer as char;

    // let character = decimal as char; // a float can not be directly converted to a char

    println!("Casting: {} -> {} -> {}", decimal, integer, character);
}