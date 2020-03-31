#![allow(unreachable_code)]

pub fn main() {
    if_else();
    loop_();
    nesting_and_labels();
    while_();
    for_and_range();
    match_();
}
fn match_() {
    let number = 13;

    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13...19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}
fn for_and_range() {
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
fn while_() {
    let mut n = 1;

    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }
}
fn nesting_and_labels() {
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");

            // 이 break 는 외부 루프를 종결한다.
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}
fn loop_() {
    let mut count = 0;
    println!("Let's count until infinity!");

    loop {
        // 무한루프
        count += 1;

        if count == 3 {
            println!("three");
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");
            break;
        }
    }
}
fn if_else() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");
        n * 10 //; 를 적지 않은 경우, 해당 값을 반환함을 의미한다.
    } else {
        println!(", and is a big number, reduce by two");
        n / 10
    };

    println!("{} -> {}", n, big_n);
}
