use forth_rs::Forth;
use rustyline::{error::ReadlineError, DefaultEditor};
use std::env;

fn repl(forth: &mut Forth) {
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
            Err(err) => {
                println!("error: {:?}", err);
                break;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut forth = Forth::new(1024);

    if args.len() < 2 {
        repl(&mut forth);
        return;
    }

    if &args[1] == "-h" || &args[1] == "--help" {
        println!(
            "Usage: {} [FILE]...\n\nIf no files are given, opens REPL.",
            args[0]
        );
        return;
    }

    if let Err(msg) = forth.eval_file(&args[1]) {
        panic!("{}", msg);
    }
}
