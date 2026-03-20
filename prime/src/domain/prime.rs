use crate::domain::sieve::SieveGenerator;

pub trait PrimeGenerator {
    fn run(&mut self) -> PrimeResult;
}

pub struct Prime {
    sieve: Box<dyn SieveGenerator>,
}

pub struct PrimeResult {
    pub primes: Vec<usize>,
    pub elapsed: usize,
    pub sieve_name: &'static str,
}

impl Prime {
    pub fn new(sieve: Box<dyn SieveGenerator>) -> Self {
        Prime { sieve: sieve }
    }
}

impl PrimeGenerator for Prime {
    fn run(&mut self) -> PrimeResult {
        let mut current = 2usize;
        let mut primes: Vec<usize> = vec![];

        let start = std::time::Instant::now();
        loop {
            primes.push(current);
            self.sieve.mark_multiples(current);
            match self.sieve.next_unmarked(current) {
                Some(next) => current = next,
                _ => break,
            }
        }
        let elapsed = start.elapsed().as_millis();

        PrimeResult {
            primes: (primes),
            elapsed: (elapsed as usize),
            sieve_name: (self.sieve.name()),
        }
    }
}
