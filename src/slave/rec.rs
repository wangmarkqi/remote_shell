use std::env;
use std::path::Path;
use std::process::Command;
use crate::common::utils::*;
use crate::common::order::*;
use std::net::SocketAddr;
use super::stream_cmd::cmd_stream_others;
use crate::common::cmd::Cmd;
#[derive(PartialEq, Debug, Clone, Eq)]
pub struct Rec {
    ord: Order,
    peer: SocketAddr,
}

impl Rec {
    pub fn new(addr: SocketAddr, ord: &Order) -> Self {
        Rec {
            ord: ord.clone(),
            peer: addr,
        }
    }
    pub fn restart(&self) -> anyhow::Result<()> {
        let mut ord = self.ord.clone();
        let back = "系统已经重启".as_bytes().to_vec();
        ord.data = back;
        ord.send(self.peer)?;
        system_shutdown::reboot().unwrap();
        Ok(())
    }
    pub fn cd(&self) -> anyhow::Result<()> {
        let mut ord = self.ord.clone();
        let args = &ord.args;
        if args.len() != 1 {
            let res = "args should be 1 ".to_string();
            ord.data = res.as_bytes().to_vec();
            ord.send(self.peer)?;
            return Ok(());
        }
        let new_dir = args[0].clone();
        let old = env::current_dir().unwrap();
        if let Err(e) = env::set_current_dir(Path::new(&new_dir)) {
            let res = format!("error from cd:{}", e);
            ord.data = res.as_bytes().to_vec();
            ord.send(self.peer)?;
            return Ok(());
        }
        let res = format!("成功 cd:{}", &new_dir);
        ord.data = res.as_bytes().to_vec();
        ord.send(self.peer)?;
        Ok(())
    }

    pub fn send(&self) -> anyhow::Result<()> {
        let mut ord = self.ord.clone();
        let args = &ord.args;
        let data = &ord.data;
        if args.len() != 2 {
            let res = "args should be 2 ".to_string();
            ord.data = res.as_bytes().to_vec();
            ord.send(self.peer)?;
            return Ok(());
        }
        let savepath = &args[1];
        create_file_dir(savepath);
        let res = write_file_as_u8(savepath, data)?;
        ord.data = res.as_bytes().to_vec();
        ord.send(self.peer)?;
        Ok(())
    }

    pub fn rec(&self) -> anyhow::Result<()> {
        let mut ord = self.ord.clone();
        let args = &ord.args;
        if args.len() != 2 {
            let res = "args should be 2 ".to_string();
            ord.data = res.as_bytes().to_vec();
            ord.cmd=Cmd::Others.to_string();
            ord.send(self.peer)?;
            return Ok(());
        }
        dbg!(&args[1]);
        if let Ok(content) = read_file_as_u8(&args[1]) {
            ord.data = content;
        }else{
            let res=format!("无法读取文件{}",&args[1]);
            ord.data=res.as_bytes().to_vec();
        }
        ord.send(self.peer)?;
        Ok(())
    }
    pub fn others(&self) -> anyhow::Result<()> {
        cmd_stream_others(self.peer, &self.ord)
    }
}



