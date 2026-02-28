use super::SieveGenerator;

#[allow(dead_code)]
pub struct SieveV3 {
    flags: Vec<u64>,
    max: usize,
}

#[allow(dead_code)]
impl SieveV3 {
    pub fn new(size: usize) -> Self {
        SieveV3 {
            flags: vec![0u64; size / 64 + 1],
            max: size,
        }
    }
}

impl SieveGenerator for SieveV3 {
    fn mark_multiples(&mut self, step: usize) {
        // 偶数は篩にかけない
        if (step & 1) == 0 {
            return;
        }

        // 篩にかける必要のない場合終了
        if step > self.max / step {
            return;
        }

        // 偶数を切り捨てて篩をかける
        for value in (step * step..=self.max).step_by(step * 2).map(|x| x >> 1) {
            let offset = value / 64;
            let mask = 1u64 << (value % 64);

            self.flags[offset] |= mask;
        }
    }

    fn next_unmarked(&self, index: usize) -> Option<usize> {
        // ２の場合は３を返す
        if index <= 2 {
            return Some(3);
        }
        if self.max <= index {
            return None;
        }

        let next = (index + 2..=self.max)
            .step_by(2)
            .map(|x| x >> 1)
            .find(|&x| self.flags[x / 64] & (1u64 << (x % 64)) == 0);
        match next {
            Some(v) => Some(v * 2 + 1),
            None => None,
        }
    }
}

#[cfg(test)]
mod test_v3 {
    use super::*;
    use crate::{Prime, PrimeGenerator};

    #[test]
    fn test_prime_numbers_less_than_50() {
        // Arrange
        let sieve = SieveV3::new(50usize);
        let mut prime = Prime::new(sieve);
        let expected = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

        // Arrange
        prime.run();
        let actual = prime.values();

        // Assert
        assert_eq!(actual, expected, "50未満の素数は一致しません");
    }

    #[test]
    fn test_prime_numbers_less_than_100() {
        // Arrange
        let sieve = SieveV3::new(100usize);
        let mut prime = Prime::new(sieve);
        let expected = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];

        // Arrange
        prime.run();
        let actual = prime.values();

        // Assert
        assert_eq!(actual, expected, "100未満の素数は一致しません");
    }
}
