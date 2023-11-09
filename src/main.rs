use forth_rs::Forth;
use rustyline::{error::ReadlineError, DefaultEditor};

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
    let mut forth = Forth::new(1024);

    repl(&mut forth);
}
