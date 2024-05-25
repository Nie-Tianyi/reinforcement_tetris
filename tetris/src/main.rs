//! 表示注释

use std::fmt::{Display, Formatter};
use std::io::{stdout, Write};
use std::io::prelude::*;
use std::sync::mpsc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use tetris::tetris::{Tetris, UserAction};



fn main() {

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            let line = line.unwrap();
            println!("You typed: {}", line);
            let action = match line.as_str() {
                "w" => UserAction::Rotate,
                "a" => UserAction::Left,
                "s" => UserAction::Down,
                "d" => UserAction::Right,
                _ => continue,
            };
            if tx.send(action).is_err(){
                break;
            };
        }
    });


    let mut stdout = stdout();
    let mut game = Tetris::new();
    loop {
        clear_console();
        let (main_view,score,next_shape)= game.view();
        writeln!(stdout, "{}", TUI(main_view,score,next_shape)).unwrap();
        stdout.flush().unwrap();
        let input = rx.try_recv().unwrap_or(UserAction::None);
        game.next(input);
        if game.is_game_over() {
            break;
        }
        sleep(Duration::from_millis(500)); // 暂停500毫秒
    }
    println!("Game Over!")
}

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush().unwrap();
}

struct TUI([[bool; 10]; 20], i64, [[bool; 4]; 4]);

impl Display for TUI {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|row| {
            row.iter().map(|&cell| if cell { " ■" } else { " □" }).collect::<String>()
        }).collect::<Vec<String>>().join("\n"))?;
        write!(f, "\n\nScore: {}\n\nNext Shape:\n", self.1)?;
        write!(f, "{}", self.2.iter().map(|row| {
            row.iter().map(|&cell| if cell { " ■" } else { " □" }).collect::<String>()
        }).collect::<Vec<String>>().join("\n"))?;
        Ok(())
    }
}