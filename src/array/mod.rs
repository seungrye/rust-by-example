use std::mem;

pub fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    println!("array size: {}", xs.len());

    println!("array occupies {} bytes", mem::size_of_val(&xs));
    println!("borrow the whole array as a slice");
    analyze_slice(&ys[1..4]);
    // println!("{}", xs[5]); // 색인이 범위를 넘어가면 panic으로 넘어간다.
}

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}
