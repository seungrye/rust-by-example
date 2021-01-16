
pub fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}

#[allow(clippy::needless_return)]
fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    // 'move' keyword signals that all captures occur by value.
    // This is required because any captures by reference would be dropped as soon as the function exited.
    return move||println!("This is a {}", text);
}

#[allow(clippy::needless_return)]
fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    return move||println!("This is a {}", text);
}

#[allow(clippy::needless_return)]
fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    // Note. 'move' keyword 를 사용하지 않으면, 다음과 같은 에러가 발생합니다.
    // ```note: wrap the `()` in a closure with no arguments: `|| { /* code */ }```
    return move||println!("This is a {}", text);
}
