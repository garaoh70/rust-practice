fn main() {
    let prime = PrimeSieveV1::new();

    println!("Hello, world! {}", prime.sieve[0]);
}

struct PrimeSieveV1 {
    sieve: Vec<bool>,
    #[allow(dead_code)]
    prime: Vec<u32>,
}

impl PrimeSieveV1 {
    pub fn new() -> Self {
        PrimeSieveV1 {
            sieve: vec![false; 256],
            prime: vec![2u32],
        }
    }

    #[allow(dead_code)]
    pub fn make_sieve(&mut self, step: usize) {
        let len = self.sieve.len();

        (step..len)
            .step_by(step)
            .for_each(|x| self.sieve[x] = true);
    }

    #[allow(dead_code)]
    pub fn find_next_prime(&self, index: usize) -> Option<usize> {
        return self
            .sieve
            .iter()
            .skip(index)
            .position(|&x| x == false)
            .map(|x| x + index);
    }
}
