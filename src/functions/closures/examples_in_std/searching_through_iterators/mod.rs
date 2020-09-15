pub fn main() {
    iter_find();
    iter_position();
}

// if you want the index of the item, use Iterator::position.
fn iter_position() {
    let vec = vec![1,9, 3, 3, 13, 2];

    let index_of_first_even_number = vec.iter().position(|&x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(5));

    let index_of_first_negative_number = vec.iter().position(|&x| x < 0);
    assert_eq!(index_of_first_negative_number, None);
}

fn iter_find(){
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`
    let mut iter = vec1.iter();
    // `into_iter()` for vecs yields `i32`
    let mut into_iter = vec2.into_iter();

    // `iter()` for vecs yields `&i32`, and we want to reference one of its items,
    // so we have to destructure `&&i32` to `i32`
    println!("find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    // `into_iiter()` for vecs yields `i32`, and we want to reference one of its items,
    // so we have to destructure `&i32` to `i32`
    println!("find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let _array2 = [4, 5, 6];

    println!("find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    // Note. array 의 .into_iter() 는 사용하지 않는것을 권장함. .iter() 를 대신 사용하도록 합니다.
    // `into_iter()` for arrays unusually yields `&i32`
    // println!("find 2 in array2: {:?}", _array2.into_iter().find(|&&x| x == 2));

}