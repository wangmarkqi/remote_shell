use serde::{Deserialize, Serialize};
use reqwest::blocking::ClientBuilder;
use super::myfs::*;


pub const CMD_CD: &'static str = "cd";
pub const CMD_SEND: &'static str = "send";
pub const CMD_REC: &'static str = "rec";
pub const CMD_USE: &'static str = "use";
pub const CMD_RESTART: &'static str = "restart";
pub const CMD_WAIT: &'static str = "wait";
pub const FAIL: &'static str = "fail";
pub const SUCCESS: &'static str = "success";
pub const OVER: &'static str = "over";
pub const UP: &'static str = "up";
pub const DOWN: &'static str = "down";
pub const REQ_DURATION: &'static u64 = &40;
pub const STREAM_DURATION: &'static u64 = &40;
pub const TOPIC_ALL: &'static str = "topic_order";
pub const TOPIC_RESTART: &'static str = "topic_restart";

pub const API_REQ: &'static str = "/req";
pub const API_HB: &'static str = "/hb";
pub const API_RESP: &'static str = "/resp";



#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Order {
    pub slave: String,
    pub status: String,
    pub req: String,
    pub resp: String,
}

impl Order {
    pub fn new_from_host(input: &str, _slave: &str) -> Order {
        Order {
            slave: _slave.to_string(),
            status:SUCCESS.to_string(),
            req: input.to_string(),
            resp: "".to_string(),
        }
    }

}


pub fn post_order(url_t: &str, ord: &Order) -> anyhow::Result<Order> {
    let host = get_dot_env("PASS_URL");
    let timeout = *REQ_DURATION as i64;
    let slave = &ord.slave;
    let mut url = "".to_string();
    if url_t == API_REQ {
        url = format!("{}{}/{}/{}", host, API_REQ, slave, timeout);
    } else {
        url = format!("{}{}/{}", host, API_RESP, slave);
    }
    let body = json!(&ord);
    let res = ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .no_proxy()
        .build()
        .and_then(|c| c.post(&url).json(&body).send())?;
    let res1 = res.text()?;

    let res2 = serde_json::from_str(&res1);
    match res2 {
        Ok(res3) => Ok(res3),
        Err(e) => Err(anyhow!("error={}",&res1)),
    }
}

pub fn hb_ord(top: &str) -> anyhow::Result<Order> {
    let host = get_dot_env("PASS_URL");
    let url = format!("{}{}/{}", host, API_HB, top);
    let res = reqwest::blocking::get(&url)?;
    let res1 = res.text()?;
    let res2: Order = serde_json::from_str(&res1)?;
    Ok(res2)
}

