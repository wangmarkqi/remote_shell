use crate::common::order::*;
use crate::common::myfs::*;
use std::collections::HashMap;
use std::io::stdout;
use std::io::Write;
use std::path::Path;
use crossterm::style::Colorize;
use crossterm::style::Styler;
use super::shell::show;
use super::trans::Trans;
use crate::host::command::Command;

pub trait Process {
    fn cmd_empty(&self) -> anyhow::Result<bool>;
    fn cmd_restart(&self) -> anyhow::Result<bool>;
    fn cmd_other(&self) -> anyhow::Result<bool>;
    fn cmd_send(&self) -> anyhow::Result<bool>;
    fn cmd_rec(&self) -> anyhow::Result<bool>;
}

impl Process for Command {
    fn cmd_empty(&self) -> anyhow::Result<bool> {
        print!("\n{}\n", "命令为空".red().on_black().bold());
        stdout().flush()?;
        Ok(false)
    }
    fn cmd_restart(&self) -> anyhow::Result<bool> {
        let ord = Order::new_from_host(CMD_RESTART, &self.slave);
        // 使用pass重启命令和其他走一个主题
        let res = post_order(API_REQ, &ord);
        match res {
            Ok(r) => {
                return Ok(true);
            }
            Err(e) => {
                let res = format!("发送失败:{}", e);
                println!("/n{}/n", res.red());
                return Ok(false);
            }
        }
    }
    fn cmd_other(&self) -> anyhow::Result<bool> {
        let mut ord = Order::new_from_host(&self.input, &self.slave);
        let resp = self.send_order(&ord);
        show(&resp.slave, &resp.status, &resp.resp);
        if resp.resp.starts_with(FAIL) || resp.status == FAIL {
            return Ok(false);
        }
        return Ok(true);
        Ok(true)
    }

    fn cmd_send(&self) -> anyhow::Result<bool> {
        let mut ord = Order::new_from_host(&self.input, &self.slave);
        if let Ok((cmd, args)) = parse_input(&self.input) {
            if cmd == CMD_SEND && args.len() >= 2 {
                if let Ok(content) = read_file_as_hex(&args[0]) {
                    ord.resp = content;
                }
            }
        }

        let resp = self.send_order(&ord);
        show(&resp.slave, &resp.status, &resp.req);
        stdout().flush()?;
        if resp.resp.starts_with(FAIL) || resp.status == FAIL {
            return Ok(false);
        }
        Ok(true)
    }


    fn cmd_rec(&self) -> anyhow::Result<bool> {
        let ord = Order::new_from_host(&self.input, &self.slave);

        let resp = self.send_order(&ord);
        if resp.status == FAIL || resp.resp.starts_with(FAIL) {
            show(&resp.slave, &resp.status, &resp.req);
            stdout().flush()?;
            return Ok(false);
        }

        if let Ok((_, args)) = parse_input(&self.input) {
            // 目录不存在创建
            create_file_dir(&args[0]);
            if let Ok(_) = write_file_as_hex(&args[0], &resp.resp) {
                show(&resp.slave, &resp.status, &args[0]);
                return Ok(true);
            }
        }
        show(&resp.slave, &resp.status, &resp.resp);
        Ok(true)
    }
}


