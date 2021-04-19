mod interpreter;

fn main() {
    let mut eval = interpreter::Interpreter::new("19+9+1+");
    loop {
        if eval.isNotValidPos(eval.x, eval.y) { break; };
        eval.tick();
        eval.incPos();
    }
    println!("{}", eval.stack[0]);
}
