pub struct StringBuilder {
    scratch_size: usize,
    main: String,
    scratch: String,
}

impl StringBuilder {
    /// Constructor mirroring your forecasted size logic
    pub fn new(forecasted_size: usize) -> Self {
        Self {
            scratch_size: forecasted_size,
            main: String::new(),
            scratch: String::with_capacity(forecasted_size),
        }
    }

    /// Append any type that implements std::fmt::Display (strings, numbers, bools, etc.)
    pub fn append<T: std::fmt::Display>(&mut self, value: T) -> &mut Self {
        use std::fmt::Write;
        let _ = write!(self.scratch, "{}", value);

        if self.scratch.len() > self.scratch_size {
            self.main.push_str(&self.scratch);
            self.scratch.clear();
        }
        self
    }

    /// Flushes scratch into main and returns a read-only reference to the full string
    pub fn str(&mut self) -> &str {
        if !self.scratch.is_empty() {
            self.main.push_str(&self.scratch);
            self.scratch.clear();
        }
        &self.main
    }
}