mod types;
mod conversion;

fn main() {
    types::casting::main();
    types::literals::main();
    types::inference::main();
    types::aliasing::main();

    conversion::from_and_into::main();
    conversion::tryfrom_and_tryinto::main();
    conversion::to_and_from_strings::main();
}
