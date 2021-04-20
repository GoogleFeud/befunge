
mod input;

pub enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN
}

pub enum EventType {
    INTEGER,
    STRING
}

pub struct Interpreter<'a> {
    pub code: Vec<Vec<char>>,
    pub stack: Vec<i32>,
    pub direction: Direction,
    pub x: usize,
    pub y: usize,
    pub str_mode: bool,
    pub ended: bool,
    pub on_output: &'a dyn Fn(i32, EventType)
}

impl<'a> Interpreter<'a> {

    pub fn new(code: &str, output: &'a dyn Fn(i32, EventType)) -> Self {
        Interpreter {
            x: 0,
            y: 0,
            direction: Direction::RIGHT,
            stack: vec![],
            code: input::to_grid(code),
            str_mode: false,
            ended: true,
            on_output: output
        }
    }

    pub fn inc_pos(&mut self) {
        match self.direction {
            Direction::RIGHT => {
                if self.y == self.code[self.x].len() - 1 { self.y = 0 }
                else { self.y += 1 }
            },
            Direction::LEFT => {
                if self.y == 0 { self.y = self.code[self.x].len() - 1 }
                else { self.y -= 1 }
            },
            Direction::UP => {
                if self.x == 0 { self.x = self.code.len() - 1 }
                else { self.x -= 1 }
            },
            Direction::DOWN => {
                if self.x == self.code.len() - 1 { self.x = 0 }
                else { self.x += 1 }
            }
        };
    }

    pub fn is_not_valid_pos(&self, x: usize, y: usize) -> bool {
        x >= self.code.len() || y >= self.code[x].len()
    }

    pub fn tick(&mut self) {
        if self.ended { return; }
        let character = self.code[self.x][self.y];
        if self.str_mode {
            if character == '"' {
                self.str_mode = false;
                return;
            }
            self.stack.push(character as i32);
        }
        match character {
            '1' ..= '9' => self.stack.push(character.to_digit(10).unwrap() as i32),
            '+' => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                if a.is_none() || b.is_none() { panic!("Invalid."); };
                self.stack.push(a.unwrap() + b.unwrap());
            },
            '"' => self.str_mode = true,
            '.' => (self.on_output)(self.stack.pop().unwrap_or(0), EventType::INTEGER),
            ',' => (self.on_output)(self.stack.pop().unwrap_or(0), EventType::STRING),
            '@' => self.ended = true,
            _ => {}
        };
    }

    pub fn run(&mut self) {
        self.ended = false;
        while !self.ended {
            self.tick();
            self.inc_pos();
        }
    } 

}