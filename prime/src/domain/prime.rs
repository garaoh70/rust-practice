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
