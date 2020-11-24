use crate::common::order::*;
use crate::common::myfs::*;
use crate::slave::sync_cmd::cmd_sync_others;
use std::io::Write;
use std::io::stdout;
use crate::common::myfs::*;
use super::process_cmds::*;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn dispatch() {
    print!("slave begin to work\n");

    let topic = get_dot_env("SLAVE_ID");
    loop {
        match _dispatch(&topic) {
            Ok(()) => print!("success one round\n"),
            Err(e) => {
                continue;
            }
        }
    }
}

pub fn _dispatch(topic: &str) -> anyhow::Result<()> {
    let mut ord = hb_ord(topic)?;

    if ord.slave.clone() != topic.to_string() {
        dbg!("not call me");
        return Ok(());
    }
    // 检查ord时候可以parse
    if let Err(e) = parse_input(&ord.req) {
        ord.status = FAIL.to_owned();
        ord.resp = format!("parse command error:{}", e);
        post_order(API_RESP, &ord)?;
        return Ok(());
    }

    let (cmd, _) = parse_input(&ord.req).unwrap();
    if cmd == CMD_RESTART {
        ord.status = SUCCESS.to_owned();
        ord.resp = "系统已经重启".to_string();
        post_order(API_RESP, &ord)?;
        system_shutdown::reboot().unwrap();
        return Ok(());
    }

    // 同步的处理主体
    dbg!("sync cmd");
    let (cmd, args) = parse_input(&ord.req).unwrap();
    let ord_from_slave = {
        match cmd.as_str() {
            CMD_CD if args.len() == 1 => cmd_cd(&ord),
            CMD_SEND => cmd_send(&ord),
            CMD_REC => cmd_rec(&ord),
            _ => cmd_sync_others(&ord),
        }
    };
    // 处理主体


    post_order(API_RESP, &ord_from_slave)?;
    return Ok(());

    Ok(())
}

