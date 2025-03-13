use colored::*;
use rusqlite;
use rusqlite::{Connection, Result};
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    eprintln!("{}", "\u{1F680} Installing env-warden ...".cyan());

    let path = "/var/db/env-warden.db";

    // Verificar si el directorio existe
    let db_dir = Path::new("/var/db");
    if !db_dir.exists() {
        eprintln!(
            "{}",
            "\u{26A0} Directory /var/db does not exist. Creating it...".yellow()
        );
        fs::create_dir_all(db_dir).unwrap();
    }

    eprintln!("{}", "\u{1F528} Creating DB ...".blue());

    // Conectar a SQLite y crear la DB si no existe
    let conn = Connection::open(path).unwrap_or_else(|e| {
        eprintln!("{}", format!("\u{274C} Failed to open DB: {}", e).red());
        std::process::exit(1);
    });

    // Intentar crear una tabla para validar permisos de escritura
    conn.execute(
        "CREATE TABLE IF NOT EXISTS test_table (id INTEGER PRIMARY KEY, name TEXT);",
        [],
    )
    .unwrap_or_else(|e| {
        eprintln!("{}", format!("\u{274C} Failed to write to DB: {}", e).red());
        std::process::exit(1);
    });

    eprintln!("{}", "\u{2705} env-warden installed successfully!".green());
    Ok(())
}
