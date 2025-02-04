mod cube;
mod terminal;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use cube::Cube;
use std::time::{Duration, Instant};
use std::thread::sleep;
use std::env;

const FPS: u64 = 60;

fn handle_input(cube: &mut Cube, auto: bool) -> bool {
    if event::poll(Duration::from_millis(1)).unwrap() {
        if let Event::Key(key_event) = event::read().unwrap() {
            if key_event.code == KeyCode::Char('d') && key_event.modifiers.contains(KeyModifiers::CONTROL) {
                return true;
            }
            if bool::from(auto) {
                return false;
            }
            if key_event.code == KeyCode::Char('e') {
                cube.rotation.x -= 0.01;
            }
            if key_event.code == KeyCode::Char('r') {
                cube.rotation.x += 0.01;
            }
            if key_event.code == KeyCode::Char('d') {
                cube.rotation.y -= 0.01;
            }
            if key_event.code == KeyCode::Char('f') {
                cube.rotation.y += 0.01;
            }
            if key_event.code == KeyCode::Char('c') {
                cube.rotation.z -= 0.01;
            }
            if key_event.code == KeyCode::Char('v') {
                cube.rotation.z += 0.01;
            }
        }
    }
    return false;
}

fn help() {
    println!("cargo run -- manual : Run the program in manual mode");
    println!("cargo run -- auto : Run the program in automatic mode");
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let auto;
    if args.len() != 2 {
        help();
        return;
    } else if args[1] == "manual" {
        auto = false;
    } else if args[1] == "auto" {
        auto = true;
    } else {
        help();
        return;
    }

    let frame_duration: Duration = Duration::from_secs_f64(1.0 / FPS as f64);
    let mut cube = cube::Cube::new(auto);
    let terminal = terminal::Terminal::new();
    let mut last_frame = Instant::now();

    terminal.initiate_terminal();

    loop {
        if handle_input(&mut cube, auto) {
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
