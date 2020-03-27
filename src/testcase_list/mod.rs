use std::fmt;
pub fn main() {
    let v = List(vec![1, 2, 3]);

    println!("{}", v);
}

struct List(Vec<i32>);
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        let List(ref vec) = *self; // 문법적 의미를 모르겠음;

        for (index, v) in vec.iter().enumerate() {
            if index != 0 {
                write!(f, ", ")?;
            }

            write!(f, "{}: {}", index, v)?;
        }
        return write!(f, "]");
    }
}
