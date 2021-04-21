use std::convert::TryFrom;
use std::fs;
use std::io;
use std::env;
mod interpreter;

fn main() {
    let output = |val, val_type| {
        match val_type {
            interpreter::EventType::INTEGER => print!("{} ", val),
            interpreter::EventType::STRING => print!("{}", u8::try_from(val).unwrap_or(0) as char)
        };
    };

    let input = |interp: &mut interpreter::Interpreter, val_type| {
        match val_type {
            interpreter::EventType::INTEGER => {
                let mut res = String::new(); 
                io::stdin().read_line(&mut res).expect("Failed to read int");
                interp.stack.push(res.trim().parse::<i64>().expect("Invalid integer pvovided"));
            }
            interpreter::EventType::STRING => {
                let mut res = String::new(); 
                io::stdin().read_line(&mut res).expect("Failed to read string");
                for character in res.chars() {
                    interp.stack.push(character as i64);
                }
            }
        }
    };

    let filename = env::args().nth(1).expect("Please provide a file to run.");

    let code = fs::read_to_string(filename).expect("Could not find file");
    let mut eval = interpreter::Interpreter::new(&code, &output, &input);

    eval.run();

    println!("\nStack: {:?}", eval.stack);
}
