mod types;
mod conversion;
mod expressions;
mod flow_of_control;

fn main() {
    types::casting::main();
    types::literals::main();
    types::inference::main();
    types::aliasing::main();

    conversion::from_and_into::main();
    conversion::tryfrom_and_tryinto::main();
    conversion::to_and_from_strings::main();

    expressions::main();

    flow_of_control::if_else::main();
    flow_of_control::loop_::main();
    flow_of_control::loop_::nesting_and_labels::main();
    flow_of_control::loop_::returning_from_loops::main();
    flow_of_control::while_::main();
    flow_of_control::for_and_range::main();
    flow_of_control::match_::main();
}
