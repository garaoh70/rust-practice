use crate::repository::RepositoryGenerator;

use rusqlite::{Connection, Error};

pub struct Primes {
    path: &'static str,
}

impl Primes {
    pub fn new(path: &'static str) -> Self {
        Primes { path: path }
    }
}

impl RepositoryGenerator for Primes {
    fn extract_prime(&self, start: i64, end: i64) -> Result<Vec<i64>, Error> {
        // データベース接続
        let db = Connection::open(self.path)?;

        let mut stmt = db.prepare("SELECT value FROM primes WHERE value > ?1 AND value < ?2")?;

        let rows = stmt.query_map([start, end], |row| {
            let value: i64 = row.get(0)?;
            Ok(value)
        })?;

        Ok(rows.flatten().collect())
    }
}
