#![allow(overflowing_literals)] // overflow 관련 경고 무시

pub fn main() {
    casting();
    literals();
    inference();
    alias();
}

type NanoSecond = u64;
type Inch = u64;

/// type 은 이미 존재하는 타입에 새로운 이름을 사용할 수 있습니다.타입
/// 들은 반드시 CamelCase 로 지어야 합니다.
fn alias() {
    let nanoseconds: NanoSecond = 5;
    let inches: Inch = 2;

    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}

fn inference() {
    let elem = 5u8;

    // 비어있는 벡터를 선언한 시점에, 컴파일러는 벡터의 정확한 타입을 알지 못한다.
    // 단순히 알수없는 타입의 벡터라고만 알고 있는 상황이다. (Vec<_>)
    let mut vec = Vec::new(); // 비어 있는 벡터 선언

    vec.push(elem);
    // elem 을 벡터에 추가한 시점에, 컴파일러는 vec 가 u8 타입의 벡터임을 알게 된다. (Vec<u8>)
    // 만약 push 를 주석 처리하면, 컴파일 에러가 발생함을 확인할수 있다.

    println!("{:?}", vec);
}
fn literals() {
    // 접미사가 붙은 리터럴, 이들의 타입은 초기화 될때 알려줄 수 있다.
    {
        let x = 1u8;
        println!("size of 'x' in bytes: {}", std::mem::size_of_val(&x));
    }
    {
        let y = 2u32;
        println!("size of 'y' in bytes: {}", std::mem::size_of_val(&y));
    }
    {
        let z = 3f32;
        println!("size of 'z' in bytes: {}", std::mem::size_of_val(&z));
    }
    // 접미사가 없는 리터럴, 이들의 타입은 그들의 사용처에 달렸다.
    {
        let i = 1;
        println!("size of 'i' in bytes: {}", std::mem::size_of_val(&i));
    }
    {
        let f = 1.0;
        println!("size of 'f' in bytes: {}", std::mem::size_of_val(&f));
    }
}

#[allow(clippy::clippy::unnecessary_cast)]
fn casting() {
    let decimal = 65.4321_f32;

    // 암시적인 변환은 안됨
    //let integer: u8 = decimal; // ERROR

    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} => {} => {}", decimal, integer, character);

    println!("1000 as a u16 is : {}", 1000 as u16);

    println!("1000 as a u8 is : {}", 1000 as u8);
    println!("-1 as a u8 is : {}", (-1i8) as u8);
    println!("1000 mod 256 is : {}", 1000 % 256);

    println!("128 as a i16 is : {}", 128 as i16);
    println!("128 as a i8 is : {}", 128 as i8);

    println!("1000 as a i8 is : {}", 1000 as i8);
    println!("232 as a i8 is : {}", 232 as i8);
}
