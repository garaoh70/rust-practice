use crate::cli::Arguments;
use crate::domain::prime::PrimeResult;

use async_trait::async_trait;

#[async_trait]
pub trait RepositoryGenerator {
    async fn append(&self, args: &Arguments, result: &PrimeResult);
    #[allow(dead_code)]
    async fn extract(&self, args: &Arguments) -> Vec<usize>;
}

pub mod file_binary;
pub mod file_vec;
pub mod sqlite;

pub use file_binary::BinaryU64;
#[allow(unused_imports)]
pub use file_vec::BinaryVector;
pub use sqlite::SQLite;
