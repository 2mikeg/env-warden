use std::collections::HashMap;

use crate::error::Error;
use rusqlite::{params, Connection, OpenFlags, Result};

#[derive(Debug)]
pub struct Manager {
    conn: Option<Connection>,
}

impl Default for Manager {
    fn default() -> Self {
        Self { conn: None }
    }
}

impl Manager {
    pub fn new(path: &str) -> Result<Self, Error> {
        let conn = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
        )?;

        let _ = conn.execute(
            "CREATE TABLE IF NOT EXISTS envs (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                value TEXT NOT NULL
            )",
            [],
        )?;

        Ok(Self { conn: Some(conn) })
    }

    pub fn search(&self) -> Result<HashMap<String, (String, String)>, Error> {
        eprintln!("Searching for env variables");

        let conn = self.conn.as_ref().unwrap();

        let mut stmt = conn.prepare("SELECT id, name, value FROM envs")?;
        let env_iter = stmt.query_map([], |row| {
            Ok((
                row.get::<_, i32>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?;

        let mut hmap = HashMap::<String, (String, String)>::new();

        for env in env_iter {
            let (id, name, value) = env?;
            let chmp = (name, value);
            hmap.insert(id.to_string(), chmp);
        }

        Ok(hmap)
    }

    pub fn insert(&self, name: &str, value: &str) -> Result<(), Error> {
        eprintln!("Inserting env variable");

        let conn = self.conn.as_ref().unwrap();

        conn.execute(
            "INSERT INTO envs (name, value) VALUES (?1, ?2)",
            params![name, value],
        )?;

        Ok(())
    }

    pub fn prune(&self) -> Result<(), Error> {
        eprintln!("Pruning env variables");

        let conn = self.conn.as_ref().unwrap();

        conn.execute("DELETE FROM envs", [])?;

        Ok(())
    }
}
