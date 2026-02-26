use super::PrimeGenerator;
use crate::domain::sieve::{SieveGenerator, SieveV3};
use stopwatch::Stopwatch;

#[allow(dead_code)]
pub struct PrimeV3 {
    primes: Vec<usize>,
    is_completed: bool,
    elapsed: i64,
}

#[allow(dead_code)]
impl PrimeV3 {
    pub fn new() -> Self {
        PrimeV3 {
            primes: vec![],
            is_completed: false,
            elapsed: 0i64,
        }
    }
}

impl PrimeGenerator for PrimeV3 {
    fn run(&mut self, numbers: usize) {
        if self.is_completed {
            return;
        }

        let mut sw = Stopwatch::new();
        sw.start();
        let mut sieve = SieveV3::new(numbers);
        let mut current = Some(2usize);

        while let Some(x) = current {
            self.primes.push(x);
            sieve.mark_multiples(x);
            current = sieve.next_unmarked(x);
        }

        sw.stop();
        self.elapsed = sw.elapsed_ms();
        self.is_completed = true;
    }

    fn values(&self) -> &[usize] {
        &self.primes
    }

    fn elapsed(&self) -> i64 {
        self.elapsed
    }
}

#[cfg(test)]
mod test_v3 {
    use super::*;

    #[test]
    fn test_prime_numbers_less_than_50() {
        // Arrange
        let mut prime = PrimeV3::new();
        let expected = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

        // Arrange
        prime.run(50);
        let actual = prime.primes;

        // Assert
        assert_eq!(actual, expected, "50未満の素数は一致しません");
    }

    #[test]
    fn test_prime_numbers_less_than_100() {
        // Arrange
        let mut prime = PrimeV3::new();
        let expected = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];

        // Arrange
        prime.run(100);
        let actual = prime.primes;

        // Assert
        assert_eq!(actual, expected, "100未満の素数は一致しません");
    }
}
