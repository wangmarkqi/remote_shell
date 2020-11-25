pub mod slave;
pub mod common;
pub mod host;

use async_std::task::block_on;

#[macro_use]
extern crate anyhow;

// use std::io::{stdout, Write};
use common::cmd::Cmd;

fn main() -> anyhow::Result<()> {
    let swap = "39.96.40.177:4222".to_string();
    // block_on(slave::slave_entrance::dispatch("wq", &swap));
    block_on(host::shell::run_shell(&swap));
    // block_on(host::cmd_process::test_cmd());

    Ok(())
}
