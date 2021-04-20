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

pub enum StackOperation {
    PUSH,
    POP
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
    pub on_input: &'a dyn Fn(&mut Self, EventType),
    pub on_stack_change: Option<&'a dyn Fn(i64, StackOperation)>,
    pub on_p: Option<&'a dyn Fn(usize, usize, i64)>
}

impl<'a> Interpreter<'a> {

    pub fn new(code: &str, output: &'a dyn Fn(i64, EventType), input: &'a dyn Fn(&mut Self, EventType)) -> Self {
        Interpreter {
            x: 0,
            y: 0,
            direction: Direction::RIGHT,
            stack: vec![],
            code: input::to_grid(code),
            str_mode: false,
            ended: true,
            on_output: output,
            on_input: input,
            on_stack_change: None,
            on_p: None
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

    #[inline]
    fn push(&mut self, character: i64) {
        self.stack.push(character);
        if self.on_stack_change.is_some() { self.on_stack_change.unwrap()(character, StackOperation::PUSH) }
    }

    #[inline]
    fn pop(&mut self) -> i64 {
        let val = self.stack.pop();
        if self.on_stack_change.is_some() { self.on_stack_change.unwrap()(0, StackOperation::POP); };
        match val {
            Some(x) => x,
            None => 0
        }
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
            self.push(character as i64);
            return;
        }
        match character {
            '0' ..= '9' => self.push(character.to_digit(10).unwrap() as i64),
            '+' => {
                let a = self.pop();
                let b = self.pop();
                self.push(a + b);
            },
            '-' => {
                let a = self.pop();
                let b = self.pop();
                self.push(b - a);
            },
            '*' => {
                let a = self.pop();
                let b = self.pop();
                self.push(a * b);
            },
            '/' => {
                let a = self.pop();
                if a == 0 { panic!("Cannot divide by 0") }
                let b = self.pop();
                self.stack.push(b / a);
            },
            '%' => {
                let a = self.pop();
                let b = self.pop();
                self.stack.push(b % a);
            },
            '!' => {
                let val = self.pop();
                self.stack.push((val == 0) as i64);
            },
            '`' => {
                let a = self.pop();
                let b = self.pop();
                self.stack.push((b > a) as i64)
            },
            '_' => {
                if self.pop() == 0 { self.direction = Direction::RIGHT }
                else { self.direction = Direction::LEFT }
            },
            '|' => {
                if self.pop() == 0 { self.direction = Direction::DOWN }
                else { self.direction = Direction::UP }
            },
            ':' => self.push(*self.stack.last().unwrap_or(&0)),
            '$' => {
                self.pop();
            },
            '?' => {
                let num = rand::random::<f64>();
                if num < 0.25 { self.direction = Direction::RIGHT }
                else if num < 0.50 { self.direction = Direction::LEFT }
                else if num < 0.75 { self.direction = Direction::UP }
                else { self.direction = Direction::DOWN };
            },
            'p' => {
                let x = self.pop() as usize;
                let y = self.pop() as usize;
                let val = self.pop();
                if self.is_not_valid_pos(x, y) { self.push(0); return; }
                self.code[x][y] = u8::try_from(val).unwrap_or(0) as char;
                if self.on_p.is_some() { self.on_p.unwrap()(x, y, val) }
            },
            'g' => {
                let x = self.pop() as usize;
                let y = self.pop() as usize;
                if self.is_not_valid_pos(x, y) { self.push(0); return; }
                self.push(self.code[x][y] as i64);
            },
            '\\' => {
                let val1 = self.pop();
                let val2 = self.pop();
                self.push(val1);
                self.push(val2);
            },
            '&' => (self.on_input)(self, EventType::INTEGER),
            '~' => (self.on_input)(self, EventType::STRING),
            '#' => self.inc_pos(),
            '"' => self.str_mode = true,
            '.' => (self.on_output)(self.pop(), EventType::INTEGER),
            ',' => (self.on_output)(self.pop(), EventType::STRING),
            '@' => self.ended = true,
            '>' => self.direction = Direction::RIGHT,
            'v' => self.direction = Direction::DOWN,
            '<' => self.direction = Direction::LEFT,
            '^' => self.direction = Direction::UP,
            ' ' => {},
            _ => panic!("Unsupported instruction: {:?}", character)
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