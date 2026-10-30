///  \class  Common::MonteCarlo::DeltaOmega
///  \brief  Riemann partition interval element[cite: 11]

pub struct DeltaOmega {
    pub mediana: f64,
    pub eta: f64,
    pub category_frequency: f64,
    pub delta_omega_cardinality: usize,
}

impl DeltaOmega {
    pub fn new(mediana_point: f64, eta_measure: f64) -> Self {
        DeltaOmega {
            mediana: mediana_point,
            eta: eta_measure,
            category_frequency: 0.0,
            delta_omega_cardinality: 0,
        }
    }

    pub fn get_mediana(&self) -> f64 {
        self.mediana
    }

    pub fn belongs_to_delta_omega(&self, candidate: f64) -> bool {
        (candidate - self.mediana).abs() <= self.eta
    }
}