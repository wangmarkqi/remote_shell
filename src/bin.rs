pub mod slave;
pub mod common;
pub mod host;


#[macro_use]
extern crate anyhow;

// use std::io::{stdout, Write};
use common::cmd::Cmd;

fn main() -> anyhow::Result<()> {
    // let swap = "x.x.x.x:xxxx".to_string();
    // slave::slave_entrance::dispatch("wq", &swap,"./data/slave");
    // host::shell::run_shell(&swap,"./data/master");
    Ok(())
}
