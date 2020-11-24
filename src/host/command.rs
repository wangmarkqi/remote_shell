use serde::{Deserialize, Serialize};
use crate::common::myfs::*;
use crate::common::order::*;
use super::process::*;
use crossterm::style::Styler;
use super::shell::show;
use crossterm::{
    execute, queue,
    style::{self, Colorize}, cursor, terminal, Result,
};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Command {
    pub input: String,
    pub cmd: String,
    pub args: Vec<String>,
    pub slave: String,
    pub pos: i32,
}

impl Command {
    pub fn default() -> Self {
        Command {
            input: "".to_string(),
            cmd: "".to_string(),
            args: vec![],
            slave: "".to_string(),
            pos: -1,
        }
    }
    pub fn parse(&mut self, input: &str) -> anyhow::Result<()> {
        let mut input = input.replace("\r", "");

        if let Err(e) = parse_input(&input) {
            print!("--->:{}\n", e.to_string().red().on_black());
            return Ok(());
        }

        let (cmd, args) = parse_input(&input)?;
        self.cmd = cmd;
        self.args = args;
        self.input = input;
        Ok(())
    }
    pub fn run(&mut self) -> anyhow::Result<bool> {
        if self.cmd == CMD_USE {
            if self.args.len() == 1 {
                self.slave = self.args[0].clone();
                print!("\n--->:{}\n", self.slave.clone().green().bold());
                return Ok(true);
            } else {
                show(&self.slave.clone(), FAIL, "use args should be 1");
                return Ok(false);
            }
        }
        if self.slave == "" {
            show(&self.slave.clone(), FAIL, "slave can not empty, use <slaveid> first");
            return Ok(false);
        }
        let res = {
            match self.cmd.as_str() {
                "" => self.cmd_empty()?,
                CMD_RESTART => self.cmd_restart()?,
                CMD_SEND => self.cmd_send()?,
                CMD_REC => self.cmd_rec()?,
                // waite仅仅和cmd other有关，把waitefalse传给slave，发流接流
                _ => self.cmd_other()?,
            }
        };
        Ok(res)
    }
}