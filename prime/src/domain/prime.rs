use crate::domain::sieve::SieveGenerator;
use stopwatch::Stopwatch;

pub trait PrimeGenerator {
    fn run(&mut self);
    #[allow(dead_code)]
    fn values(&self) -> &[usize];
    fn elapsed(&self) -> i64;
}

#[allow(dead_code)]
pub struct Prime<S: SieveGenerator> {
    sieve: S,
    primes: Vec<usize>,
    is_completed: bool,
    elapsed: i64,
}

#[allow(dead_code)]
impl<S: SieveGenerator> Prime<S> {
    pub fn new(sieve: S) -> Self {
        Prime {
            sieve: sieve,
            primes: vec![],
            is_completed: false,
            elapsed: 0i64,
        }
    }
}

impl<S: SieveGenerator> PrimeGenerator for Prime<S> {
    fn run(&mut self) {
        if self.is_completed {
            return;
        }

        let mut current = Some(2usize);

        let mut sw = Stopwatch::new();
        sw.start();

        while let Some(x) = current {
            self.primes.push(x);
            self.sieve.mark_multiples(x);
            current = self.sieve.next_unmarked(x);
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
