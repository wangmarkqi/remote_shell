pub mod slave;
pub mod common;
pub mod host;


#[macro_use]
extern crate anyhow;

// use std::io::{stdout, Write};
use common::cmd::Cmd;

fn main() -> anyhow::Result<()> {
    let swap = "39.96.40.177:4222".to_string();
    // slave::slave_entrance::dispatch("21risk", &swap,"./data/slave");
    host::shell::run_shell(&swap,"./data/master");
    Ok(())
}
