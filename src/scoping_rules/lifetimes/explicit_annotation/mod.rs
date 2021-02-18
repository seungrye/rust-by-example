fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

fn failed_borrow<'a>() {
    let _x = 12;

    // let y: &'a i32 = &_x;
    // Note. _x 변수가 함수가 y 참조(빌림) 보다 더 먼저
    // 파괴(y 는 함수와 생명주기가 같음)되므로
    // lifetime 문제가 발생합니다. (원본이 참조보다 먼저 사라지면 안됨)
}
pub fn main() {
    let (nine, seven) = (9, 7);
    print_refs(&nine, &seven);

    failed_borrow();
}
