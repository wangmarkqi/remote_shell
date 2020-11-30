pub mod slave;
pub mod common;
pub mod host;
#[macro_use]
extern crate anyhow;
pub use slave::slave_entrance::dispatch;
pub use host::shell::run_shell;
pub use host::shell_process::ShellProcess;
pub use host::cmd_process::CmdProcess;

