///  \author  F. Erriques (Translated to Rust)
///  \since   August 2001

pub struct Integrate {
    pub functional_form: fn(f64) -> f64,
    pub result: f64,
    left: f64,
    right: f64,
    parts: u64,
    left_infinity: f64,
}

impl Integrate {
    pub fn reset_parameters(&mut self, left_p: f64, right_p: f64, parts_p: u64) {
        self.left = left_p;
        self.right = right_p;
        self.parts = parts_p;
    }

    pub fn equi_trapezium_default(&self) -> f64 {
        self.equi_trapezium(self.left, self.right, self.parts)
    }

    pub fn equi_log_default(&self) -> f64 {
        self.equi_log(self.left, self.right, self.parts)
    }

    pub fn new(
        initializer: fn(f64) -> f64,
        left_p: f64,
        right_p: f64,
        parts_p: u64,
        left_infinity_p: f64,
    ) -> Self {
        let mut obj = Integrate {
            functional_form: initializer,
            result: 0.0,
            left: 0.0,
            right: 0.0,
            parts: 0,
            left_infinity: left_infinity_p,
        };
        obj.reset_parameters(left_p, right_p, parts_p);
        obj
    }

    pub fn new_infinite(initializer: fn(f64) -> f64, left_infinity_p: f64) -> Self {
        Integrate {
            functional_form: initializer,
            result: 0.0,
            left: 0.0,
            right: 0.0,
            parts: 0,
            left_infinity: left_infinity_p,
        }
    }

    pub fn equi_trapezium(&self, a: f64, b: f64, n: u64) -> f64 {
        let delta = (b - a) / (n as f64);
        let mut res = 0.0;
        let mut x = a + delta;

        while x < b {
            res += (self.functional_form)(x);
            x += delta;
        }
        res *= delta;
        res += ((self.functional_form)(a) + (self.functional_form)(b)) * 0.5 * delta;
        res
    }

    pub fn percentile(&self, measure: f64) -> f64 {
        let delta = 1e-3;
        let mut res = 0.0;
        let mut x = self.left_infinity;

        while res <= measure {
            res += 0.5
                * delta
                * ((self.functional_form)(x + delta) + (self.functional_form)(x));
            x += delta;
        }
        x
    }

    pub fn equi_log(&self, a: f64, b: f64, n: u64) -> f64 {
        let delta = (b - a) / (n as f64);
        let mut res = 0.0;
        let mut x = a;

        while x < b {
            let f_x_delta = (self.functional_form)(x + delta);
            let f_x = (self.functional_form)(x);
            res += delta * (f_x_delta - f_x) / (f_x_delta.ln() - f_x.ln());
            x += delta;
        }
        res
    }

    pub fn profile_default(&mut self, ist_par: fn(&Integrate, f64, f64, u64) -> f64) -> f64 {
        self.profile(ist_par, self.left, self.right, self.parts)
    }

    pub fn profile(
        &mut self,
        ist_par: fn(&Integrate, f64, f64, u64) -> f64,
        a: f64,
        b: f64,
        n: u64,
    ) -> f64 {
        let start = std::time::Instant::now();
        self.result = ist_par(self, a, b, n);
        let duration = start.elapsed();
        duration.as_secs_f64()
    }
}