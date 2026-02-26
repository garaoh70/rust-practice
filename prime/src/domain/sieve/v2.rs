use super::SieveGenerator;

#[allow(dead_code)]
pub struct SieveV2 {
    sieves: Vec<bool>,
}

#[allow(dead_code)]
impl SieveV2 {
    pub fn new(size: usize) -> Self {
        SieveV2 {
            sieves: vec![false; size],
        }
    }
}

impl SieveGenerator for SieveV2 {
    fn mark_multiples(&mut self, step: usize) {
        let length = self.sieves.len();
        if step <= length / step {
            (step * step..length)
                .step_by(step)
                .for_each(|x| self.sieves[x] = true);
        }
    }

    fn next_unmarked(&mut self, index: usize) -> Option<usize> {
        let next = index + 1;

        if self.sieves.len() <= next {
            return None;
        }

        self.sieves
            .iter()
            .skip(next)
            .position(|&x| x == false)
            .map(|x| x + next)
    }
}
