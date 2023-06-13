struct PrefixSum {
    vec: Vec<i32>,
    sum: Vec<i32>,
}

impl PrefixSum {
    pub fn new(vec: Vec<i32>) -> Self {
        let mut sum = vec![0; vec.len()];
        let mut value = 0;

        /*
        for i in 0..vec.len() {
            value += vec[i];
            sum[i] = value;
        }
        */
        vec.iter().enumerate().for_each(|(i, v)| {
            value += v;
            sum[i] = value;
        });

        Self { vec, sum }
    }

    fn range_sum(&self, low: usize, high: usize) -> Option<i32> {
        Some(self.sum.get(high)? - self.sum.get(low)?)
    }
}
