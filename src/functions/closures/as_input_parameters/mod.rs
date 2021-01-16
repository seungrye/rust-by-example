pub fn main() {
    use std::mem;

    let greeting = "hello";

    // 'to_owned' creates owned data from borrowed one (??)
    let mut farewell = "goodbye".to_owned();

    // closure 선언
    let diary = || {
        // 'greeting' is by reference: requires 'Fn'
        println!("I said {}.", greeting);

        // Mutation forces 'farewell' to be captured by mutable reference.
        // Now requires 'FnMut'
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzzz");

        // Manually calling drop forces 'farewell' to be captured by value.
        // Now requires 'FnOnce'
        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
}

fn apply<F>(f: F)
where
    F: FnOnce(),
{
    // Fn, FnMut, FnOnce 로 변경해 가면서 테스트 필요.
    f();
}

// A function which takes a closure and returns an 'i32'.
#[allow(clippy::needless_return)]
fn apply_to_3<F>(f: F) -> i32
// and The closure takes an 'i32' and returns an 'i32'
where
    F: Fn(i32) -> i32,
{
    return f(3);
}
