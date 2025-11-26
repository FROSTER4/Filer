mod commands;
mod current_catalog;

use std::error::Error;
use std::io::{stdin, stdout, Write};

use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use commands::*;
use current_catalog::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut current_dir: CurrentCatalog =
        CurrentCatalog::new(DirectoryMoveType::Home, None, None).unwrap();

    let mut stdout = stdout().into_raw_mode()?;
    let stdin = stdin();
    let mut keys = stdin.keys();

    // ANSI-code to clear the screen (doesn't work on Windows)
    let clear_screen = "\x1B[2J\x1B[H";

    write!(stdout, "{}{}", clear_screen, cursor::Hide)?;
    stdout.flush()?;

    println!("{}", current_dir.print_files());

    loop {
        let key = match keys.next() {
            Some(Ok(k)) => k,
            Some(Err(_)) => continue, // Пропускаем итерацию при попытке чтения ключа
            None => break,
        };

        match key {
            Key::Ctrl('q') => {
                break;
            }
            Key::Left => {
                current_dir =
                    CurrentCatalog::new(DirectoryMoveType::Prev, Some(current_dir.path), None)
                        .unwrap();
            }
            Key::Right => {
                current_dir =
                    CurrentCatalog::new(DirectoryMoveType::Entry, None, Some(current_dir)).unwrap();
            }
            Key::Up => {
                current_dir.prev();
            }
            Key::Down => {
                current_dir.next();
            }
            Key::Char('.') => {
                current_dir.set_hidden_flag();
            }
            Key::Char('~') => {
                current_dir = go_to_home();
            }
            Key::Alt('/') => {
                current_dir = go_to_root();
            }
            _ => write!(stdout, "").unwrap(),
        };

        write!(stdout, "{}", clear_screen)?;
        write!(stdout, "{}", current_dir.print_files())?;
        stdout.flush()?;
    }

    write!(stdout, "{}", cursor::Show)?;

    Ok(())
}

/* code to clear the screen(termion - it works to Linux and Windows)
write!(stdout, "{}", termion::clear::All)
*/
