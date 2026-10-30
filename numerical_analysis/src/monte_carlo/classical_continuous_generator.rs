///  \class  Common::MonteCarlo::ClassicalContinuousGenerator
///  \brief  Continuous random extraction generator[cite: 7, 8]

use crate::monte_carlo::delta_omega::DeltaOmega;
use rand::Rng;
use rand::rngs::StdRng;
use rand::SeedableRng;

pub struct ClassicalContinuousGenerator {
    current_seed: u64,
    generator_sup: f64,
    min: f64,
    sup: f64,
    the_interval_measure: f64,
    omothetia: f64,
    translation: f64,
    continuous_population: Vec<f64>,
    frequency_distribution: Vec<DeltaOmega>,
    rng: StdRng,
}

impl ClassicalContinuousGenerator {
    pub fn new(seed: Option<u64>, left: f64, right: f64) -> Result<Self, String> {
        if right - left <= 0.0 {
            return Err("wrong boundaries: should be left<right.".to_string());
        }
        let current_seed = seed.unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        });
        let rng = StdRng::seed_from_u64(current_seed);

        let mut rnd = ClassicalContinuousGenerator {
            current_seed,
            generator_sup: i32::MAX as f64,
            min: 0.0,
            sup: 0.0,
            the_interval_measure: 0.0,
            omothetia: 0.0,
            translation: 0.0,
            continuous_population: Vec::new(),
            frequency_distribution: Vec::new(),
            rng,
        };
        rnd.reset_extraction_interval(left, right);
        Ok(rnd)
    }

    pub fn reset_extraction_interval(&mut self, left: f64, right: f64) {
        self.min = left;
        self.sup = right;
        self.the_interval_measure = right - left;
        self.omothetia = self.the_interval_measure / self.generator_sup;
        self.translation = left;
        self.continuous_population.clear();
        self.frequency_distribution.clear();
    }

    pub fn next_double_in_interval(&mut self) -> f64 {
        let original_extracted = self.rng.gen_range(0..i32::MAX) as f64;
        let temp = original_extracted * self.omothetia + self.translation;
        if self.min <= temp && self.sup > temp {
            self.continuous_population.push(temp);
        }
        temp
    }

    pub fn build_omega(&mut self, partition_left: f64, partition_right: f64) {
        let eta = (partition_right - partition_left) / 20.0;
        let mut mediana = partition_left;
        let mut position = partition_left - 1.0e-80;
        while mediana < partition_right {
            position += 2.0 * eta;
            mediana = position + eta;
            if mediana >= partition_right {
                break;
            }
            self.frequency_distribution.push(DeltaOmega::new(mediana, eta));
        }
    }

    pub fn build_continuous_frequency_distribution(&mut self) {
        let pop_card = self.continuous_population.len() as f64;
        if pop_card == 0.0 {
            return;
        }
        let weight = 1.0 / pop_card;
        let population = self.continuous_population.clone();

        for val in population {
            for item in &mut self.frequency_distribution {
                if item.belongs_to_delta_omega(val) {
                    item.category_frequency += weight;
                    item.delta_omega_cardinality += 1;
                    break;
                }
            }
        }
    }

    pub fn show_current_seed(&self) -> u64 {
        self.current_seed
    }
}