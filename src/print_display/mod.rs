use std::fmt;
pub fn main() {
    let minmax = MinMax(0, 14);
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);
    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2 { x: 3.3, y: 7.2 };
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let complex = Complex { x: 3.3, y: 7.2 };
    println!("Compare Complexs:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}

#[derive(Debug)]
struct Complex {
    x: f32,
    y: f32,
}
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{} + {}i", self.x, self.y);
    }
}
#[derive(Debug)]
struct Point2 {
    x: f32,
    y: f32,
}
impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "x: {}, y: {}", self.x, self.y);
    }
}
#[derive(Debug)]
struct MinMax(i64, i64);
impl fmt::Display for MinMax {
    // WIP
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "({}, {})", self.0, self.1);
    }
}
