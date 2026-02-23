pub struct SieveV2 {
    sieves: Vec<bool>,
}

impl SieveV2 {
    pub fn new(size: usize) -> Self {
        SieveV2 {
            sieves: vec![false; size],
        }
    }

    pub fn mark(&mut self, index: usize) {
        if self.sieves.len() <= index {
            return;
        }

        self.sieves[index] = true;
    }

    pub fn next_unmarked(&mut self, index: usize) -> Option<usize> {
        if self.sieves.len() <= index {
            return None;
        }

        self.sieves
            .iter()
            .skip(index)
            .position(|&x| x == false)
            .map(|x| x + index)
    }
}
