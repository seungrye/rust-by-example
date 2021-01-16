#![allow(dead_code)]

mod linked_list;

fn enum_example() {
    let person = Person::Height(18);
    let amira = Person::Weight(10);
    let dave = Person::Info {
        name: "Dave".to_owned(), // cloning 해서 복사본을 생성함.
        height: 72,
    };

    let rebecca = Person::Scientist;
    let rohan = Person::Engineer;

    inspect(person);
    inspect(amira);
    inspect(dave);
    inspect(rebecca);
    inspect(rohan);
}

fn enum_linked_list_example() {
    let mut list = linked_list::List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length : {}", list.len());
    println!("{}", list.stringify());
}
pub fn main() {
    enum_example();
    enum_use_example();
    enum_c_like_example();
    enum_linked_list_example();
}

fn inspect(person: Person) {
    match person {
        Person::Engineer => println!("Is an engineer!"),
        Person::Scientist => println!("Is a scientist!"),
        Person::Height(i) => println!("Has a height of {}.", i), // 역 구조화
        Person::Weight(i) => println!("Has a weight of {}.", i), // 역 구조화
        Person::Info { name, height } => println!("{} is {} tall!", name, height), // Info 를 name 과 height 로 역 구조화
    }
}

enum Person {
    Engineer,
    Scientist,
    Height(i32),
    Weight(i32),
    Info { name: String, height: i32 },
}

fn enum_use_example() {
    use Status::{Poor, Rich};
    use Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}

enum Status {
    Rich,
    Poor,
}
enum Work {
    Civilian,
    Soldier,
}

fn enum_c_like_example() {
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}

enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}
