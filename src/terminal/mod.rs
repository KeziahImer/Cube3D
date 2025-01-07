use crossterm::{execute, cursor::{Hide, Show}, terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, SetSize}};
use std::io::{self, Write};

const TERMINAL_WIDTH: f32 = 60.0;
const TERMINAL_HEIGHT: f32 = 30.0;

pub struct Terminal {
    cols: u16,
    rows: u16,
}

impl Terminal {
    pub fn new() -> Terminal {
        let (cols, rows) = size().unwrap();
        Terminal {
            cols,
            rows,
        }
    }
    pub fn initiate_terminal(&self) {
        enable_raw_mode().unwrap();
        execute!(io::stdout(), Hide, Clear(ClearType::All), SetSize(TERMINAL_WIDTH as u16, TERMINAL_HEIGHT as u16)).unwrap();
    }
    
    pub fn clear_terminal(&self) {
        execute!(io::stdout(), Show, Clear(ClearType::All), SetSize(self.cols, self.rows)).unwrap();
        disable_raw_mode().unwrap();
    }

    pub fn draw(&self, x: f32, y: f32, z: f32) {
        let resized_x: u16 = (x * 20.0 + (TERMINAL_WIDTH / 2.0)).floor() as u16;
        let resized_y: u16 = (y * 10.0 + (TERMINAL_HEIGHT / 2.0)).floor() as u16;
        let c;
        if z < -0.5 {
            c = '#';
        } else if z < 0.0 {
            c = '@';
        } else if z < 0.5 {
            c = '*';
        } else {
            c = '.';
        }
        execute!(io::stdout(), crossterm::cursor::MoveTo(resized_x, resized_y), crossterm::style::Print(c)).unwrap();
    }

    pub fn clear(&self) {
        execute!(io::stdout(), Clear(ClearType::All)).unwrap();
    }

    pub fn flush(&self) {
        io::stdout().flush().unwrap();
    }
}
