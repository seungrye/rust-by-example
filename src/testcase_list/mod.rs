// mod types;
// mod conversion;
// mod expressions;
// mod flow_of_control;
// mod functions;

// fn main() {
//     types::casting::main();
//     types::literals::main();
//     types::inference::main();
//     types::aliasing::main();

//     conversion::from_and_into::main();
//     conversion::tryfrom_and_tryinto::main();
//     conversion::to_and_from_strings::main();

//     expressions::main();

//     flow_of_control::if_else::main();
//     flow_of_control::loop_::main();
//     flow_of_control::loop_::nesting_and_labels::main();
//     flow_of_control::loop_::returning_from_loops::main();
//     flow_of_control::while_::main();
//     flow_of_control::for_and_range::main();
//     flow_of_control::match_::main();
//     flow_of_control::if_let::main();
//     flow_of_control::while_let::main();

//     functions::methods::main();
//     functions::closures::main();
//     functions::higher_order_functions::main();
// }

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
