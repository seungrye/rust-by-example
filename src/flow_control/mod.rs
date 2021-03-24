#![allow(unreachable_code)]

pub fn main() {
    if_let();
    while_let();
}
fn waste_strokes2() {
    let mut optional = Some(0);

    #[allow(clippy::while_let_loop)]
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

    #[allow(clippy::single_match)]
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


















