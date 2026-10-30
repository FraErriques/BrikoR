///  \class  Numerics::Primes
///  \brief  An Analytic Number Theory class for Prime generation and Riemann-zeta evaluation
///  \author F. Erriques (Translated to Rust)


use super::internal_algos;
use crate::real_integration;
use crate::complex_integration;
use crate::complex::Complex;
use common::common_data::Crash;
use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::path::Path;

#[derive(Clone, Debug)]
pub struct SingleFactor {
    pub factor_base: u64,
    pub factor_multiplicity: u64,
}

#[derive(Clone, Debug)]
pub struct DumpElement {
    pub ordinal: u64,
    pub prime: u64,
}

#[derive(Clone, Debug)]
pub struct AsinglePointInStream {
    pub ordinal: u64,
    pub prime: u64,
    pub start_position_of_record: u64,
    pub end_position_of_record: u64,
}

#[derive(Clone, Debug)]
pub struct LogIntegralPillarPoint {
    pub threshold: u64,
    pub log_integral: u64,
}

#[derive(Clone, Debug)]
pub struct Omega {
    pub small: i32,
    pub big: i32,
}

pub struct Primes {
    pub soglia_distanza: u32,
    pub memory_mapped_dump: BTreeMap<u64, u64>,
    pub log_integral_pillars: Vec<LogIntegralPillarPoint>,
    pub is_healthly_constructed: bool,
    pub can_operate: bool,
    sequential_dump_path: Option<String>,
    random_dump_path: Option<String>,
    mesh_renewal_path: Option<String>,
    local_integral_path: Option<String>,
    global_integral_path: Option<String>,
}

impl Primes {
    pub fn new(semi_amplitude_of_each_map_segment: u32) -> Self {
        let mut obj = Self {
            soglia_distanza: semi_amplitude_of_each_map_segment,
            memory_mapped_dump: BTreeMap::new(),
            log_integral_pillars: Vec::new(),
            is_healthly_constructed: true,
            can_operate: true,
            sequential_dump_path: None,
            random_dump_path: None,
            mesh_renewal_path: None,
            local_integral_path: None,
            global_integral_path: None,
        };

        obj.feed_sequential_dump_path();
        obj.feed_custom_dump_path();
        obj.feed_mesh_specification_path();
        obj.feed_local_integral_path();
        obj.feed_global_integral_path();
        let _ = obj.distribution_function_from_existing_mesh();

        if obj.sequential_dump_path.is_none() || obj.random_dump_path.is_none() {
            obj.is_healthly_constructed = false;
        }

        obj
    }

    pub fn sequential_stream_size(&self) -> Result<u64, Crash> {
        let path = self.sequential_dump_path.as_ref().ok_or_else(|| {
            Crash::new("Sequential dump path not configured.")
        })?;
        let mut file = File::open(path).map_err(|_| Crash::new("Unable to open sequential dump."))?;
        file.seek(SeekFrom::End(-1))
            .map_err(|_| Crash::new("Seek failed in sequential stream size."))?;
        let pos = file.stream_position().map_err(|_| Crash::new("Stream position failed."))?;
        Ok(pos)
    }

    pub fn query_map(&self, desired_ordinal: u64) -> u64 {
        self.memory_mapped_dump.get(&desired_ordinal).copied().unwrap_or(0)
    }

    pub fn integer_decomposition(&self, dividend: u64) -> Result<Vec<SingleFactor>, Crash> {
        if dividend < 1 {
            return Err(Crash::new("IntegerDecomposition takes arguments >= +1"));
        }
        if dividend == 1 {
            return Ok(vec![SingleFactor {
                factor_base: 0,
                factor_multiplicity: 0,
            }]);
        }

        let log_integral_val = real_integration::trapezi(
            2.0,
            dividend as f64,
            (dividend as f64) * 4.0,
            internal_algos::log_integral_co_chain,
        )?;
        let estimated_ord = std::cmp::max(log_integral_val as usize, 25);

        let mut involved_primes = Vec::with_capacity(estimated_ord);
        for c in 0..estimated_ord {
            involved_primes.push(self[c as u64 + 1]);
        }

        let mut dividendo = dividend;
        let mut factorization: Vec<SingleFactor> = Vec::new();
        let mut i = 0;
        let mut last_division_was_diophantine = false;

        while dividendo > 1 {
            if i >= involved_primes.len() {
                return Err(Crash::new("Involved primes array exhausted during factorization."));
            }
            let divisore = involved_primes[i];
            if divisore == 0 || (divisore as f64).abs() < 1.0e-6 {
                return Err(Crash::new("Invalid divisor encountered."));
            }

            let real_quotient = (dividendo as f64) / (divisore as f64);
            let int_quotient = dividendo / divisore;

            if (real_quotient - (int_quotient as f64)).abs() < 1.0e-12 {
                if !last_division_was_diophantine {
                    factorization.push(SingleFactor {
                        factor_base: divisore,
                        factor_multiplicity: 0,
                    });
                }
                if let Some(last) = factorization.last_mut() {
                    last.factor_multiplicity += 1;
                }
                last_division_was_diophantine = true;
                dividendo = int_quotient;
            } else {
                if last_division_was_diophantine {
                    // completed factor block
                }
                i += 1;
                last_division_was_diophantine = false;
            }
        }

        factorization.push(SingleFactor {
            factor_base: 0,
            factor_multiplicity: 0,
        });
        Ok(factorization)
    }

    pub fn omega_producer(&self, candidate: u64) -> Omega {
        let Ok(fact) = self.integer_decomposition(candidate) else {
            return Omega { small: 0, big: 0 };
        };
        let small = fact.iter().filter(|f| f.factor_base != 0).count() as i32;
        let big = fact.iter().map(|f| f.factor_multiplicity).sum::<u64>() as i32;
        Omega { small, big }
    }

    pub fn liouville_lambda(&self, candidate: u64) -> i32 {
        if candidate < 1 {
            return 0;
        }
        let omega = self.omega_producer(candidate);
        (-1.0_f64).powi(omega.big) as i32
    }

    pub fn moebius_mu(&self, candidate: u64) -> i32 {
        if candidate < 1 {
            return 0;
        }
        let omega = self.omega_producer(candidate);
        let delta_kronecker = if omega.small == omega.big { 1 } else { 0 };
        delta_kronecker * (-1.0_f64).powi(omega.big) as i32
    }

    pub fn get_nearest_integral(&self, candidate_prime_threshold: u64) -> Option<LogIntegralPillarPoint> {
        let size = self.log_integral_pillars.len();
        if size < 2 {
            return self.log_integral_pillars.first().cloned();
        }
        for c in 0..size - 1 {
            if self.log_integral_pillars[c].threshold < candidate_prime_threshold
                && self.log_integral_pillars[c + 1].threshold >= candidate_prime_threshold
            {
                return Some(self.log_integral_pillars[c].clone());
            }
        }
        self.log_integral_pillars.last().cloned()
    }

    fn feed_sequential_dump_path(&mut self) {
        if self.sequential_dump_path.is_none() {
            self.sequential_dump_path = Some(String::from("./primeDefaultFile.txt"));
        }
    }

    fn feed_custom_dump_path(&mut self) {
        if self.random_dump_path.is_none() {
            self.random_dump_path = Some(String::from("./primeCustomFile.txt"));
        }
    }

    fn feed_mesh_specification_path(&mut self) {
        if self.mesh_renewal_path.is_none() {
            self.mesh_renewal_path = Some(String::from("./meshRenewalFile.txt"));
        }
    }

    fn feed_local_integral_path(&mut self) {
        if self.local_integral_path.is_none() {
            self.local_integral_path = Some(String::from("./localIntegralFile.txt"));
        }
    }

    fn feed_global_integral_path(&mut self) {
        if self.global_integral_path.is_none() {
            self.global_integral_path = Some(String::from("./globalIntegralFile.txt"));
        }
    }

    fn distribution_function_from_existing_mesh(&mut self) -> bool {
        let Some(path) = &self.global_integral_path else {
            return false;
        };
        let Ok(file) = File::open(path) else {
            return false;
        };
        let reader = BufReader::new(file);
        let mut line_ordinal = 0;

        for line_res in reader.lines() {
            let Ok(curr_line) = line_res else { continue };
            line_ordinal += 1;
            if line_ordinal > 3 && !curr_line.is_empty() {
                let tokens: Vec<&str> = curr_line.split('\t').collect();
                if tokens.len() >= 2 {
                    if let (Ok(threshold), Ok(log_integral)) =
                        (tokens[0].parse::<u64>(), tokens[1].parse::<u64>())
                    {
                        self.log_integral_pillars.push(LogIntegralPillarPoint {
                            threshold,
                            log_integral,
                        });
                    }
                }
            }
        }
        true
    }
}

impl std::ops::Index<u64> for Primes {
    type Output = u64;

    fn index(&self, desired_ordinal: u64) -> &Self::Output {
        // For convenience in indexing syntax matching `(*this)[c+1]`
        // In idiomatic Rust, mutable bisection filling is handled prior or via interior mutability if needed.
        // Here we return a reference from the map or panic if missing.
        self.memory_mapped_dump
            .get(&desired_ordinal)
            .expect("Ordinal not found in memory map.")
    }
}