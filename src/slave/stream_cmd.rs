use std::process::{Command, Stdio, ChildStdout, ChildStderr};
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::io::prelude::*;
use crate::common::order::*;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use udp_hole_punching as hole;
use std::net::SocketAddr;
use crate::common::order::Order;
use std::time::{Instant, Duration};

pub async fn cmd_stream_others(peer: SocketAddr, ord: &Order) -> anyhow::Result<()> {
    let input_line = ord.input.clone();
    let mut child = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", &input_line])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&input_line)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?
    };

    // let start = Order::start();
    // start.send(peer).await;

    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let worker=thread::spawn(move || {
        loop {
            // 永远从错误No Data Read 退出
            dbg!("in the loop");
            // let stdout = child.stdout.take().ok_or(anyhow!("No Data Read"))?;
            let stdout = child.stdout.take().unwrap();
            let stderr = child.stderr.take().unwrap();
            let s1 = read_out(stdout);
            let s2 = read_out(stderr);
            if s1 != "".to_string() {
                tx.send(s1).unwrap();
            }
            if s2 != "".to_string() {
                tx.send(s2).unwrap();
            }
        }
    });
    let secs = Duration::from_secs(14);
    let res = rx.recv_timeout(secs);
        match res {
        Ok(s) => {
            let mut back = ord.clone();
            back.data = s.as_bytes().to_vec();
            back.send(peer).await;
        },
        Err(e) => {
            // 杀掉不工作的worker和rx,tx
            drop(rx);
            drop(worker);
            dbg!("i can not wait,time is over");
        }
    }

    // let finish = Order::finish();
    // finish.send(peer).await;

    Ok(())
}

fn read_out<T: std::io::Read>(mut t: T) -> String {
    let reader = BufReader::new(t);
    let lines = reader.lines();
    let mut res = vec![];
    for line in lines {
        let s = line.unwrap_or("decode error".to_string());
        dbg!(&s);
        if s != "".to_string() {
            res.push(s);

        }
    }
    let joined = res.join("\n");
    joined
}
