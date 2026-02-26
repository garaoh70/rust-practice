use super::PrimeGenerator;

#[allow(dead_code)]
pub struct PrimeV1 {
    sieve: Vec<bool>,
    primes: Vec<usize>,
}

#[allow(dead_code)]
impl PrimeV1 {
    pub fn new() -> Self {
        PrimeV1 {
            sieve: vec![false; 50],
            primes: vec![],
        }
    }

    fn mark_multiples(&mut self, step: usize) {
        let len = self.sieve.len();

        (step..len).step_by(step).for_each(|x| self.sieve[x] = true);
    }

    fn next_unmarked(&self, index: usize) -> Option<usize> {
        self.sieve
            .iter()
            .skip(index)
            .position(|&x| x == false)
            .map(|x| x + index)
    }
}

impl PrimeGenerator for PrimeV1 {
    fn run(&mut self, _: usize) {
        let mut current = Some(2usize);

        while let Some(x) = current {
            self.primes.push(x);
            self.mark_multiples(x);
            current = self.next_unmarked(x);
        }
    }

    fn values(&self) -> &[usize] {
        &self.primes
    }

    fn elapsed(&self) -> i64 {
        0i64
    }
}

#[cfg(test)]
mod test_v1 {
    use super::*;

    #[test]
    fn test_prime_numbers_less_than_50() {
        // Arrange
        let mut prime = PrimeV1::new();
        let expected = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

        // Arrange
        prime.run(0);
        let actual = prime.values();

        // Assert
        assert_eq!(actual, expected, "50未満の素数は一致しません");
    }
}
