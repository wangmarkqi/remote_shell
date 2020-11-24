use crate::common::order::*;
use crate::common::myfs::*;
use std::collections::HashMap;
use std::io::stdout;
use std::io::Write;
use std::path::Path;
use crossterm::style::Colorize;
use crossterm::style::Styler;
use crate::host::command::Command;
use super::shell::show;

pub trait Trans {
    fn send_order(&self, ord: &Order) -> Order;
}

impl Trans for Command {
    fn send_order(&self, ord: &Order) -> Order {
        let res = err2ord(post_order(API_REQ, ord), ord);
        res
    }
}


fn err2ord(ord_res: anyhow::Result<Order>, ord_origin: &Order) -> Order {
    match ord_res {
        Ok(o) => o,
        Err(e) => {
            let mut res = ord_origin.clone();
            res.resp = format!("{}:{}", FAIL, e);
            res
        }
    }
}



