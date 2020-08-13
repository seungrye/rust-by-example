pub fn main() {
    tuples();
    enums();
    destructure_pointers();
    destructure_structures();
}
fn destructure_structures() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    };

    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y } = foo;

    println!("a = {}, b = {}, y = {}", a, b, y);

    // 구조체를 destructuring 하거나 변수의 이름을 변경할 수 있습니다.
    // 순서는 중요하지 않습니다.
    let Foo { y: i, x: j } = foo;
    println!("i = {:?}, j = {:?}", i, j);

    // 변수를 무시하는것도 가능합니다.
    let Foo { y, .. } = foo;
    println!("y = {}", y);

    // 필드에 대한 취급이 빠져있다면, 에러가 발생합니다.
    // let Foo { y } = foo;
}
fn destructure_pointers() {
    let reference = &4;
    match reference {
        // 'reference' 를 '&val' 에 패턴매칭 시키면, 이는 &i32 와 비교하는것과 같습니다.
        // 매치되면 & 가 드랍되어 i32 가 val 로 할당되게 됩니다.
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // & 를 생략하고 싶다면, 매칭시키기 전에 역참조를 하도록 합니다.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // 우측 값이 참조가 아니므로, 다음의 변수는 참조 변수가 아니다.
    let _not_a_reference = 3;

    // 'ref' 는 참조임을 명시하고자 하는 의도입니다. 이 참조는 할당됩니다.
    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            // 참조를 얻고, 이를 역참조하여 값을 추가합니다.
            *m += 10;
            println!("We added 10, 'mut_value': {:?}", m);
        }
    }
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}
fn enums() {
    let color = Color::RGB(122, 17, 40);
    println!("What color is it?");
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, satuation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, satuation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!(
            "Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
            c, m, y, k
        ),
    }
}
fn tuples() {
    let pair = (0, -2);

    println!("Tell me about {:?}", pair);

    // match 는 tuple 의 역구조화에 사용될 수 있다.
    match pair {
        (0, y) => println!("First is '0' and 'y' is '{:?}", y),
        (x, 0) => println!("'x' is '{:?}' and last is '0'", x),
        _ => println!("It doesn't matter what they are"),
    }
}
