use crate::{
    errors::Error::{self, DivisionByZero, StackUnderflow},
    expressions::Expr::{self, Begin, IfElseThen, Loop, NewFunction, Word},
    forth::Forth,
    numbers::Int,
    parser::Parser,
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
#[test_case("*/", &[912345678, 34, 100], &[310197530]; "mul div")]
#[test_case("*/mod", &[912345678, 34, 100], &[52, 310197530]; "mul div rem")]
#[test_case("mod", &[5, 2], &[1]; "simple mod")]
#[test_case("/mod", &[5, 2], &[1, 2]; "simple div mod")]
#[test_case("2*", &[7], &[14]; "times two")]
#[test_case("2/", &[18], &[9]; "divide by two")]
#[test_case("1+", &[10], &[11]; "add one")]
#[test_case("1-", &[10], &[9]; "sub one")]
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
#[test_case("0=", &[0], &[-1]; "is zero for zero")]
#[test_case("0=", &[5], &[0]; "is zero for non-zero")]
#[test_case("not", &[0], &[-1]; "not true")]
#[test_case("not", &[-1], &[0]; "not false")]
#[test_case("and", &[0, 0], &[0]; "and for false false")]
#[test_case("and", &[0, -1], &[0]; "and for false true")]
#[test_case("and", &[-1, 0], &[0]; "and for true false")]
#[test_case("and", &[-1, -1], &[-1]; "and for true true")]
#[test_case("or", &[0, 0], &[0]; "or for false false")]
#[test_case("or", &[0, -1], &[-1]; "or for false true")]
#[test_case("or", &[-1, 0], &[-1]; "or for true false")]
#[test_case("or", &[-1, -1], &[-1]; "or for true true")]
#[test_case("xor", &[0, 0], &[0]; "xor for false false")]
#[test_case("xor", &[0, -1], &[-1]; "xor for false true")]
#[test_case("xor", &[-1, 0], &[-1]; "xor for true false")]
#[test_case("xor", &[-1, -1], &[0]; "xor for true true")]
#[test_case("3 pick", &[1, 2, 3, 4], &[1, 2, 3, 4, 2]; "pick")]
#[test_case("3 roll", &[1, 2, 3, 4], &[1, 3, 4, 2]; "roll")]
#[test_case("clearstack", &[1, 2, 3, 4], &[]; "clearstack")]
#[test_case("swap", &[1, 2], &[2, 1]; "simple swap")]
#[test_case("swap", &[1, 2, 3, 4], &[1, 2, 4, 3]; "swap with multiple elements on stack")]
#[test_case("dup", &[1, 2], &[1, 2, 2]; "dup")]
#[test_case("drop", &[1, 2, 3, 4], &[1, 2, 3]; "drop")]
#[test_case("rot", &[1, 2, 3, 4], &[1, 3, 4, 2]; "rot")]
#[test_case("over", &[1, 2], &[1, 2, 1]; "over")]
#[test_case("depth", &[], &[0]; "depth of empty stack")]
#[test_case("depth", &[5, 10, 18, 2], &[5, 10, 18, 2, 4]; "depth of non-empty stack")]
#[test_case("if 10 then", &[-1], &[10]; "if-then true branch")]
#[test_case("if 10 then", &[0], &[]; "if-then false branch")]
#[test_case("if 10 else 20 then", &[-1], &[10]; "if-else-then true branch")]
#[test_case("if 10 else 20 then", &[0], &[20]; "if-else-then false branch")]
#[test_case(": f 42 ; f", &[], &[42]; "trivial function")]
#[test_case(": f if 10 else 20 then ; f", &[-1], &[10]; "function with if-else-then true branch")]
#[test_case(": f if 10 else 20 then ; f", &[0], &[20]; "function with if-else-then false branch")]
#[test_case(": f dup 1 + dup 5 < if recurse then ; f", &[0], &[0, 1, 2, 3, 4, 5]; "recursion")]
#[test_case("begin 1 + dup 10 > until", &[0], &[11]; "begin until loop")]
#[test_case("begin 1 + dup 10 < while repeat", &[0], &[10]; "begin while loop")]
#[test_case("begin leave again", &[], &[]; "begin leave again")]
#[test_case("begin -1 if leave then again", &[], &[]; "conditionally leave begin again loop")]
#[test_case("begin 1 + dup 10 > if leave then again", &[0], &[11]; "begin again")]
#[test_case("do i loop", &[5, 0], &[0, 1, 2, 3, 4]; "do loop")]
#[test_case("3 0 do 2 0 do j i loop loop", &[], &[0, 0, 0, 1, 1, 0, 1, 1, 2, 0, 2, 1]; "nested do loop")]
fn eval_string(word: &str, init_stack: &[i32], expected_stack: &[i32]) {
    let expected_stack = expected_stack.iter().map(|x| Int(*x)).collect::<Vec<_>>();
    let init_stack = init_stack.iter().map(|x| Int(*x)).collect::<Vec<_>>();

    let mut forth = Forth::new(10);
    forth.data_stack = init_stack;
    assert!(forth.eval_string(word).is_ok());
    assert_eq!(expected_stack, forth.data_stack);
}

#[test_case("+"; "add")]
#[test_case("-"; "sub")]
#[test_case("*"; "mul")]
#[test_case("/"; "div")]
#[test_case("mod"; "modulo")]
#[test_case("/mod"; "mod rem")]
#[test_case("*/"; "mul div")]
#[test_case("*/mod"; "mul div mod")]
#[test_case("="; "equal")]
#[test_case("<>"; "not equal")]
#[test_case("<"; "less")]
#[test_case(">"; "greater")]
#[test_case("1+"; "add one")]
#[test_case("1-"; "sub one")]
#[test_case("not"; "not")]
#[test_case("and"; "and")]
#[test_case("or"; "or")]
#[test_case("xor"; "xor")]
#[test_case("swap"; "swap")]
#[test_case("over"; "over")]
#[test_case("pick"; "pick")]
#[test_case("roll"; "roll")]
#[test_case("!"; "set variable")]
#[test_case("dup"; "dup")]
#[test_case("drop"; "drop")]
#[test_case("@"; "fetch")]
#[test_case("negate"; "negate")]
#[test_case("."; "dot")]
#[test_case("."; "emit")]
fn underflow_for_empty_stack(word: &str) {
    let mut forth = Forth::new(10);
    forth.data_stack.clear();
    assert_eq!(forth.eval_string(word), Err(StackUnderflow), "empty stack");
}

#[test_case("+"; "add")]
#[test_case("-"; "sub")]
#[test_case("*"; "mul")]
#[test_case("/"; "div")]
#[test_case("*/"; "mul div")]
#[test_case("*/mod"; "mul div mod")]
#[test_case("mod"; "modulo")]
#[test_case("/mod"; "mod rem")]
#[test_case("="; "equal")]
#[test_case("<>"; "not equal")]
#[test_case("<"; "less")]
#[test_case(">"; "greater")]
#[test_case("and"; "and")]
#[test_case("or"; "or")]
#[test_case("xor"; "xor")]
#[test_case("swap"; "swap")]
#[test_case("over"; "over")]
#[test_case("1 pick"; "pick")]
#[test_case("1 roll"; "roll")]
#[test_case("!"; "set variable")]
fn underflow_for_one_value_on_stack(word: &str) {
    let mut forth = Forth::new(10);
    forth.data_stack = vec![Int(1)];
    assert_eq!(forth.eval_string(word), Err(StackUnderflow),);
}

#[test_case("*/"; "mul div")]
#[test_case("*/mod"; "mul div mod")]
#[test_case("2 pick"; "pick")]
#[test_case("2 roll"; "roll")]
fn underflow_for_two_value_on_stack(word: &str) {
    let mut forth = Forth::new(10);
    forth.data_stack = vec![Int(1), Int(2)];
    assert_eq!(forth.eval_string(word), Err(StackUnderflow),);
}

#[test_case("1 0 /", DivisionByZero; "div division by zero")]
#[test_case("1 0 mod", DivisionByZero; "mod division by zero")]
#[test_case("1 0 /mod", DivisionByZero; "div mod division by zero")]
#[test_case("1 2 0 */", DivisionByZero; "mul div division by zero")]
#[test_case("1 2 0 */mod", DivisionByZero; "mul div mod division by zero")]
#[test_case("-1 if 1 0 / then", DivisionByZero; "if-then propagates errors")]
#[test_case("-1 if 1 0 / else 0 then", DivisionByZero; "if-then-else propagates errors on true branch")]
#[test_case("0 if 0 else 1 0 / then", DivisionByZero; "if-then-else propagates errors on false branch")]
#[test_case(": f 1 0 / . 2 2 + ; f", DivisionByZero; "function propagates errors")]
#[test_case("begin 1 0 / again", DivisionByZero; "begin loop propagates errors")]
#[test_case("1 2 */", StackUnderflow; "mul div not enough elements")]
#[test_case("1 2 */mod", StackUnderflow; "mul div mod not enough elements")]
fn errors(code: &str, err: Error) {
    let mut forth = Forth::new(10);
    assert_eq!(forth.eval_string(code), Err(err));
}

#[test]
fn constants() {
    use crate::expressions::Expr::Constant;

    let mut forth = Forth::new(10);

    assert!(forth.get_word("x").is_none());
    assert!(forth.eval_string("42 constant x").is_ok());
    assert_eq!(Some(Constant(Int(42))), forth.get_word("x"));

    assert!(forth.get_word("y").is_none());
    assert!(forth.eval_string("123 constant y").is_ok());
    assert_eq!(Some(Constant(Int(123))), forth.get_word("y"));

    assert_eq!(
        Err(Error::Redefined("x".into())),
        forth.eval_string("0 constant x"),
        "errors on redefinition"
    );
}

#[test]
fn variables() {
    let mut forth = Forth::new(10);

    assert!(forth.eval_string("variable x").is_ok());
    assert_eq!(forth.data_stack, vec![]);
    assert!(forth.eval_string("5 x !").is_ok());
    assert!(forth.eval_string("x @").is_ok());
    assert_eq!(forth.data_stack, vec![Int(5)]);

    assert!(forth.eval_string("7 x !").is_ok());
    assert!(forth.eval_string("x @").is_ok());
    assert_eq!(forth.data_stack, vec![Int(5), Int(7)]);

    assert!(forth.eval_string("17 y !").is_err());
}

#[test]
fn return_stack() {
    let mut forth = Forth::new(10);

    assert!(forth.eval_string("42 >r").is_ok());
    assert_eq!(forth.data_stack, &[]);
    assert_eq!(forth.return_stack, &[Int(42)]);

    assert!(forth.eval_string("r@").is_ok());
    assert_eq!(forth.data_stack, &[Int(42)]);
    assert_eq!(forth.return_stack, &[Int(42)]);

    assert!(forth.eval_string("r>").is_ok());
    assert_eq!(forth.data_stack, &[Int(42), Int(42)]);
    assert_eq!(forth.return_stack, &[]);
}

#[test_case(
        "",
        &[];
        "nothing"
    )]
#[test_case(
        "hello",
        &[Word("hello".into())];
        "just a word"
    )]
#[test_case(
        "HELLO",
        &[Word("hello".into())];
        "just a word uppercase"
    )]
#[test_case(
        " \t\t hello",
        &[Word("hello".into())];
        "spaces before word"
    )]
#[test_case(
        "hello \t\t  ",
        &[Word("hello".into())];
        "spaces after word"
    )]
#[test_case(
        " : foo ; ",
        &[NewFunction("foo".into(), vec![])];
        "empty function"
    )]
#[test_case(
        " : foo bar 2 + ; ",
        &[NewFunction(
            "foo".into(),
            vec![Word("bar".into()), Word("2".into()), Word("+".into())]
        )];
        "some function"
    )]
#[test_case(
        " : foo ( n1 n2 -- n3 ) bar 2 + ; ",
        &[NewFunction(
            "foo".into(),
            vec![Word("bar".into()), Word("2".into()), Word("+".into())]
        )];
        "some function with a comment"
    )]
#[test_case(
        " : FOO BAR 2 + ; ",
        &[NewFunction(
            "foo".into(),
            vec![Word("bar".into()), Word("2".into()), Word("+".into())]
        )];
        "some function uppercase"
    )]
#[test_case(
        " if then ",
        &[IfElseThen(vec![], vec![])];
        "empty if else then block"
    )]
#[test_case(
        " if yes . then ",
        &[IfElseThen(vec![Word("yes".into()), Word(".".into())], vec![])];
        "if then block"
    )]
#[test_case(
        " if yes + else no - . then ",
        &[IfElseThen(
            vec![Word("yes".into()), Word("+".into())],
            vec![Word("no".into()), Word("-".into()), Word(".".into())]
        )];
        "if else then block"
    )]
#[test_case(
        " IF YES + ELSE NO - . THEN ",
        &[IfElseThen(
            vec![Word("yes".into()), Word("+".into())],
            vec![Word("no".into()), Word("-".into()), Word(".".into())]
        )];
        "if else then block uppercase"
    )]
#[test_case(
        " do loop ",
        &[Loop(vec![])];
        "empty do loop"
    )]
#[test_case(
        " 5 0 do i . loop ",
        &[
            Word("5".into()), Word("0".into()),
            Loop(vec![Word("i".into()), Word(".".into())])
        ];
        "do loop"
    )]
#[test_case(
        " 5 0 DO I . LOOP ",
        &[
            Word("5".into()), Word("0".into()),
            Loop(vec![Word("i".into()), Word(".".into())])
        ];
        "do loop uppercase"
    )]
#[test_case(
        " begin again ",
        &[Begin(vec![])];
        "empty begin again"
    )]
#[test_case(
        " begin until ",
        &[Begin(vec![Word("until".into())])];
        "empty begin until"
    )]
#[test_case(
        " begin while repeat ",
        &[Begin(vec![Word("while".into())])];
        "empty begin while"
    )]
#[test_case(
        "hello ( this is a comment ) world",
        &[Word("hello".into()), Word("world".into())];
        "skip comment in the middle"
    )]
fn parsing(input: &str, expected: &[Expr]) {
    let parser = Parser::from(input);
    let result: Result<Vec<Expr>, Error> = parser.collect();
    assert_eq!(result.unwrap(), expected);
}

#[test_case(": foo bar"; "unclosed function")]
#[test_case("if 2 +"; "unclosed if")]
#[test_case("if 2 + else 3 -"; "unclosed if else")]
#[test_case("begin foo bar"; "unclosed begin")]
#[test_case("begin foo while bar"; "unclosed begin while")]
#[test_case("do i . 2 +"; "unclosed do")]
#[test_case("include"; "include without continuation")]
#[test_case("variable"; "variable without continuation")]
#[test_case("constant"; "constant without continuation")]
#[test_case(".\" hello, world!"; "unclosed string")]
#[test_case(".( hello, world!"; "unclosed instant print")]
#[test_case("( foo bar baz"; "unclosed comment")]
fn parsing_errors(input: &str) {
    let parser = Parser::from(input);
    let result: Result<Vec<Expr>, Error> = parser.collect();
    assert!(result.is_err());
}
