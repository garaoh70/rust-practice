pub trait PrimeGenerator {
    fn run(&mut self, max: usize);
    fn values(&self) -> &[usize];
    fn elapsed(&self) -> i64;
}

pub mod v1;
pub mod v2;
pub mod v3;

pub use v1::PrimeV1;
pub use v2::PrimeV2;
pub use v3::PrimeV3;
