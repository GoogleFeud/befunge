use std::convert::TryFrom;
use std::fs;
mod interpreter;

fn main() {
    let output = |val, val_type| {
        match val_type {
            interpreter::EventType::INTEGER => print!("{}", val),
            interpreter::EventType::STRING => print!("{}", u8::try_from(val).unwrap_or(0) as char)
        };
    };

    let code = fs::read_to_string("test.bf").expect("Could not find file test.bf");

    let mut eval = interpreter::Interpreter::new(&code, &output);

    eval.run();
}
