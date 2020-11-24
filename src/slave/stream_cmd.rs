use std::process::{Command, Stdio, ChildStdout, ChildStderr};
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::io::prelude::*;
use crate::common::order::*;
use crate::common::myfs::*;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use crate::common::subpub::{conf_sub, pub_info, sub_info};

pub fn reply_order_stream() {
    println!("stream begin to work");
    dbg!("stream begin to work");
    let me = get_dot_env("SLAVE_ID");
    let empty = Order::new_from_host("", "");

    let up_ok = conf_sub(UP).unwrap();
    let down_ok = conf_sub(DOWN).unwrap_or(false);
    // 管子没好
    if !up_ok || !down_ok {
        panic!("sub pub conf not work");
    }

    loop {
        let res = sub_info(UP);
        if res.len() == 0 {
            continue;
        }
        // 收到指令
        for s in res.iter() {
            let ord: Order = serde_json::from_str(&s).unwrap_or(empty.clone());
            // 不是我
            if ord.slave.clone() != me.to_string() {
                dbg!("not call me");
                continue;
            }

            if let Err(e) = cmd_stream_others(&ord) {
                let res = format!("{}", e);
                if res != "No Data Read".to_string() {
                    pub_info(DOWN, &res).unwrap_or(false);
                }
            }

            pub_info(DOWN, OVER).unwrap_or(false);
        }
    }
}


pub fn cmd_stream_others(slave: &Order) -> anyhow::Result<()> {
    let req = slave.req.clone();
    let mut child = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", &req])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&req)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?
    };

    loop {
        // 永远从错误No Data Read 退出
        let stdout = child.stdout.take().ok_or(anyhow!("No Data Read"))?;
        let stderr = child.stderr.take().ok_or(anyhow!("No Data Read"))?;
        read_then_pub(stdout)?;
        read_then_pub(stderr)?;
    }
    Ok(())
}

fn read_then_pub<T: std::io::Read>(mut t: T) -> anyhow::Result<()> {
    let reader = BufReader::new(t);
    let lines = reader.lines();
    for line in lines {
        let s = line.unwrap_or("decode error".to_string());
        dbg!(&s);
        if s != "".to_string() {
            pub_info(DOWN, &s)?;
        }
    }
    Ok(())
}
