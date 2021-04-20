use std::convert::TryFrom;
mod interpreter;

fn main() {
    let output = |val, val_type| {
        match val_type {
            interpreter::EventType::INTEGER => print!("{}", val),
            interpreter::EventType::STRING => print!("{}", u8::try_from(val).unwrap_or(0) as char)
        };
    };

    let mut eval = interpreter::Interpreter::new("\"olleh\",,,,,@", &output);

    eval.run();
}
