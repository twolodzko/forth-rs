use crate::errors::Error;
use crate::forth::{Forth, Int};
use test_case::test_case;

#[test_case("0", &[], &[0]; "zero")]
#[test_case("42", &[], &[42]; "number")]
#[test_case("+", &[2, 2], &[4]; "simple add")]
fn execute_happy_paths(word: &str, init_stack: &[Int], expected_stack: &[Int]) {
    let mut forth = Forth::new(10);
    forth.stack = init_stack.to_vec();

    forth.execute(word).expect("failed to execute");
    assert_eq!(expected_stack, forth.stack);
}

#[test_case("+", &[], Error::StackUnderflow; "add with empty stack")]
#[test_case("+", &[2], Error::StackUnderflow; "add with one value on stack")]
fn execute_unhappy_paths(word: &str, init_stack: &[Int], error_message: Error) {
    let mut forth = Forth::new(10);
    forth.stack = init_stack.to_vec();

    assert_eq!(Err(error_message), forth.execute(word));
}
