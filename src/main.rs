mod array;
mod cast;
mod constants;
mod enums;
mod flow_control;
mod fmt;
mod formatted_print;
mod functions;
mod print_debug;
mod print_display;
mod scoping_rules;
mod structs;
mod testcase_list;
mod tuples;
mod variable_bindings;
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
    // flow_of_control::loop_::nesting_and_labels::main();
    flow_of_control::loop_::returning_from_loops::main();
    flow_of_control::while_::main();
    flow_of_control::for_and_range::main();
    flow_of_control::match_::main();
    flow_of_control::if_let::main();
    flow_of_control::while_let::main();

    functions::methods::main();
    functions::closures::main();
    functions::higher_order_functions::main();

    fmt::main();
    formatted_print::main();
    print_debug::main();
    print_display::main();
    testcase_list::main();
    tuples::main();
    array::main();
    structs::main();
    enums::main();
    constants::main();
    variable_bindings::main();
    cast::main();
    // destructuring::main();
    flow_control::main();
    functions::main();
    scoping_rules::lifetimes::main();
    scoping_rules::lifetimes::explicit_annotation::main();
    scoping_rules::lifetimes::functions::main();
    scoping_rules::lifetimes::methods::main();
}
