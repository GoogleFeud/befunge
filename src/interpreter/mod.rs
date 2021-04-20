use std::convert::TryFrom;
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
    pub stack: Vec<i64>,
    pub direction: Direction,
    pub x: usize,
    pub y: usize,
    pub str_mode: bool,
    pub ended: bool,
    pub on_output: &'a dyn Fn(i64, EventType),
    pub on_input: &'a dyn Fn(EventType) -> i64
}

impl<'a> Interpreter<'a> {

    pub fn new(code: &str, output: &'a dyn Fn(i64, EventType), input: &'a dyn Fn(EventType) -> i64) -> Self {
        Interpreter {
            x: 0,
            y: 0,
            direction: Direction::RIGHT,
            stack: vec![],
            code: input::to_grid(code),
            str_mode: false,
            ended: true,
            on_output: output,
            on_input: input
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
        if self.is_not_valid_pos(self.x, self.y) { 
            self.ended = true;
            return;
        }
        let character = self.code[self.x][self.y];
        if self.str_mode {
            if character == '"' {
                self.str_mode = false;
                return;
            }
            self.stack.push(character as i64);
            return;
        }
        match character {
            '0' ..= '9' => self.stack.push(character.to_digit(10).unwrap() as i64),
            '+' => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push(b.unwrap_or(0) + a.unwrap_or(0));
            },
            '-' => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push(b.unwrap_or(0) - a.unwrap_or(0));
            },
            '*' => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push(b.unwrap_or(0) * a.unwrap_or(0));
            },
            '/' => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push(b.unwrap_or(0) / a.unwrap_or(0));
            },
            '%' => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push(b.unwrap_or(0) % a.unwrap_or(0));
            },
            '!' => {
                let val = self.stack.pop();
                self.stack.push((val.unwrap_or(0) == 0) as i64);
            },
            '`' => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push((b.unwrap_or(0) > a.unwrap_or(0)) as i64)
            },
            '_' => {
                let val = self.stack.pop();
                if val.unwrap_or(0) == 0 { self.direction = Direction::RIGHT }
                else { self.direction = Direction::LEFT }
            },
            '|' => {
                let val = self.stack.pop();
                if val.unwrap_or(0) == 0 { self.direction = Direction::DOWN }
                else { self.direction = Direction::UP }
            },
            ':' => {
                let len = self.stack.len();
                if len == 0 { return }
                self.stack.push(self.stack[len - 1]);
            },
            '$' => {
                self.stack.pop();
            },
            '?' => {
                let num = rand::random::<f64>();
                if num < 0.25 { self.direction = Direction::RIGHT }
                else if num < 0.50 { self.direction = Direction::LEFT }
                else if num > 0.75 { self.direction = Direction::UP }
                else { self.direction = Direction::DOWN };
            },
            'p' => {
                let x = self.stack.pop().unwrap_or(0) as usize;
                let y = self.stack.pop().unwrap_or(0) as usize;
                let val = self.stack.pop().unwrap_or(0);
                if self.is_not_valid_pos(x, y) { return }
                self.code[x][y] = u8::try_from(val).unwrap_or(0) as char;
            },
            'g' => {
                let x = self.stack.pop().unwrap_or(0) as usize;
                let y = self.stack.pop().unwrap_or(0) as usize;
                if self.is_not_valid_pos(x, y) { return }
                self.stack.push(self.code[x][y] as i64);
            },
            '\\' => {
                let val1 = self.stack.pop().unwrap_or(0);
                let val2 = self.stack.pop().unwrap_or(0);
                self.stack.push(val1);
                self.stack.push(val2);
            },
            '&' => self.stack.push((self.on_input)(EventType::INTEGER)),
            '~' => self.stack.push((self.on_input)(EventType::STRING)),
            '#' => self.inc_pos(),
            '"' => self.str_mode = true,
            '.' => (self.on_output)(self.stack.pop().unwrap_or(0), EventType::INTEGER),
            ',' => (self.on_output)(self.stack.pop().unwrap_or(0), EventType::STRING),
            '@' => self.ended = true,
            '>' => self.direction = Direction::RIGHT,
            'v' => self.direction = Direction::DOWN,
            '<' => self.direction = Direction::LEFT,
            '^' => self.direction = Direction::UP,
            ' ' => {},
            _ => panic!("{:?}", character)
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