///  \class  Common::MonteCarlo::ClassicalDiscreteGenerator
///  \brief  Discrete random extraction generator[cite: 9, 10]

use crate::monte_carlo::delta_omega::DeltaOmega;
use rand::Rng;
use rand::rngs::StdRng;
use rand::SeedableRng;

pub struct ClassicalDiscreteGenerator {
    current_seed: u64,
    generator_sup: f64,
    min: i32,
    sup: i32,
    the_interval_measure: f64,
    omothetia: f64,
    translation: f64,
    discrete_population: Vec<i32>,
    frequency_distribution: Vec<DeltaOmega>,
    rng: StdRng,
}

impl ClassicalDiscreteGenerator {
    pub fn new(seed: Option<u64>, left: i32, right: i32) -> Result<Self, String> {
        if right - left <= 0 {
            return Err("wrong boundaries: should be left<right.".to_string());
        }
        let current_seed = seed.unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        });
        let rng = StdRng::seed_from_u64(current_seed);

        let mut rnd = ClassicalDiscreteGenerator {
            current_seed,
            generator_sup: i32::MAX as f64,
            min: 0,
            sup: 0,
            the_interval_measure: 0.0,
            omothetia: 0.0,
            translation: 0.0,
            discrete_population: Vec::new(),
            frequency_distribution: Vec::new(),
            rng,
        };
        rnd.reset_extraction_interval(left, right);
        Ok(rnd)
    }

    pub fn reset_extraction_interval(&mut self, left: i32, right: i32) {
        self.min = left;
        self.sup = right;
        self.the_interval_measure = (right - left) as f64;
        self.omothetia = self.the_interval_measure / self.generator_sup;
        self.translation = left as f64;
        self.discrete_population.clear();
        self.frequency_distribution.clear();
    }

    pub fn next_integer_in_interval(&mut self) -> i32 {
        let randomized = self.rng.gen_range(0..i32::MAX) as f64;
        let transformed = randomized * self.omothetia + self.translation;
        let rounded = transformed.round() as i32;
        self.discrete_population.push(rounded);
        rounded
    }

    pub fn build_omega(&mut self, partition_left: i32, partition_right: i32) {
        for pos in partition_left..=partition_right {
            self.frequency_distribution.push(DeltaOmega::new(pos as f64, 1e-80));
        }
    }

    pub fn build_discrete_frequency_distribution(&mut self) {
        let pop_card = self.discrete_population.len() as f64;
        if pop_card == 0.0 {
            return;
        }
        let weight = 1.0 / pop_card;
        let population = self.discrete_population.clone();

        for val in population {
            for item in &mut self.frequency_distribution {
                if item.belongs_to_delta_omega(val as f64) {
                    item.category_frequency += weight;
                    item.delta_omega_cardinality += 1;
                    break;
                }
            }
        }
    }
}