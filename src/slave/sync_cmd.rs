use std::process::{Command, Stdio};
use crate::common::order::*;
use crate::common::order::REQ_DURATION;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn cmd_sync_others(slave: &Order) -> Order{
    let mut  feedback=slave.clone();
    let slave1=slave.clone();
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    // 把命令行丢到另外一个线程，免得祸害自己
    let worker=thread::spawn(move || {
        let res = sync_cmd(&slave1);
        match res {
            Ok(s) => tx.send(s).unwrap(),
            Err(e) => {
                dbg!("cmd fail");
                let s = format!("无法执行cmd{}", e);
                tx.send(s).unwrap();
            }
        }
        ;
    });
    let secs = Duration::from_secs(*REQ_DURATION);
    let res = rx.recv_timeout(secs);
    // 执行错误也在ok中，拿到错误信息就不是错
    // 超过时间才是错误,没有返回
    match res {
        Ok(s) => {
            feedback.status=SUCCESS.to_string();
            feedback.resp=s;
        },
        Err(e) => {
            // 杀掉不工作的worker和rx,tx
            drop(rx);
            drop(worker);
            dbg!("i can not wait,time is over");
            feedback.status=FAIL.to_string();
            feedback.resp=format!("time out:{}", e);
        }
    }
    feedback
}

pub fn sync_cmd(slave: &Order) -> anyhow::Result<String> {
    let req = slave.req.clone();
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", &req])
            .stdout(Stdio::piped())
            .spawn()?
            .wait_with_output()?
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&req)
            .stdout(Stdio::piped())
            .spawn()?
            .wait_with_output()?
    };
    if output.status.success() {
        let res1 = output.stdout;
        let res2 = String::from_utf8_lossy(&res1);
        let res3 = res2.as_ref().to_string();
        return Ok(res3);
    }
    let err = output.stderr;
    let err = String::from_utf8_lossy(&err);
    let err = err.as_ref().to_string();
    Err(anyhow!(err))
}