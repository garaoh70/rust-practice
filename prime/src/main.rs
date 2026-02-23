fn main() {
    let mut sieve = PrimeSieveV1::new();

    sieve.run();

    for prime in sieve.primes {
        println!("{}", prime);
    }
}

struct PrimeSieveV1 {
    sieve: Vec<bool>,
    primes: Vec<usize>,
}

impl PrimeSieveV1 {
    pub fn new() -> Self {
        PrimeSieveV1 {
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

    fn run(&mut self) {
        let mut current = Some(2usize);

        while let Some(x) = current {
            self.primes.push(x);
            self.mark_multiples(x);
            current = self.next_unmarked(x);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_numbers_less_than_50() {
        // Arrange
        let mut prime = PrimeSieveV1::new();
        let expected = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

        // Arrange
        prime.run();
        let actual = prime.primes;

        // Assert
        assert_eq!(actual, expected, "50未満の素数は一致しません");
    }
}
