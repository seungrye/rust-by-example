mod array;
mod cast;
mod constants;
mod enums;
mod flow_control;
mod fmt;
mod formatted_print;
mod print_debug;
mod print_display;
mod structs;
mod testcase_list;
mod tuples;
mod variable_bindings;
fn main() {
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
    flow_control::main();
}
