use rusqlite::Connection;
use rusqlite::Error;

pub trait RepositoryGenerator {
    fn extract_prime(
        &self,
        connection: &Connection,
        start: i64,
        end: i64,
    ) -> Result<Vec<i64>, Error>;
}

pub mod primes;

pub use primes::Primes;
