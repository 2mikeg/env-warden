use crate::cli::Cli;
use crate::db::Manager;
use clap::Parser;

mod cli;
mod db;
mod error;
fn main() {
    let manager_res = Manager::new("/Users/mike/.local/share/db.sqlite");
    let manager = manager_res.unwrap();

    let mut cli = Cli::parse();

    cli.execute(manager);
}
