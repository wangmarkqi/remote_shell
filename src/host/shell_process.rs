use serde::{Deserialize, Serialize};
use crate::common::utils::*;
use crate::common::order::*;
use crate::common::cmd::Cmd;
use super::cmd_process::*;
use crossterm::style::Styler;
use super::shell::show;
use udp_hole_punching as hole;
use crossterm::{
    execute, queue,
    style::{self, Colorize}, cursor, terminal, Result,
};

#[derive( PartialEq, Debug, Clone)]
pub struct ShellProcess{
    pub input: String,
    pub cmd: Cmd,
    pub args: Vec<String>,
    pub pos: i32,
}

impl ShellProcess{
    pub fn default() -> Self {
        ShellProcess{
            input: "".to_string(),
            cmd: Cmd::None,
            args: vec![],
            pos: -1,
        }
    }
    pub fn order(&self) -> Order {
        Order {
            input:self.input.to_string().clone(),
            cmd: self.cmd.to_string().clone(),
            args: self.args.clone(),
            data: vec![],
        }
    }
    pub fn parse(&mut self, input: &str) -> anyhow::Result<()> {
        let mut input = input.replace("\r", "");

        if let Err(e) = parse_input(&input) {
            print!("\n{}解析错误:{}\n",input, e.to_string().red().on_black());
            print!("{}", ">>".white().bold());
            return Err(e);
        }

        let (cmd, args) = parse_input(&input)?;
        self.cmd = Cmd::from_str(&cmd);
        self.args = args;
        self.input = input;
        Ok(())
    }
    pub async fn run(&self)  {
        let res = {
            match self.cmd {
                Cmd::Use => self.cmd_use().await,
                Cmd::Where => self.cmd_where().await,
                Cmd::None => self.cmd_empty().await,
                Cmd::Send => self.cmd_send().await,
                // waite仅仅和cmd other有关，把waitefalse传给slave，发流接流
                _ => self.cmd_other().await,
            }
        };
        if let Err(e)=res{
            let e=format!("执行错误：{}",e);
            show(&self.cmd.to_string(),&e);
        }
    }
}