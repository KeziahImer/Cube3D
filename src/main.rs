mod cube;
mod terminal;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::time::{Duration, Instant};
use std::thread::sleep;

const FPS: u64 = 60;

fn handle_input() -> bool {
    if event::poll(Duration::from_millis(1)).unwrap() {
        if let Event::Key(key_event) = event::read().unwrap() {
            if key_event.code == KeyCode::Char('d') && key_event.modifiers.contains(KeyModifiers::CONTROL) {
                println!("Exiting...");
                return true;
            }
        }
    }
    return false;
}

fn main() {
    let frame_duration: Duration = Duration::from_secs_f64(1.0 / FPS as f64);
    let mut cube = cube::Cube::new();
    let terminal = terminal::Terminal::new();
    let mut last_frame = Instant::now();

    terminal.initiate_terminal();

    loop {
        if handle_input() {
            break;
        }

        let now = Instant::now();
        let elapsed = now.duration_since(last_frame);

        if elapsed >= frame_duration {
            cube.update();
            cube.render(&terminal);
            last_frame = now;
        } else {
            sleep(frame_duration - elapsed);
        }
    }

    terminal.clear_terminal();

}
