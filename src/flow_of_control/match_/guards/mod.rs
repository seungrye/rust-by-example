
pub fn main() {
    let pair = (2, -2);

    println!("Tell me about {:?}", pair);

    // a match guard can be added to filter the arm
    // Note. 두가지 상황에 다 match 되더라도, 첫번째 걸리는 컨디션의 것을 실행함.
    //       pair 가 (0, 0) 인 경우, 두가지 condition 을 모두 만족하지만, 'These are twins' 만 출력함.
    match pair {
        (x, y) if x == y => println!("These are twins"),  // `if confition` part is a guard
        (x, y) if x + y == 0 => println!("Animatter, boooooooooooooooom!!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No Correlation..."),
    }
}