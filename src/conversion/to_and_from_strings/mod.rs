mod converting_to_string {
    use std::fmt;
    use std::fmt::{Formatter, Result};

    struct Circle {
        radius: i32
    }

    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            return write!(f, "Circle of radius {}", self.radius);
        }
    }

    pub fn main() {
        let circle = Circle{ radius: 6 };
        println!("{}", circle.to_string());
    }
}

mod parsing_a_string {
    pub fn main() {
        // 명시적으로 parse 타입을 명시하지 않았다면, 반환받는 변수(parsed) 의 타입을 명시해야 합니다.
        let parsed:i32 = "5".parse().unwrap();
        let turbo_parsed = "10".parse::<i32>().unwrap();

        let sum = parsed + turbo_parsed;
        println!("Sum : {:?}", sum);
    }
}
pub fn main() {
    converting_to_string::main();
    parsing_a_string::main();
}