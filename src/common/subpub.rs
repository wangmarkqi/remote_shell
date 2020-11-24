use serde::{Deserialize, Serialize};
use reqwest::blocking::ClientBuilder;
use super::myfs::*;
use super::order::*;
use std::collections::HashMap;

pub fn conf_sub(sufix: &str) -> anyhow::Result<bool> {
    let host = get_dot_env("PASS_URL");
    let uid = get_dot_env("SLAVE_ID");
    let url = format!("{}/conf?topic={}_{}&consume=once&len=50&timeout=50", host, uid, sufix);
    let body = reqwest::blocking::get(&url)?.text()?;
    let dic: HashMap<String, String> = serde_json::from_str(&body)?;
    if dic["status"] == SUCCESS || dic["reason"] == "duplicate topic conf" {
        return Ok(true);
    }
    Ok(false)
}

pub fn pub_info(sufix: &str, s: &str) -> anyhow::Result<bool> {
    let host = get_dot_env("PASS_URL");
    let uid = get_dot_env("SLAVE_ID");
    let url = format!("{}/pub/{}_{}", host, uid, sufix);

    let res = ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .no_proxy()
        .build()
        .and_then(|c| c.post(&url).body(s.to_string()).send())?;
    let res1 = res.text()?;
    let dic: HashMap<String, String> = serde_json::from_str(&res1)?;
    if dic["status"] == SUCCESS {
        return Ok(true);
    }
    Ok(false)
}

fn _sub_info(sufix: &str) -> anyhow::Result<Vec<String>> {
    let host = get_dot_env("PASS_URL");
    let uid = get_dot_env("SLAVE_ID");
    let url = format!("{}/sub/{}_{}", host, uid,sufix);
    let body = reqwest::blocking::get(&url)?.text()?;
    let res: Vec<String> = serde_json::from_str(&body)?;
    Ok(res)
}

pub fn sub_info(sufix: &str) -> Vec<String> {
    let v=vec![];
    let res=_sub_info(sufix);
    match res{
        Ok(l)=>l,
        Err(e)=>v,
    }
}
