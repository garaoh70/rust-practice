use super::SieveGenerator;

#[allow(dead_code)]
pub struct SieveV4 {
    flags: Vec<u128>,
    max: usize,
}

#[allow(dead_code)]
impl SieveV4 {
    pub fn new(size: usize) -> Self {
        SieveV4 {
            flags: vec![0u128; size / 128 + 1],
            max: size,
        }
    }
}

impl SieveGenerator for SieveV4 {
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
            let offset = value / 128;
            let mask = 1u128 << (value % 128);

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
            .find(|&x| self.flags[x / 128] & (1u128 << (x % 128)) == 0);
        match next {
            Some(v) => Some(v * 2 + 1),
            None => None,
        }
    }
}

#[cfg(test)]
mod test_v4 {
    use super::*;
    use crate::{Prime, PrimeGenerator};

    #[test]
    fn test_prime_numbers_less_than_1000() {
        // Arrange
        let sieve = SieveV4::new(1000usize);
        let mut prime = Prime::new(sieve);
        let expected = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271,
            277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379,
            383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479,
            487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599,
            601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701,
            709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823,
            827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941,
            947, 953, 967, 971, 977, 983, 991, 997,
        ];

        // Arrange
        prime.run();
        let actual = prime.values();

        // Assert
        assert_eq!(actual, expected, "1000未満の素数は一致しません");
    }
}
