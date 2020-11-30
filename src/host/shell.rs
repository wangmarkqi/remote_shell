use std::{
    io::{stdin, Write, stdout},
    time::Duration,
};
use crossterm::{
    execute, queue, cursor, terminal, Result,
    event::{self, KeyEvent, read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, ClearType},
    style::{self, Colorize, Styler},
};
use crate::host::history_process::History;
use crate::common::order::*;
use udp_hole_punching as hole;
use crate::common::utils::*;
use super::shell_process::ShellProcess;
use crate::common::cmd::Cmd;

pub fn run_shell(swap_server: &str,db_path:&str) -> anyhow::Result<()> {
    let mut conf = hole::Conf::default();
    conf.swap_server = swap_server.to_string();
    conf.db_path=db_path.to_string();
    conf.id = "".to_string();
    conf.set();

    hole::init_udp();
    std::thread::spawn(|| {
        hole::listen();
    });



    enable_raw_mode()?;
    let mut stdout = stdout();
    let mut line = String::new();
    let mut cmd = ShellProcess::default();
    std::thread::spawn(|| {
        show_cmd_res();
    });

    print!("\n{}{}{}\n", "***********".red(), "欢迎使用远程shell".green().bold(), "*************".red());
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
            let res = cmd.parse(&line);
            if let Err(e) = cmd.parse(&line) {
                print!("\n{}解析错误:{}\n", &line, e.to_string().red().on_black());
                print!("{}", ">>".white().bold());
                stdout.flush()?;
            } else {
                cmd.run();
                cmd.save()?;
            }
            line = String::new();
        }
        if event == Event::Key(KeyCode::Esc.into()) {
            break;
        }
    }

    disable_raw_mode();
    Ok(())
}

fn show_cmd_res() -> anyhow::Result<()> {
    let mut stdout = stdout();
    loop {
        let (peer, ord) = Order::read_order_from_cache();
        let peer = peer.to_string();
        let myinput = &ord.input.clone();
        if &myinput.as_str() == &"" {
            continue;
        }
        if ord.cmd == Cmd::Start.to_string() {
            print!("\n{}--->{}{}\n", &peer.bold().yellow().on_green(), myinput.clone().red().bold(), "开始接收：".green().bold());
            stdout.flush()?;
            continue;
        }
        if ord.cmd == Cmd::Finish.to_string() {
            print!("*********{}************\n", "接收结束".red().bold());
            print!("{}", ">>".white().bold());
            stdout.flush()?;
            continue;
        }
        if ord.cmd == Cmd::Rec.to_string() {
            let args = &ord.args;
            let data = &ord.data;
            let savepath = &args[0];
            create_file_dir(savepath);
            let res = write_file_as_u8(savepath, data)?;
            print!("{}\n", &res.green().bold());
            stdout.flush()?;
            continue;
        }
        let res = String::from_utf8_lossy(&ord.data);
        let res = format!("{}", res);
        if res != "".to_string() {
            print!("{}\n", &res.green().bold());
            stdout.flush()?;
        }
    }
}

pub fn show(cmd: &str, res: &str) {
    let peer = {
        match hole::read_peer_address() {
            Err(e) => format!("地址解析错误{}", e),
            Ok(addr) => addr.to_string(),
        }
    };
    let mut s = res.to_string().trim().to_string();
    if s.ends_with("\n") {
        s.pop();
    }
    print!("\n{}--->{}\n{}\n", peer.bold().yellow().on_green(), cmd.red().bold(), s.green().bold());
}
