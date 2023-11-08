use crate::{
    errors::Error::{self, StackUnderflow},
    expressions::Int,
    forth::Forth,
};
use test_case::test_case;

#[test_case("0", &[], &[0]; "zero")]
#[test_case("42", &[], &[42]; "number")]
#[test_case("true", &[], &[-1]; "true word")]
#[test_case("false", &[], &[0]; "false word")]
#[test_case("+", &[2, 2], &[4]; "simple add")]
#[test_case("swap", &[1, 2], &[2, 1]; "simple swap")]
#[test_case("swap", &[1, 2, 3, 4], &[1, 2, 4, 3]; "swap with multiple elements on stack")]
#[test_case("dup", &[1, 2], &[1, 2, 2]; "dup")]
#[test_case("drop", &[1, 2, 3, 4], &[1, 2, 3]; "drop")]
fn eval_happy_path(word: &str, init_stack: &[Int], expected_stack: &[Int]) {
    let mut forth = Forth::new(10);
    forth.stack = init_stack.to_vec();

    forth.eval_string(word).expect("failed to execute");
    assert_eq!(expected_stack, forth.stack);
}

#[test_case("+", &[], StackUnderflow; "add with empty stack")]
#[test_case("+", &[2], StackUnderflow; "add with one value on stack")]
#[test_case("swap", &[], StackUnderflow; "swap with empty stack")]
#[test_case("dup", &[], StackUnderflow; "dup with empty stack")]
#[test_case("drop", &[], StackUnderflow; "drop with empty stack")]
// #[test_case("if", &[], CompileOnlyWord(String::from("if")); "if outside of compile")]
// #[test_case("do", &[], CompileOnlyWord(String::from("do")); "do outside of compile")]
// #[test_case("begin", &[], CompileOnlyWord(String::from("begin")); "begin outside of compile")]
fn eval_unhappy_path(word: &str, init_stack: &[Int], error_message: Error) {
    let mut forth = Forth::new(10);
    forth.stack = init_stack.to_vec();

    assert_eq!(Err(error_message), forth.eval_string(word));
}

// #[test]
// fn simple_function() {
//     use crate::compiled::Function;
//     let mut forth = Memory::new(10);

//     // : add2 2 + ;
//     forth.dictionary.insert(
//         "add2".to_string(),
//         Definition::Function(Function::new(&["2", "+"])),
//     );
//     // 3 add2
//     forth.stack = vec![3];
//     forth.execute("add2").expect("failed to execute");

//     assert_eq!(vec![5], forth.stack);
// }
