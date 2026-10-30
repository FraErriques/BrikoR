///  \class  Common::RandCpp11
///  \brief  Random generator wrapper[cite: 12, 13]

use rand::Rng;
use rand::rngs::StdRng;
use rand::SeedableRng;

pub struct RandCpp11 {
    rng: StdRng,
}

impl RandCpp11 {
    pub fn new() -> Self {
        RandCpp11 {
            rng: StdRng::from_entropy(),
        }
    }

    pub fn with_seed(seed: u64) -> Self {
        RandCpp11 {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn rand(&mut self) -> f64 {
        self.rng.gen_range(0.0..1.0)
    }
}