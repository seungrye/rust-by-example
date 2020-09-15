mod capturing;
mod as_input_parameters;
mod type_anonymity;
mod input_functions;
mod as_output_parameters;
mod examples_in_std;

pub fn main() {
    fn function (i:i32) -> i32 { i + 1 }

    let closure_annotated = |i : i32| -> i32 { i + 1 };
    let closure_inferred =  |i| i + 1;  // i 의 타입을 추론을 통해 알아냄. 1줄짜리는 {} 쓰지 않을 수 있음.

    let i = 1;

    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    let one = || 1; // clsoure with no arguments. 리턴타입은 추론을 통해서 (i32) 타입이 반환됨.
    println!("closure returning one: {}", one());

    //--------------------------------------------------------------------------
    capturing::main();
    as_input_parameters::main();
    type_anonymity::main();
    input_functions::main();
    as_output_parameters::main();
    examples_in_std::main();
}