pub fn main() {
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple fourth value: {}", long_tuple.3);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    println!("one element tuple: {:?}", (5u32));
    println!("just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("{}", matrix);

    println!("Transpose:\n{}", transpose(matrix));
}
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_, bool_) = pair;

    return (bool_, int_);
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "({} {})", self.0, self.1)?;
        return write!(f, "({} {})", self.2, self.3);
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    let trans = Matrix(matrix.0, matrix.2, matrix.1, matrix.3);
    return trans;
}
