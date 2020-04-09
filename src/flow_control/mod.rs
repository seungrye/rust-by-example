#![allow(unreachable_code)]

pub fn main() {
    if_else();
    loop_();
    nesting_and_labels();
    while_();
    for_and_range();
    match_();
    match_guard();
    match_binding();
    if_let();
    while_let();
}
fn waste_strokes2() {
    let mut optional = Some(0);
    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("'i' is '{:?}'. try again", i);
                    optional = Some(i + 1);
                }
            }
            _ => break, // None 에 대해 처리해야 하기 때문에 필요함. 공간낭비로 보임.
        }
    }
}
fn while_let() {
    waste_strokes2();

    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None; // Some() 이 None 이라면 while 루프 탈출
        } else {
            println!("'i' is '{:?}'. try again", i);
            optional = Some(i + 1);
        }
    }
}
fn waste_strokes() {
    let optional = Some(7);
    match optional {
        Some(i) => println!("This is a really long string and `{:?}", i),
        _ => {} // None 에 대해 처리해야 되기 때문에 필요함. 공간낭비로 보임.
    }
}
fn if_let() {
    waste_strokes();

    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // let 으로 number 를 역 구조화해서 Some(i) 에 넣고, Some 이 valid 하다면
    // {} 를 수행함.
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // valid 하지 않을 경우, else 를 태움
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    let i_like_letters = false;
    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with a emoticon :)");
    }
}
fn match_binding() {
    println!("Tell me type of person you are");
    match 15 {
        0 => println!("I am not born yet I guess"),
        // match 를 1 ... 12 까지 직접적으로 할 수 있지만, 그러면 정확히 나이를 알 수 없으므로,
        // n 에 1 ... 12 를 바인드 한다. 이제 나이를 알수 있게 되었다.
        n @ 1..=12 => println!("I am a child of age {:?}", n),
        n @ 13..=19 => println!("I am a teen of age {:?}", n),
        n => println!("I am an old person of age {:?}", n),
    }
}
fn match_guard() {
    let pair = (2, -2);
    println!("Tell me about {:?}", pair);

    match pair {
        (x, y) if x + y == 0 => println!("Antimatter, 뻥~!"),
        //               ^ 요 'if 조건문' 이 guard 부분이다.
        (x, y) if x == y => {
            println!("These are twins");
        }
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}
fn match_() {
    let number = 13;

    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
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
