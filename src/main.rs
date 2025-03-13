use crate::cli::Cli;
use crate::db::Manager;
use clap::Parser;
use std::env;
use std::path::PathBuf;

mod cli;
mod db;
mod error;
fn main() {
    let home_dir = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let db_path = PathBuf::from(home_dir).join(".local/share/env-warden.sqlite");

    let manager_res = Manager::new(db_path.to_str().unwrap());
    let manager = manager_res.unwrap();

    let mut cli = Cli::parse();

    cli.execute(manager);
}
