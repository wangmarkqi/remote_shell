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


pub async fn dispatch(id: &str, swap_server: &str) {
    let mut conf = hole::Conf::default();
    conf.swap_server = swap_server.to_string();
    conf.id = id.to_string();
    conf.set();
    hole::init_udp().await.unwrap();
    async_std::task::spawn(async {
        hole::listen().await;
    });
    print!("******************slave begin to work\n");
    loop {
        match _dispatch().await {
            Ok(_) => {}
            Err(e) => {}
        }
    }
}

pub async fn _dispatch() -> anyhow::Result<()> {
    let (peer, mut ord) = Order::read_order_from_cache();

    if ord.cmd == "".to_string() {
        return Ok(());
    }
    dbg!("receive ord");
    dbg!(&ord);
    dbg!(&peer);
    let rec = Rec::new(peer, &ord);

    let cmd = Cmd::from_str(&ord.cmd);
    // 检查ord时候可以parse

    let start = Order::start();
    start.send(peer).await;

    let res = {
        match cmd {
            Cmd::Restart => rec.restart().await,
            Cmd::Cd => rec.cd().await,
            Cmd::Send => rec.send().await,
            Cmd::Rec => rec.rec().await,
            _ => rec.others().await,
        }
    };
    if let Err(e) = res {
        let res = format!("错误：{}", e);
        ord.data = res.as_bytes().to_vec();
        ord.send(peer).await?;
    }

    let finish = Order::finish();
    finish.send(peer).await;

    Ok(())
}

