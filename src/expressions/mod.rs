pub fn main() {
    let x = 5u32; // u32 type 의 숫자 5

    let y = {
        let x_squared = x*x;
        let x_cube = x_squared * x;

        x_cube + x_squared + x // assign to y. this line did NOT end with ';'
    };

    let z = {
        2 * x // assign to z. this line did NOT end with ';'
    };

    println!("x => {:?}", x);
    println!("y => {:?}", y);
    println!("z => {:?}", z);
}