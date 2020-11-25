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
use async_trait::async_trait;
use super::shell_process::ShellProcess;
use std::net::SocketAddr;

#[async_trait]
pub trait CmdProcess {
    async fn cmd_empty(&self) -> anyhow::Result<()>;
    async fn cmd_use(&self) -> anyhow::Result<()>;
    async fn cmd_other(&self) -> anyhow::Result<()>;
    async fn cmd_send(&self) -> anyhow::Result<()>;
    async fn cmd_where(&self) -> anyhow::Result<()>;
}

#[async_trait]
impl CmdProcess for ShellProcess {
    async fn cmd_where(&self) -> anyhow::Result<()> {
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
    async fn cmd_use(&self) -> anyhow::Result<()> {
        if self.args.len() == 1 {
            let peer_id = self.args[0].clone();
            hole::ask_peer_address(&peer_id).await?;
        } else {
            let res="use args should be 1";
            print!("\n{}\n", res.red().bold());
        }
        print!("\n{}", ">>".white().bold());
        stdout().flush();
        Ok(())
    }
    async fn cmd_empty(&self) -> anyhow::Result<()> {
        print!("\n{}\n", "命令为空".red().on_black().bold());
        print!("{}", ">>".white().bold());
        stdout().flush()?;
        Ok(())
    }
    async fn cmd_other(&self) -> anyhow::Result<()> {
        let addr = hole::read_peer_address();
        if let Err(e) = addr {
            println!("\n地址解析失败:{}\n", e.to_string());
            print!("{}", ">>".white().bold());
            stdout().flush()?;
            return Ok(());
        }
        let peer = addr.unwrap();
        let order = self.order();
        order.send(peer).await;
        Ok(())
    }

    async fn cmd_send(&self) -> anyhow::Result<()> {
        let addr = hole::read_peer_address();
        dbg!(&addr);
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
        order.send(peer).await;
        Ok(())
    }
}

pub async fn test_cmd() -> anyhow::Result<()> {
    let ord = Order {
        input: "ls".to_string(),
        cmd: "others".to_string(),
        args: vec![],
        data: vec![],
    };
    let mut conf = hole::Conf::default();
    conf.swap_server = "39.96.40.177:4222".to_string();
    conf.set();
    hole::init_udp().await?;
    async_std::task::spawn(async {
        hole::listen().await;
    });
    hole::ask_peer_address("wq").await?;
    async_std::task::sleep(std::time::Duration::from_secs(9)).await;
    let peer = hole::read_peer_address()?;
    dbg!(&peer);
    ord.send(peer).await;
    loop{}

    Ok(())
}
