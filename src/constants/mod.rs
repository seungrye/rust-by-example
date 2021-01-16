// Note. Statics have by default a `static lifetime
#[allow(clippy::clippy::redundant_static_lifetimes)]
static LANGUAGE: &'static str = "Rust"; // `'static` lifetime 을 갖도록 명시함.
const THRESHOLD: i32 = 10;

#[allow(clippy::clippy::needless_return)]
fn is_big(n: i32) -> bool {
    return n > THRESHOLD;
}
pub fn main() {
    let n = 16;

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    //THRESHOLD = 5; // const 는 수정불가함
}
