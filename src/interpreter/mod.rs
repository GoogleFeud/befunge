
mod input;

pub enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN
}

pub struct Interpreter {
    pub code: Vec<Vec<char>>,
    pub stack: Vec<i32>,
    pub direction: Direction,
    pub x: usize,
    pub y: usize,
    pub strMode: bool
}

impl Interpreter {

    pub fn new(code: &str) -> Self {
        Interpreter {
            x: 0,
            y: 0,
            direction: Direction::RIGHT,
            stack: vec![],
            code: input::to_grid(code),
            strMode: false
        }
    }

    pub fn incPos(&mut self) {
        match self.direction {
            Direction::RIGHT => self.y += 1,
            Direction::LEFT => self.y -= 1,
            Direction::UP => self.x -= 1,
            Direction::DOWN => self.x += 1
        };
    }

    pub fn isNotValidPos(&self, x: usize, y: usize) -> bool {
        x >= self.code.len() || y >= self.code[x].len()
    }

    pub fn tick(&mut self) {
        let character = self.code[self.x][self.y];
        match character {
            '1' ..= '9' => self.stack.push(character.to_digit(10).unwrap() as i32),
            '+' => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                if a.is_none() || b.is_none() { panic!("Invalid."); };
                self.stack.push(a.unwrap() + b.unwrap());
            }
            _ => {}
        };
    }

}