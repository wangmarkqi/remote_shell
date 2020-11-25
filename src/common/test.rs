use async_std::task::{spawn,block_on};
use udp_hole_punching as hole;
use std::net::SocketAddr;
use std::time::Duration;

pub async fn test_callee_listen() -> anyhow::Result<()> {
    let mut conf = hole::Conf::default();
    conf.swap_server = "39.96.40.177:4222".to_string();
    conf.id = "wq".to_string();
    conf.set();
    hole::init_udp().await?;
    spawn(async {
        hole::listen().await;
    });

    loop {
        let (addr, v) = hole::rec_from();
        if v.len() > 0 {
            let s = String::from_utf8_lossy(&v);
            dbg!("callee rec res");
            dbg!(s.len());
            let back = {
                let mut v = vec![];
                for i in 0..1024 * 10 {
                    v.push(8 as u8);
                }
                v
            };
            hole::send(&back, addr).await?;
        }
    };
    Ok(())
}

pub async fn test_caller_api() -> anyhow::Result<()> {
    let mut conf = hole::Conf::default();
    conf.swap_server = "39.96.40.177:4222".to_string();
    conf.set();
    hole::init_udp().await?;
    spawn(async {
        hole::listen().await;
    });
    hole::ask_peer_address("wq").await?;
    async_std::task::sleep(Duration::from_secs(9)).await;
    let addr = hole::read_peer_address()?;
    dbg!("begin");

    let msg = {
        let mut v = vec![];
        for i in 0..1024 * 10 {
            v.push(8 as u8);
        }
        v
    };

    loop {
        hole::send(&msg, addr).await?;
        async_std::task::sleep(Duration::from_secs(4)).await;
        let (addr, v) = hole::rec_from();
        if v.len() > 0 {
            let s = String::from_utf8_lossy(&v);
            dbg!(s.len());
        }
    }
    Ok(())
}