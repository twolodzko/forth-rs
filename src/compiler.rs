use crate::compiled::{
    imp,
    Compiled::{self, Function, IfThenElse, Word},
};

type WordsIter<'a> = std::slice::Iter<'a, String>;

fn compile_ite(iter: &mut WordsIter) -> Option<Compiled> {
    let mut acc = Vec::new();
    let mut then = Vec::new();

    while let Some(compiled) = next(iter) {
        if let Compiled::Word(word) = &compiled {
            match word.as_str() {
                "then" => {
                    then = acc.clone();
                    acc = Vec::new();
                    continue;
                }
                "else" => break,
                _ => (),
            }
        }
        acc.push(compiled);
    }
    let otherwise = acc;

    Some(IfThenElse(imp::IfThenElse { then, otherwise }))
}

fn next(iter: &mut WordsIter) -> Option<Compiled> {
    let word = iter.next()?;
    match word.as_str() {
        "if" => compile_ite(iter),
        "begin" => unimplemented!(),
        "do" => unimplemented!(),
        _ => Some(Word(word.clone())),
    }
}

pub fn compile_function(iter: &mut WordsIter) -> Option<(String, Compiled)> {
    let name = iter.next()?;

    let mut body = Vec::new();
    while let Some(compiled) = next(iter) {
        body.push(compiled);
    }

    Some((name.clone(), Function(imp::Function { body })))
}
