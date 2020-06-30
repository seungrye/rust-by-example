use ::function_name::named;

fn tuples() {
    let pair = (0, -2);

    println!("Tell me about {:?}", pair);

    // match can be used to destructure a tuple
    match pair {
        (0, y) => println!("First is '0' and 'y' is '{:?}'", y),
        (x, 0) => println!("'x' is '{:?}' and last is '0'", x),
        _ => println!("It doesn't matter what they are"),
    }
}

#[allow(dead_code)]
enum Color {
    //These  3 are specified solely by their name
    Red,
    Blue,
    Green,
    // These likewise tie 'u32' tuples to dirrerent name: color model
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32)
}
fn enums() {
    let color = Color::RGB(122, 17, 40);

    println!("What color is it?");
    match color {
        Color::Red  => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, Green: {}, Blue: {}", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, Saturation: {}, Value: {}", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, Saturation: {}, Lightness: {}", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, Magenta: {}, Yellow: {}", c, m, y),
        Color::CMYK(c, m, y, k) => println!("Cyan: {}, Magenta: {}, Yellow: {}, Key (Black): {}", c, m, y, k),
    }
}

fn pointers_n_ref() {
    // Assign a reference of type 'i32'. The '&' signifies there is a reference being assigned.
    let reference = &4;

    match reference {
        // 만약 reference 변수를 '&val' 에 패턴매칭을 하면, &i32 과 &val 로 비교될 수 있다.
        // 이 경우, 매칭되는 '&' 는 빠지고, i32 가 val 에 할당된다.
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // To avoid the '&', you dereference before matching
    match *reference {
        val => println!("Got a  value via destructuring: {:?}", val),
    }

    let _not_a_reference = 3;

    let ref _is_a_reference = 3;  // 참조를 위해 'ref' 라는 키워드를 사용할수도 있음. (우측값에 '&' 를 붙이지 않아도 됨)

    // 다음의 값들은 'ref' 및 'ret mut' 로 받을 수 있습니다.
    let value = 5;
    let mut mut_value = 6;

    match value {
        // &r => println!("Got a reference to a value: {:?}", r),  // 이걸로는 받을 수 없음
        ref r => println!("Got a reference to a value : {:?}", r),
    }

    match mut_value {
        // &mut mr => println!("Got a reference to a value : {:?}", mr),  // 이걸로는 받을 수 없음
        ref mut mr => {
            *mr += 10; // mutable 참조를 획득했고, 값을 수정하기 위해서는 먼저 역참조(*)를 해야 한다.
            println!("We added 10, mut_value is {:?}", mr);
        }
    }
}
#[named]
fn structs() {

    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x:(1, 2), y:3};

    match foo {
        Foo { x: (1, b), y} => println!("First of x is 1, b = {}, y = {}", b, y),
        Foo { x: i, y: 2} => println!("y is 2 and i => {:?}", i),
        // some variable can ignored (무시되어야 할 변수들은, .. 으로 취급되며, 뒤에 위치해야 함)
        Foo { y, ..} => println!("y = {}", y),
    }
}
pub fn main() {
    tuples();
    enums();
    pointers_n_ref();
    structs();
}