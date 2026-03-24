use rusqlite::{Connection, Error};

use crate::repository::RepositoryGenerator;

pub struct Primes;

impl RepositoryGenerator for Primes {
    fn extract_prime(
        &self,
        connection: &Connection,
        start: i64,
        end: i64,
    ) -> Result<Vec<i64>, Error> {
        let mut stmt =
            connection.prepare("SELECT value FROM primes WHERE value >= ?1 AND value <= ?2")?;

        let rows = stmt.query_map([start, end], |row| {
            let value: i64 = row.get(0)?;
            Ok(value)
        })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }
}
