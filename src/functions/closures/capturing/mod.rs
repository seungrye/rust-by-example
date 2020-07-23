
pub fn main() {
    closure_usage();
    closure_take_ownership();
}

fn closure_usage() {
    use std::mem;

    let color = String::from("green");

    // A closure to print `color` which immediately borrows (`&`) `color` and stores the borrow and closure in the `print` variable.
    // It will remain borrowed until `print` is used the last time
    // `println!` only requires arguments by immutable reference so it doesn't impose anything more restrictive
    let print = || println!("`color`: {}", color);

    print();

    // `color` can be borrowed immutably again, because the closure only holds an immutable reference to `color`
    let _reborrow = &color;
    print();

    // A move or reborrow is allowed after the final use of 'print'
    let _color_moved = color;
    // print();

    let mut count = 0;
    // A closure to increment `count` could take either `&mut count` or `count` but `&mut count` is less restrictive so it takes that.
    // Immediately borrows `count`. (가 뭔말인지 잘 이해가 안되는뎀@.@?)
    // A `mut` is required on `inc` because a `&mut` is stored inside.
    // Thus, calling the closure mutates the closure which requires a `mut`.
    let mut inc = || { count += 1; println!("`count`: {}", count);};

    inc();

    // The closure still mutably borrows `count` because it is called later.
    // An attempt to reborrow will lead to an error.
    // let _reborrow = &count;
    inc();

    // A non-copy type
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value.
    // A copy type would copy into the closure leaving the original untouched.
    let consume = || { println!("`movable`: {:?}", movable); mem::drop(movable);};
    // `consume` consumes the variable so this can only be called once
    consume();
    // consume();
}

fn closure_take_ownership() {
    let haystack = vec![1, 2, 3];

    // using `move` before vertical pipes forces closure to take ownership of captured variables
    let contains = move|needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // following code will be errored, because borrow checker doesn't allow re-using variable after it has been moved.
    // println!("There are {} element in vec", haystack.len());
}