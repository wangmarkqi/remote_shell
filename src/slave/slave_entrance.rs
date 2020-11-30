use crate::common::order::*;
use crate::common::utils::*;
use std::io::Write;
use std::io::stdout;
use crate::common::utils::*;
use crate::common::cmd::Cmd;
use super::rec::Rec;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::time::Duration;
use udp_hole_punching as hole;


pub fn dispatch(id: &str, swap_server: &str, db_path: &str) {
    let mut conf = hole::Conf::default();
    conf.swap_server = swap_server.to_string();
    conf.db_path = db_path.to_string();
    conf.id = id.to_string();
    conf.set();

    hole::init_udp();
    std::thread::spawn(|| {
        hole::listen();
    });


    print!("******************slave begin to work\n");
    loop {
        match _dispatch() {
            Ok(_) => {}
            Err(e) => {}
        }
    }
}

pub fn _dispatch() -> anyhow::Result<()> {
    let (peer, mut ord) = Order::read_order_from_cache();

    if ord.cmd == "".to_string() {
        return Ok(());
    }
    dbg!("receive ord");
    dbg!(&peer);
    let rec = Rec::new(peer, &ord);

    let cmd = Cmd::from_str(&ord.cmd);
    // 检查ord时候可以parse
    dbg!(&cmd);

    let start = Order::start();
    start.send(peer);

    let res = {
        match cmd {
            Cmd::Restart => rec.restart(),
            Cmd::Cd => rec.cd(),
            Cmd::Send => rec.send(),
            Cmd::Rec => rec.rec(),
            _ => rec.others(),
        }
    };
    if let Err(e) = res {
        let res = format!("错误：{}", e);
        ord.data = res.as_bytes().to_vec();
        ord.send(peer)?;
    }

    let finish = Order::finish();
    finish.send(peer);

    Ok(())
}

