use crate::domain::sieve::SieveGenerator;

pub trait PrimeGenerator {
    fn run(&mut self);
    fn values(&self) -> &[usize];
    fn elapsed(&self) -> u128;
}

pub struct Prime {
    sieve: Box<dyn SieveGenerator>,
    primes: Vec<usize>,
    is_completed: bool,
    elapsed: u128,
}

impl Prime {
    pub fn new(sieve: Box<dyn SieveGenerator>) -> Self {
        Prime {
            sieve: sieve,
            primes: vec![],
            is_completed: false,
            elapsed: 0u128,
        }
    }
}

impl PrimeGenerator for Prime {
    fn run(&mut self) {
        if self.is_completed {
            return;
        }

        let mut current = 2usize;

        let start = std::time::Instant::now();
        loop {
            self.primes.push(current);
            self.sieve.mark_multiples(current);
            match self.sieve.next_unmarked(current) {
                Some(next) => current = next,
                _ => break,
            }
        }
        self.elapsed = start.elapsed().as_millis();

        self.is_completed = true;
    }

    fn values(&self) -> &[usize] {
        &self.primes
    }

    fn elapsed(&self) -> u128 {
        self.elapsed
    }
}
