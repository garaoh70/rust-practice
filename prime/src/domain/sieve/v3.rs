use super::SieveGenerator;

#[allow(dead_code)]
pub struct SieveV3 {
    sieves: Vec<u64>,
    max: usize,
}

#[allow(dead_code)]
impl SieveV3 {
    pub fn new(size: usize) -> Self {
        SieveV3 {
            sieves: vec![0u64; size / 64 + 1],
            max: size,
        }
    }
}

impl SieveGenerator for SieveV3 {
    fn mark_multiples(&mut self, step: usize) {
        // 偶数は篩にかけない
        if (step % 2) == 0 {
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

            self.sieves[offset] |= mask;
        }
    }

    fn next_unmarked(&mut self, index: usize) -> Option<usize> {
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
            .find(|&x| self.sieves[x / 64] & (1u64 << (x % 64)) == 0);
        match next {
            Some(v) => Some(v * 2 + 1),
            None => None,
        }
    }
}
