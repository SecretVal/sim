#![allow(dead_code)]
use std::io::{stdout, Result};

use crossterm::{
    cursor::{MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp},
    event::{read, Event, KeyCode},
    execute, style,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen},
};

#[derive(Debug, Default)]
struct State {
    mode: Mode,
}

#[derive(Debug, Default)]
enum Mode {
    #[default]
    Normal,
    Insert,
}

fn main() -> Result<()> {
    let mut state = State::default();
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(
        stdout,
        Clear(ClearType::All),
        MoveTo(0, 0),
        EnterAlternateScreen
    )?;

    let mut quit = false;
    while !quit {
        let event = read()?;
        match state.mode {
            Mode::Normal => match event {
                Event::Key(e) => match e.code {
                    KeyCode::Char('q') => quit = true,
                    KeyCode::Char('h') => execute!(stdout, MoveLeft(1))?,
                    KeyCode::Char('j') => execute!(stdout, MoveDown(1))?,
                    KeyCode::Char('k') => execute!(stdout, MoveUp(1))?,
                    KeyCode::Char('l') => execute!(stdout, MoveRight(1))?,
                    KeyCode::Char('i') => state.mode = Mode::Insert,
                    _ => {}
                },
                _ => todo!(),
            },
            Mode::Insert => {
                let mut esc = false;
                while !esc {
                    let event = read()?;
                    match event {
                        Event::Key(key) => match key.code {
                            KeyCode::Esc => esc = true,
                            KeyCode::Backspace => {
                                execute!(stdout, MoveLeft(1), style::Print(" "), MoveLeft(1))?
                            }
                            KeyCode::Char(e) => execute!(stdout, style::Print(e))?,
                            _ => {}
                        },
                        _ => {}
                    }
                }
                state.mode = Mode::Normal;
            }
        }
    }
    disable_raw_mode()?;
    Ok(())
}
