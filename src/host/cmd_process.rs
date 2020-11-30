use crate::common::order::*;
use crate::common::utils::*;
use std::collections::HashMap;
use std::io::stdout;
use std::io::Write;
use std::path::Path;
use crossterm::style::Colorize;
use crossterm::style::Styler;
use super::shell::show;
use udp_hole_punching as hole;
use super::shell_process::ShellProcess;
use std::net::SocketAddr;

pub trait CmdProcess {
     fn cmd_empty(&self) -> anyhow::Result<()>;
     fn cmd_use(&self) -> anyhow::Result<()>;
     fn cmd_other(&self) -> anyhow::Result<()>;
     fn cmd_send(&self) -> anyhow::Result<()>;
     fn cmd_where(&self) -> anyhow::Result<()>;
}

impl CmdProcess for ShellProcess {
     fn cmd_where(&self) -> anyhow::Result<()> {
        let addr = hole::read_peer_address();
        if let Err(e) = addr {
            println!("\n地址询问失败:{}\n", e.to_string());
        }else{
            println!("\n地址询问成功:{}\n", addr.unwrap().to_string());
        }
        print!("{}", ">>".white().bold());
        stdout().flush();
        Ok(())
    }
     fn cmd_use(&self) -> anyhow::Result<()> {
        if self.args.len() == 1 {
            let peer_id = self.args[0].clone();
            hole::ask_peer_address(&peer_id)?;
        } else {
            let res="use args should be 1";
            print!("\n{}\n", res.red().bold());
        }
        print!("\n{}", ">>".white().bold());
        stdout().flush();
        Ok(())
    }
    fn cmd_empty(&self) -> anyhow::Result<()> {
        print!("\n{}\n", "命令为空".red().on_black().bold());
        print!("{}", ">>".white().bold());
        stdout().flush()?;
        Ok(())
    }
    fn cmd_other(&self) -> anyhow::Result<()> {
        let addr = hole::read_peer_address();
        if let Err(e) = addr {
            println!("\n地址解析失败:{}\n", e.to_string());
            print!("{}", ">>".white().bold());
            stdout().flush()?;
            return Ok(());
        }
        let peer = addr.unwrap();
        let order = self.order();
        order.send(peer);
        Ok(())
    }

    fn cmd_send(&self) -> anyhow::Result<()> {
        let addr = hole::read_peer_address();
        if let Err(e) = addr {
            println!("\n地址解析失败:{}\n", e.to_string());
            print!("{}", ">>".white().bold());
            stdout().flush()?;
            return Ok(());
        }
        let peer = addr.unwrap();
        let mut order = self.order();
        let bin = read_file_as_u8(&self.args[0])?;
        let n=bin.len();
        dbg!(n);
        order.data = bin;
        order.send(peer);
        Ok(())
    }
}

