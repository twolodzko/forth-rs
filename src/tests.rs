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
#[test_case("-", &[5, 2], &[3]; "simple sub")]
#[test_case("*", &[7, 2], &[14]; "simple mul")]
#[test_case("/", &[15, 3], &[5]; "simple div")]
#[test_case("mod", &[5, 2], &[1]; "simple mod")]
#[test_case("/mod", &[5, 2], &[1, 2]; "simple div mod")]
#[test_case("abs", &[9], &[9]; "abs of positive number")]
#[test_case("abs", &[-9], &[9]; "abs of negative number")]
#[test_case("negate", &[9], &[-9]; "negate positive number")]
#[test_case("negate", &[-9], &[9]; "negate negative number")]
#[test_case("=", &[0, 0], &[-1]; "equal for zeros")]
#[test_case("=", &[5, 5], &[-1]; "equal for equal")]
#[test_case("=", &[5, -5], &[0]; "equal for nonequal")]
#[test_case("<>", &[0, 0], &[0]; "not equal for zeros")]
#[test_case("<>", &[5, 5], &[0]; "not equal for equal")]
#[test_case("<>", &[5, -5], &[-1]; "not equal for nonequal")]
#[test_case("<", &[1, 2], &[-1]; "less is true")]
#[test_case("<", &[2, 1], &[0]; "less is false")]
#[test_case("<", &[1, 1], &[0]; "less for equal")]
#[test_case(">", &[2, 1], &[-1]; "greater is true")]
#[test_case(">", &[1, 2], &[0]; "greater is false")]
#[test_case(">", &[1, 1], &[0]; "greater for equal")]
#[test_case("and", &[0, 0], &[0]; "and for false false")]
#[test_case("and", &[0, -1], &[0]; "and for false true")]
#[test_case("and", &[-1, 0], &[0]; "and for true false")]
#[test_case("and", &[-1, -1], &[-1]; "and for true true")]
#[test_case("or", &[0, 0], &[0]; "or for false false")]
#[test_case("or", &[0, -1], &[-1]; "or for false true")]
#[test_case("or", &[-1, 0], &[-1]; "or for true false")]
#[test_case("or", &[-1, -1], &[-1]; "or for true true")]
#[test_case("swap", &[1, 2], &[2, 1]; "simple swap")]
#[test_case("swap", &[1, 2, 3, 4], &[1, 2, 4, 3]; "swap with multiple elements on stack")]
#[test_case("dup", &[1, 2], &[1, 2, 2]; "dup")]
#[test_case("drop", &[1, 2, 3, 4], &[1, 2, 3]; "drop")]
#[test_case("rot", &[1, 2, 3, 4], &[1, 3, 4, 2]; "rot")]
#[test_case("over", &[1, 2], &[1, 2, 1]; "over")]
fn eval(word: &str, init_stack: &[Int], expected_stack: &[Int]) {
    let mut forth = Forth::new(10);
    forth.stack = init_stack.to_vec();
    assert!(forth.eval_string(word).is_ok());
    assert_eq!(expected_stack, forth.stack);
}

#[test_case("+"; "add")]
#[test_case("-"; "sub")]
#[test_case("*"; "mul")]
#[test_case("/"; "div")]
#[test_case("mod"; "modulo")]
#[test_case("/mod"; "mod rem")]
#[test_case("="; "equal")]
#[test_case("<>"; "not equal")]
#[test_case("<"; "less")]
#[test_case(">"; "greater")]
#[test_case("and"; "and")]
#[test_case("or"; "or")]
#[test_case("swap"; "swap")]
#[test_case("over"; "over")]
#[test_case("!"; "set variable")]
fn underflow_for_one_value_on_stack(word: &str) {
    let mut forth = Forth::new(10);
    forth.stack = vec![1];
    assert_eq!(forth.eval_string(word), Err(StackUnderflow),);
}

#[test_case("+"; "add")]
#[test_case("-"; "sub")]
#[test_case("*"; "mul")]
#[test_case("/"; "div")]
#[test_case("mod"; "modulo")]
#[test_case("/mod"; "mod rem")]
#[test_case("="; "equal")]
#[test_case("<>"; "not equal")]
#[test_case("<"; "less")]
#[test_case(">"; "greater")]
#[test_case("and"; "and")]
#[test_case("or"; "or")]
#[test_case("swap"; "swap")]
#[test_case("over"; "over")]
#[test_case("!"; "set variable")]
#[test_case("dup"; "dup")]
#[test_case("drop"; "drop")]
#[test_case("@"; "fetch")]
#[test_case("negate"; "negate")]
#[test_case("."; "dot")]
#[test_case("."; "emit")]
fn underflow_for_empty_stack(word: &str) {
    let mut forth = Forth::new(10);
    forth.stack.clear();
    assert_eq!(forth.eval_string(word), Err(StackUnderflow), "empty stack");
}

#[test]
fn constants() {
    use crate::expressions::Expr::Constant;

    let mut forth = Forth::new(10);

    assert!(forth.get_word("x").is_none());
    assert!(forth.eval_string("42 constant x").is_ok());
    assert_eq!(Some(Constant(42)), forth.get_word("x"));

    assert!(forth.get_word("y").is_none());
    assert!(forth.eval_string("123 constant y").is_ok());
    assert_eq!(Some(Constant(123)), forth.get_word("y"));

    assert_eq!(
        Err(Error::Redefined("x".to_string())),
        forth.eval_string("0 constant x"),
        "errors on redefinition"
    );
}

#[test]
fn variables() {
    let mut forth = Forth::new(10);

    assert!(forth.eval_string("variable x").is_ok());
    assert!(forth.eval_string("5 x !").is_ok());
    assert!(forth.eval_string("x @").is_ok());
    assert_eq!(forth.stack, vec![5]);

    assert!(forth.eval_string("7 x !").is_ok());
    assert!(forth.eval_string("x @").is_ok());
    assert_eq!(forth.stack, vec![5, 7]);

    assert!(forth.eval_string("17 y !").is_err());
}
