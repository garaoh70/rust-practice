#[allow(dead_code)]
pub trait SieveGenerator {
    fn mark_multiples(&mut self, step: usize);
    fn next_unmarked(&mut self, index: usize) -> Option<usize>;
}

pub mod v2;
pub mod v3;

pub use v2::SieveV2;
pub use v3::SieveV3;
