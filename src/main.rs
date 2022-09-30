use anyhow::Result;
use std::ffi::OsString;
use std::io::{stdin, stdout, Write};
use std::process::Command;
use clap::Parser;

fn main() -> Result<()> {
    Args::parse().run()
}

#[derive(Parser, Debug)]
#[clap(about, author, version)]
struct Args {
    /// Name to use for window title
    #[clap(short, long)]
    name: Option<String>,

    /// The command to run
    command: OsString,
    /// Any arguments to the command
    args: Vec<OsString>,
}

impl Args {
    fn run(&self) -> Result<()> {
        loop {
            self.termstatus("");
            let result = Command::new(&self.command).args(&self.args).status()?;
            stdout().flush()?;
            self.termstatus(if result.success() { "🗸 " } else { "❌ " });
            print!("\n\nDone ({}).  ", result);
            if read_done()? {
                return Ok(());
            }
        }
    }

    pub fn termstatus(&self, status: &str) {
        if let Some(name) = &self.name {
            print!("\x1b]0;{}{}\x07", status, name);
            let _ = stdout().flush();
        }
    }
}

fn read_done() -> Result<bool> {
    let mut buffer = String::new();
    loop {
        print!("retry / quit? ");
        buffer.clear();
        stdout().flush()?;
        stdin().read_line(&mut buffer)?;
        let ans = buffer.trim();
        if !ans.is_empty() {
            if "retry".starts_with(ans) {
                return Ok(false);
            } else if "quit".starts_with(ans) {
                return Ok(true);
            }
        }
    }
}
