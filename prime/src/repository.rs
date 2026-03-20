use crate::cli::Arguments;
use crate::domain::prime::PrimeResult;

use async_trait::async_trait;

#[async_trait]
pub trait RepositoryGenerator {
    async fn append(&self, args: &Arguments, result: &PrimeResult);
    #[allow(dead_code)]
    async fn extract(&self, start: usize, length: usize) -> Vec<usize>;
}

pub mod sqlite;

pub use sqlite::SQLite;
