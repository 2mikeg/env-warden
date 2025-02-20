use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

fn main() {
    // Write the db
    let db_path = Path::new("/var/db/env-warden.db");
    let _ = File::create(db_path).unwrap();
}
