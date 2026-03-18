use super::SieveGenerator;

pub struct SieveV2 {
    flags: Vec<bool>,
}

impl SieveV2 {
    pub fn new(size: usize) -> Self {
        SieveV2 {
            flags: vec![false; size + 1],
        }
    }
}

impl SieveGenerator for SieveV2 {
    fn mark_multiples(&mut self, step: usize) {
        let len = self.flags.len();
        if step <= len / step {
            (step * step..len)
                .step_by(step)
                .for_each(|x| self.flags[x] = true);
        }
    }

    fn next_unmarked(&self, index: usize) -> Option<usize> {
        let next = index + 1;

        if self.flags.len() <= next {
            return None;
        }

        self.flags
            .iter()
            .skip(next)
            .position(|&x| x == false)
            .map(|x| x + next)
    }

    fn name(&self) -> &'static str {
        "SieveV2"
    }
}

#[cfg(test)]
mod test_v2 {
    use super::*;
    use crate::{Prime, PrimeGenerator};

    #[test]
    fn test_prime_numbers_less_than_50() {
        // Arrange
        let sieve = Box::new(SieveV2::new(50usize));
        let mut prime = Prime::new(sieve);
        let expected = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

        // Arrange
        prime.run();
        let actual = prime.values();

        // Assert
        assert_eq!(actual, expected, "50未満の素数は一致しません");
    }
}
