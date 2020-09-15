pub fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`. destructure to `i32`
    println!("2 in vec1: {}", vec1.iter().any(|&x|x == 2));

    // `into_iter()` for vecs yields `i32`. No destructuring required.
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1, 2, 3];
    let _array2 = [4, 5, 6];

    // `iter()` for arrays yield `&i32`
    println!("2 in array1: {}", array1.iter().any(|&x|x == 2));

    // `into_iter()` for arrays unusually yield `&i32`
    // Note. array 의 .into_iter() 는 사용하지 않는것을 권장함. .iter() 를 대신 사용하도록 합니다.
    // println!("2 in array2: {}", _array2.into_iter().any(|&x| x == 2));
}