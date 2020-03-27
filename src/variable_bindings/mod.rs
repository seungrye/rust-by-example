fn variable_bindings() {
    let integer = 1u32;
    let boolean = true;
    let unit = ();

    let copied_integer = integer;

    println!("Integer: {:?}", copied_integer);
    println!("Boolean: {:?}", boolean);
    println!("Meet the unit value: {:?}", unit);

    let _unused_variable = 3u32;

    let _noisy_unused_variable = 2u32;
}
pub fn main() {
    variable_bindings();
    mutability();
    scope_and_shadowing();
    declare_first();
}

fn declare_first() {
    let binding;

    {
        let x = 2;
        binding = x * x;
    }

    println!("binding: {}", binding);

    let another_binding;
    // 초기화되지 않은 바인딩 사용은 에러.
    //println!("another binding: {}", another_binding);

    another_binding = 1;

    println!("another binding: {}", another_binding);
}
fn scope_and_shadowing() {
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);

        // 이 바인딩은 외부 변수의 *shadow* 이다.
        let long_lived_binding = 5_f32;
        println!("inner long: {}", long_lived_binding);
    }

    //println!("outer short: {}", short_lived_binding);
    println!("outer long: {}", long_lived_binding);

    // 이 바인딩 또한 이전 바인딩의 *shadow* 이다.
    let long_lived_binding = 'a';
    println!("outer long: {}", long_lived_binding);
}
fn mutability() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;
    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    //_immutable_binding + 1;
}
