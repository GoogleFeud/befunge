use std::convert::TryFrom;
use std::fs;
use std::io;
use std::env;
mod interpreter;

struct Events;

impl interpreter::Events for Events {
    fn on_input(&self, stack: &mut Vec<i64>, event_type: interpreter::EventType) {
        match event_type {
            interpreter::EventType::INTEGER => {
                let mut res = String::new(); 
                io::stdin().read_line(&mut res).expect("Failed to read int");
                stack.push(res.trim().parse::<i64>().expect("Invalid integer pvovided"));
            }
            interpreter::EventType::STRING => {
                let mut res = String::new(); 
                io::stdin().read_line(&mut res).expect("Failed to read string");
                for character in res.chars() {
                    stack.push(character as i64);
                }
            }
        }
    }

    fn on_output(&self, val: i64, event_type: interpreter::EventType) {
        match event_type {
            interpreter::EventType::INTEGER => print!("{} ", val),
            interpreter::EventType::STRING => print!("{}", u8::try_from(val).unwrap_or(0) as char)
        };
    }

}

fn main() {

    let filename = env::args().nth(1).expect("Please provide a file to run.");

    let code = fs::read_to_string(filename).expect("Could not find file");
    let mut eval = interpreter::Interpreter::new(&code, Events {});

    eval.run();

    println!("\nStack: {:?}", eval.stack);
}
