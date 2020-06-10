use std::convert::From;

pub fn main() {
    from();
    into();
}

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn from() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
}

fn into() {
    let int = 5;
    // if you have implemented the From trait for your type,
    // Into will call it when necessary
    let num: Number = int.into();
    println!("My number is {:?}", num);
}