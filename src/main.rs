pub mod slave;
pub mod common;
pub mod host;



#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate serde_json;
// use std::io::{stdout, Write};


fn main() -> anyhow::Result<()> {
    // slave::slave_entrance::dispatch();
    host::shell::run_shell()?;

    Ok(())
}
