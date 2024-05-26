//! 俄罗斯方块游戏
//!
//! todo: 旋转逻辑和常见的旋转逻辑不一样，有待改进

use std::fmt::{Display, Formatter};
use std::io::{stdout, Write};
use std::sync::mpsc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use tetris::tetris::{Tetris, UserAction};
use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};


fn main() {

    let (tx,rx) = mpsc::channel();

    // 开启原始模式，输入不通过缓冲直接传递给程序
    // Windows 终端会默认执行输入两遍
    // Mac 表现正常
    enable_raw_mode().unwrap();


    thread::spawn(move || {
        loop {
            if poll(Duration::from_millis(33)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    let action = match event.code {
                        KeyCode::Char('a') => UserAction::Left,
                        KeyCode::Char('d') => UserAction::Right,
                        KeyCode::Char('s') => UserAction::Down,
                        KeyCode::Char('w') => UserAction::Rotate,
                        KeyCode::Char(' ') => UserAction::QuickDown,
                        _ => UserAction::None
                    };
                    tx.send(action).unwrap();
                }
            }
        }
    });

    // 初始化输出
    let mut stdout = stdout();
    // 初始化游戏
    let mut game = Tetris::new();
    // 一秒30帧,每一帧都查询用户输入，30帧下落一次，其余用户不输入都是None
    let mut frame_count = 0; // 帧数计数
    loop {
        frame_count += 1; // 帧数加1

        // 更新视图
        clear_console(&mut stdout); // 清空控制台
        let (main_view,score,next_shape)= game.view(); // 获取游戏视图
        writeln!(stdout, "{}", TUI(main_view,score,next_shape)).unwrap(); // 写入显示
        stdout.flush().unwrap(); // 控制台显示

        // 每15帧下落一格
        if frame_count % 15 == 0 {
            game.next(UserAction::Down);
        } else {
            let input = rx.try_recv().unwrap_or(UserAction::None);
            game.next(input)
        }

        if game.is_game_over() {
            break;
        }
        sleep(Duration::from_millis(33)); // 暂停33毫秒,1秒30帧
    }

    disable_raw_mode().unwrap();
    clear_console(&mut stdout);
    println!("Game Over!");
}

fn clear_console(stdout:&mut  std::io::Stdout) {
    execute!(stdout, crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
    execute!(stdout, crossterm::cursor::MoveTo(0, 0)).unwrap();
    stdout.flush().unwrap();
}

struct TUI([[bool; 10]; 20], i64, [[bool; 4]; 4]);

impl Display for TUI {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // 画出主视图
        for row in self.0.iter() {
            for &cell in row.iter() {
                write!(f, "{}", if cell { " ■" } else { " □" })?;
            }
            write!(f, "\r\n")?;
        }

        // 分数
        write!(f, "\r\nScore: {}\r\n\r\nNext Shape:\r\n", self.1)?;

        // 下一个形状
        for row in self.2.iter() {
            for &cell in row.iter() {
                write!(f, "{}", if cell { " ■" } else { " □" })?;
            }
            write!(f, "\r\n")?;
        }

        Ok(())
    }
}