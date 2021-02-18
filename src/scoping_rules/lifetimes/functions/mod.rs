//fn invalid_output<'a>() -> &'a String { &String::from("foo") }
// The above is invalid: `'a` must live longer than the function.
// Here, `&String::from("foo")` would create a `String`, followed by a
// reference. Then the data is dropped upon exiting the scope, leaving
// a reference to invalid data to be returned.

pub fn main() {
    let (x, y) = (7, 9);
    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    println!("pass_x({}, {}) => {}", x, y, z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}

fn add_one<'a>(i: &'a mut i32) {
    *i += 1;
}
fn print_one<'a>(x: &'a i32) {
    println!("print_one => x is {}", x);
}

fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("print_multi => x is {}, y is {}", x, y);
}

fn pass_x<'a, 'b>(x: &'a i32, _y: &'b i32) -> &'a i32 {
    return x;
}
