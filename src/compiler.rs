use crate::compiled::{imp, Compiled};

type WordsIter<'a> = std::slice::Iter<'a, String>;

fn next(iter: &mut WordsIter) -> Option<Compiled> {
    let word = iter.next()?;
    match word.as_str() {
        "if" => unimplemented!(),
        "begin" => unimplemented!(),
        "do" => unimplemented!(),
        // this should never happen!
        ":" | ";" => unimplemented!(),
        _ => Some(Compiled::Word(word.clone())),
    }
}

pub fn compile_function(iter: &mut WordsIter) -> Option<(String, Compiled)> {
    let name = iter.next()?;

    let mut body = Vec::new();
    while let Some(compiled) = next(iter) {
        body.push(compiled);
    }

    Some((name.to_string(), Compiled::Function(imp::Function { body })))
}
