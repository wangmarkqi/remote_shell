use serde::{Deserialize, Serialize};
use super::utils::*;
use super::cmd::Cmd;
use udp_hole_punching as hole;
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
pub struct Order {
    pub input: String,
    pub cmd: String,
    pub args: Vec<String>,
    pub data: Vec<u8>,
}

impl Order {
    pub fn empty() -> Self {
        Order {
            input: "".to_string(),
            cmd: "".to_string(),
            args: vec![],
            data: vec![],
        }
    }
    pub fn finish()->Self{
        Order {
            input: Cmd::Finish.to_string(),
            cmd: Cmd::Finish.to_string(),
            args: vec![],
            data: vec![],
        }
    }
    pub fn start()->Self{
        Order {
            input: Cmd::Start.to_string(),
            cmd: Cmd::Start.to_string(),
            args: vec![],
            data: vec![],
        }
    }
    pub fn read_order_from_cache() -> (SocketAddr, Self) {
        let (addr, v) = hole::rec_from();
        let ord: Order = bincode::deserialize(&v).unwrap_or(Order::empty());
        (addr, ord)
    }

    pub fn send(&self, peer:SocketAddr) ->anyhow::Result<()>{
        let encoded: Vec<u8> = bincode::serialize(self).unwrap();
        hole::send(&encoded, peer);
        Ok(())
    }

    // pub fn to_input(&self) -> String {
    //     let mut v = vec![];
    //     v.push(self.cmd.to_owned());
    //     for i in self.args.iter() {
    //         v.push(i.to_string());
    //     }
    //     let joined = v.join(" ");
    //     joined
    // }
}


