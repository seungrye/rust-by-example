fn if_let_usage() {
    let number = Some(7);
    let letter:Option<i32> = None;
    let emoticon:Option<i32> = None;

    // if 'let' desstructures 'number' into 'Some(i)', evaluate the block ('{}')
    if let Some(i) = number {
        println!("Matched {:?}", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        // destructure failed.
        println!("Didn't match a number, Let's go with a letter");
    }

    let i_like_letter = false;
    if let Some(i) = emoticon {
        println!("Matched {:?}", i);
    } else if i_like_letter {
        println!("Didn't match a number. Let's go with a letter");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)");
    }
}

enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn if_let_on_enum() {
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    }

    if let Foo::Bar = b {
        println!("b is foobar");
    }

    // if Foo::Bar == b {  // 요렇게 비교하려면, enum Foo 에 PartialEq 를 derive 해야 함.
    //     println!("b is also foobar");
    // }

    // variable c 는 Foo::Qux 와 매치됩니다.
    // Some() 과 같이, c 가 가지고 있는 값은 value 에 destructuring 됩니다.
    if let Foo::Qux(value) = c {
        println!("c is {:?}", value);
    }

    // c 가 가지고 있는 값이 100 인 경우에만 if condition 을 타도록 처리함.
    // c 가 가지고 있는 값(100) 은, value 에 복사됩니다. (소유권을 전달하지는 않는듯)
    if let Foo::Qux(value@100) = c {
        println!("c is one hundred => {:?}", value);
    }
}

pub fn main() {
    if_let_usage();
    if_let_on_enum();
}