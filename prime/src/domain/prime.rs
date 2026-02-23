use crate::domain::sieve::SieveV2;
use stopwatch::Stopwatch;

pub struct PrimeV2 {
    primes: Vec<usize>,
    is_completed: bool,
    elapsed: i64,
}

impl PrimeV2 {
    pub fn new() -> Self {
        PrimeV2 {
            primes: vec![],
            is_completed: false,
            elapsed: 0i64,
        }
    }

    pub fn run(&mut self, numbers: usize) {
        if self.is_completed {
            return;
        }

        let mut sw = Stopwatch::new();
        sw.start();
        let length = numbers + 1;
        let mut sieve = SieveV2::new(length);
        let mut current = Some(2usize);

        while let Some(x) = current {
            self.primes.push(x);

            if x <= length / x {
                (x * x..length).step_by(x).for_each(|x| sieve.mark(x));
            }

            current = sieve.next_unmarked(x + 1);
        }

        sw.stop();
        self.elapsed = sw.elapsed_ms();
        self.is_completed = true;
    }

    pub fn values(&mut self) -> Vec<usize> {
        self.primes.clone()
    }

    pub fn elapsed(&self) -> i64 {
        self.elapsed
    }
}

#[cfg(test)]
mod test_v2 {
    use super::*;

    #[test]
    fn test_prime_numbers_less_than_50() {
        // Arrange
        let mut prime = PrimeV2::new();
        let expected = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

        // Arrange
        prime.run(50);
        let actual = prime.primes;

        // Assert
        assert_eq!(actual, expected, "50未満の素数は一致しません");
    }
}