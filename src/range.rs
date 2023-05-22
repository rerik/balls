use rand::{thread_rng, Rng};

pub struct Range {
    pub min: f64,
    pub max: f64,
}

impl Range {
    pub const fn new(min: f64, max: f64) -> Self {
        Self {
            min, 
            max,
        }
    }
    pub fn gen(&self) -> f64 {
        let mut rng = thread_rng();
        rng.gen_range(self.min..=self.max)
    }
}