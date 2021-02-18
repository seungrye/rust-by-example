
pub fn main() {
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // imperative approach
    // declare accumulator variable
    let mut acc = 0;
    for n in 0.. {  // iterate: 0, 1, 2, ... to infinity
        // square the number
        let n_squared = n*n;
        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("imperative style : {}", acc);

    let sum_of_squared_odd_numbers = (0..).map(|n| n*n) // all natural numbers squared
        .take_while(|&n_suaqred| n_suaqred < upper) // below upper limit
        .filter(|&n_squared| is_odd(n_squared)) // take odder
        .fold(0, |acc, n_squared| acc + n_squared); // sum
    println!("functional style: {}", sum_of_squared_odd_numbers);
}

#[allow(clippy::needless_return)]
fn is_odd(n: i32) -> bool {
    return n % 2 == 1;
}