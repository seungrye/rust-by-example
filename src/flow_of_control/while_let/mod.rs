pub fn main() {
    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 5 {
            println!("> 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. try again", i);
            optional = Some(i + 1);
        }
    }
}