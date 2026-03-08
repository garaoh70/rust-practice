use super::SieveGenerator;

pub struct SieveV1 {
    flags: Vec<bool>,
}

impl SieveV1 {
    pub fn new(size: usize) -> Self {
        SieveV1 {
            flags: vec![false; size + 1],
        }
    }
}

impl SieveGenerator for SieveV1 {
    fn mark_multiples(&mut self, step: usize) {
        let len = self.flags.len();

        (step..len).step_by(step).for_each(|x| self.flags[x] = true);
    }

    fn next_unmarked(&self, index: usize) -> Option<usize> {
        self.flags
            .iter()
            .skip(index)
            .position(|&x| x == false)
            .map(|x| x + index)
    }

    fn name(&self) -> &'static str {
        "SieveV1"
    }
}

#[cfg(test)]
mod test_v1 {
    use super::*;
    use crate::{Prime, PrimeGenerator};

    #[test]
    fn test_prime_numbers_less_than_50() {
        // Arrange
        let sieve = Box::new(SieveV1::new(50usize));
        let mut prime = Prime::new(sieve);
        let expected = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

        // Arrange
        let result = prime.run();
        let actual = result.primes;

        // Assert
        assert_eq!(actual, expected, "50未満の素数は一致しません");
    }
}
