use crate::cli::{Cli, Commands};
use crate::db::Manager;
use clap::Parser;

use rusqlite::{params, Connection, Result};

mod cli;
mod db;
mod error;
fn main() {
    let manager_res = Manager::new("/var/db/env-warden.db");
    eprint!("Manager: {:?}", manager_res);
    let manager = manager_res.unwrap();

    let mut cli = Cli::parse();

    cli.execute(manager);
}
