pub fn main() {
    let mut count = 0;
    println!("Let's count until infinity");

    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("Ok, that's enough");
            break;
        }
    }
}

pub mod nesting_and_labels {
    #![allow(unreachable_code)]
    #![allow(unused_labels)]
    pub fn main() {
        'outer: loop {
            println!("Entered the outer loop");

            'inner: loop {
                println!("Entered the inner loop");

                //break;  // this would break only the inner loop
                break 'outer;  // this breaks the outer loop
            }

            println!("This point will never be reached");
        }

        println!("Exited the outer loop");
    }
}

pub mod returning_from_loops {
    /// break 다음에 반환할 값을 넣으면, loop 구문의 반환값으로 처리됩니다.
    pub fn main() {
        let mut counter = 0;
        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        assert_eq!(result, 20);
    }
}