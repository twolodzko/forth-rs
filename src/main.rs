use forth_rs::Forth;
use rustyline::{error::ReadlineError, DefaultEditor, Result};

fn main() -> Result<()> {
    let mut forth = Forth::new(1024);

    println!("Press ^C to exit.\n");

    let mut rl = DefaultEditor::new()?;
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                match forth.eval_string(&line) {
                    Ok(_) => println!("ok"),
                    Err(msg) => println!("error: {}", msg),
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
    Ok(())
}
