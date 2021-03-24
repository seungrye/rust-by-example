
// range notation `a..b` is yield values from a(inclusive) to b(exclusive) in steps
fn for_and_range() {
    for n in 1..101 { // from 1 to 100 (exclude 101). You can write `for n in 1..=100 {` as alternatives
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
    println!();
}


fn for_and_iterators_using_iter() {
    let names = vec!["Bob", "Frank", "Ferris"];

    // iter : This borrows each element of the collection through each iteration.
    //        Thus leaving the collection untouched and available for reuse after the loop.
    for name in names.iter() {
        #[allow(clippy::match_ref_pats)]
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name)
        }
    }

    println!("names: {:?}", names);
}

fn for_and_iterators_using_into_iter() {
    let names = vec!["Bob", "Frank", "Ferris"];

    // into_iter - This consumes the collection so that on each iteration the exact data is provided.
    //             Once the collection has been consumed it is no longer available for reuse as it has been 'moved'
    //             within the loop.
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name)
        }
    }
}
fn for_and_iterators_using_iter_mut() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    // iter_mut - This mutably borrows each element of the collection,
    //            allowing for the collection to be modified in place.
    for name in names.iter_mut() {
        #[allow(clippy::match_ref_pats)]
        match name {
            &mut "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name)
        }
    }

    println!("names: {:?}", names);
}
pub fn main() {
    for_and_range();
    for_and_iterators_using_iter();
    for_and_iterators_using_into_iter();
    for_and_iterators_using_iter_mut();
}