use std::{
    io::{stdin, Write, stdout},
    time::Duration,
};
use super::command::Command;

use crossterm::{
    execute, queue, cursor, terminal, Result,
    event::{self, KeyEvent, read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, ClearType},
    style::{self, Colorize, Styler},
};
use crate::host::history::History;
use crate::common::order::*;

pub fn run_shell() -> anyhow::Result<()> {
    enable_raw_mode()?;

    let mut stdout = stdout();

    let mut line = String::new();
    let mut cmd = Command::default();
    print!("{}", ">>".white().on_black().bold());
    stdout.flush()?;
    loop {
        let event = read()?;

        if let Event::Key(KeyEvent { code: KeyCode::Char(c), .. }) = event {
            print!("{}", c);
            stdout.flush()?;
            line.push(c);
        }

        if event == Event::Key(KeyCode::Up.into()) {
            let his = cmd.up_before();
            if his != "".to_string() {
                queue!(stdout,
                    terminal::Clear(ClearType::CurrentLine),
                    cursor::MoveLeft(999));
                print!("{}{}", ">>".white().on_black().bold(), his);
                stdout.flush()?;
                line = his;
            }
        }
        if event == Event::Key(KeyCode::Down.into()) {
            let his = cmd.down_next();
            if his != "".to_string() {
                queue!(stdout,
                    terminal::Clear(ClearType::CurrentLine),
                    cursor::MoveLeft(999));
                print!("{}{}", ">>".white().on_black().bold(), his);
                stdout.flush()?;
                line = his;
            }
        }
        if event == Event::Key(KeyCode::Tab.into()) {
            let his = cmd.tab_complement(&line);
            if his != "".to_string() {
                queue!(stdout,
                    terminal::Clear(ClearType::CurrentLine),
                    cursor::MoveLeft(999));
                print!("{}{}", ">>".white().on_black().bold(), his);
                stdout.flush()?;
                line = his;
            }
        }

        if event == Event::Key(KeyCode::Backspace.into()) {
            queue!(stdout,
                    cursor::MoveLeft(1),
                    terminal::Clear(ClearType::UntilNewLine),
                    );
            stdout.flush()?;
            line.pop();
        }
        if event == Event::Key(KeyCode::Home.into()) {
            queue!(stdout,
                    terminal::Clear(ClearType::All),
                    cursor::MoveTo(0,0));
            print!("{}", ">>".white().on_black().bold());
            stdout.flush()?;
            line = String::new();
        }
        if event == Event::Key(KeyCode::Enter.into()) {
            cmd.parse(&line);
            let res = cmd.run()?;
            if res {
                cmd.save()?;
            }
            line = String::new();
            print!("\n{}", ">>".white().bold());
            stdout.flush()?;
        }
        if event == Event::Key(KeyCode::Esc.into()) {
            break;
        }
    }

    disable_raw_mode();
    Ok(())
}

pub fn show(slave: &str, status: &str, resp: &str) {
    if status == SUCCESS {
        let mut s = resp.to_string().trim().to_string();
        if s.ends_with("\n") {
            s.pop();
        }
        print!("\n{}\n{}", slave.bold().yellow().on_green(), s.green().bold());
    } else {
        print!("\n{}{}", slave.bold().negative(), resp.red().bold());
    }
    stdout().flush();
}
