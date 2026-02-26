use super::SieveGenerator;

#[allow(dead_code)]
pub struct SieveV1 {
    sieves: Vec<bool>,
}

#[allow(dead_code)]
impl SieveV1 {
    pub fn new(size: usize) -> Self {
        SieveV1 {
            sieves: vec![false; size + 1],
        }
    }
}

impl SieveGenerator for SieveV1 {
    fn mark_multiples(&mut self, step: usize) {
        let len = self.sieves.len();

        (step..len)
            .step_by(step)
            .for_each(|x| self.sieves[x] = true);
    }

    fn next_unmarked(&mut self, index: usize) -> Option<usize> {
        self.sieves
            .iter()
            .skip(index)
            .position(|&x| x == false)
            .map(|x| x + index)
    }
}

#[cfg(test)]
mod test_v1 {
    use super::*;
    use crate::{Prime, PrimeGenerator};

    #[test]
    fn test_prime_numbers_less_than_50() {
        // Arrange
        let sieve = SieveV1::new(50usize);
        let mut prime = Prime::new(sieve);
        let expected = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

        // Arrange
        prime.run();
        let actual = prime.values();

        // Assert
        assert_eq!(actual, expected, "50未満の素数は一致しません");
    }
}
