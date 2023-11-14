use forth_rs::Forth;
use std::env;

macro_rules! die {
    ( $msg:expr ) => {{
        eprintln!("error: {:?}", $msg);
        std::process::exit(1);
    }};
}

#[cfg(feature = "repl")]
fn repl(forth: &mut Forth) {
    use rustyline::{error::ReadlineError, DefaultEditor};

    println!("Press ^C to exit.\n");

    let mut rl = DefaultEditor::new().unwrap();
    loop {
        match rl.readline("> ") {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                match forth.eval_string(&line) {
                    Ok(_) => println!(" ok"),
                    Err(msg) => println!(" error: {}", msg),
                }
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                break;
            }
            Err(err) => die!(err),
        }
    }
}

fn print_help(args: Vec<String>) {
    println!("Usage: {} [FILE]...", args[0]);
    #[cfg(feature = "repl")]
    println!("\n\nIf no files are given, opens REPL.");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut forth = Forth::new(1024);

    #[cfg(feature = "repl")]
    if args.len() < 2 {
        repl(&mut forth);
        return;
    }

    if args.len() < 2 || &args[1] == "-h" || &args[1] == "--help" {
        print_help(args);
        return;
    }

    for path in &args[1..] {
        if let Err(err) = forth.eval_file(path) {
            die!(err);
        }
    }
}
