//  \class  Numerics::RealMatrix
//  \brief  a Linear Algebra class
//  \author F. Erriques (Translated to Rust)
//  \since  Thu Dec 27 16:27:37 CET 2001 (Original) then Translated to Rust.

use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
//use std::path::Path;
//use common::Crash; // or via your workspace dependency path

pub struct RealMatrix {
    rows: usize,
    cols: usize,
    m: Vec<Vec<f64>>,
}

impl RealMatrix {
    //   Constructors and Destructors   BEGIN
    
    // fill with zeroes
    pub fn new(rows: usize, cols: usize) -> Self {
        let m = vec![vec![0.0; cols]; rows];
        RealMatrix { rows, cols, m }
    }

    // validate and load from file
    pub fn from_file(rows: usize, cols: usize, path: &str) -> Result<Self, String> {
        let mut mat = RealMatrix::new(rows, cols);
        if !mat.validate(path)? {
            return Err("suggested dimensions (rows, cols) are inconsistent with the matrix contained in the data file".to_string());
        }
        mat.readfile(path).map_err(|e| e.to_string())?;
        Ok(mat)
    }

    // create in RAM and, if desired, dump on a file (path == None means "no dump desired")
    pub fn new_with_init(rows: usize, cols: usize, randomized: bool, path: Option<&str>) -> Result<Self, io::Error> {
        let mut mat = RealMatrix::new(rows, cols);
        if randomized {
            mat.generator();
        } else {
            mat.input();
        }
        if let Some(p) = path {
            mat.writefile(p)?;
        }
        Ok(mat)
    }

    // Constructors and Destructors   END
}

// Copy implementation (equivalent to Copy Constructor)
impl Clone for RealMatrix {
    fn clone(&self) -> Self {
        RealMatrix {
            rows: self.rows,
            cols: self.cols,
            m: self.m.clone(),
        }
    }
}

impl RealMatrix {
    // algorithms   BEGIN

    pub fn det_with_copy(&self) -> Result<(f64, RealMatrix), String> {
        if self.rows != self.cols {
            return Err("determinant only on square matrices".to_string());
        }
        let n = self.rows;
        let mut copy = self.clone();

        // ######################## Kordell begin ##################################
        for row in 1..n {
            // skip first row !
            for xrow in 0..row {
                // another loop on the rows
                // the n^ row must be re-normalized n-1 times
                let k = -copy.m[row][xrow] / copy.m[xrow][xrow]; // each time with a new coefficient
                for col in 0..n {
                    copy.m[row][col] += k * copy.m[xrow][col];
                }
            }
        }
        // ######################## Kordell end   ##################################

        let mut det_val = 1.0; // init to product invariant (both for real and complex cases)!
        for index in 0..n {
            det_val *= copy.m[index][index]; // det on a triangular matrix
        }
        if n == 0 {
            return Err("\nzero-size matrix is NO NUMBER\n".to_string());
        }
        Ok((det_val, copy))
    }

    pub fn det(&self) -> Result<f64, String> {
        if self.rows != self.cols {
            return Err("determinant only on square matrices".to_string());
        }
        let n = self.rows;
        let mut copy = self.clone();

        // ######################## Kordell begin ##################################
        for row in 1..n {
            // skip first row !
            for xrow in 0..row {
                // another loop on the rows
                // the n^ row must be re-normalized n-1 times
                let k = -copy.m[row][xrow] / copy.m[xrow][xrow]; // each time with a new coefficient
                for col in 0..n {
                    copy.m[row][col] += k * copy.m[xrow][col];
                }
            }
        }
        // ######################## Kordell end   ##################################

        let mut det_val = 1.0; // init to product invariant (both for real and complex cases)!
        for index in 0..n {
            det_val *= copy.m[index][index]; // det on a triangular matrix
        }
        if n == 0 {
            return Err("\nzero-size matrix is NO NUMBER\n".to_string());
        }
        Ok(det_val)
    }

    // transpose a matrix
    pub fn transpose(&self) -> RealMatrix {
        let mut copy = RealMatrix::new(self.cols, self.rows); // create [cols][rows] to transpose [row][cols]
        for row in 0..self.rows {
            for col in 0..self.cols {
                copy.m[col][row] = self.m[row][col];
            }
        }
        copy
    }

    // the scalar product between vectors (isomorphic obviously)
    pub fn scalar(&self, first: &[f64], second: &[f64]) -> f64 {
        let mut res = 0.0;
        for index in 0..first.len() {
            res += first[index] * second[index];
        }
        res
    }

    pub fn multiply(&self, second: &RealMatrix) -> Result<RealMatrix, String> {
        if self.cols != second.rows {
            return Err("incompatible indexes for product".to_string());
        }
        let trasp_second = second.transpose();
        let mut res = RealMatrix::new(self.rows, second.cols);
        for r_a in 0..self.rows {
            for r_b in 0..second.cols {
                res.m[r_a][r_b] = self.scalar(&self.m[r_a], &trasp_second.m[r_b]);
            }
        }
        Ok(res)
    }

    pub fn add(&self, second: &RealMatrix) -> Result<RealMatrix, String> {
        if self.rows != second.rows || self.cols != second.cols {
            return Err("incompatible indexes for sum".to_string());
        }
        let mut res = RealMatrix::new(self.rows, self.cols);
        for r_a in 0..self.rows {
            for r_b in 0..self.cols {
                res.m[r_a][r_b] = self.m[r_a][r_b] + second.m[r_a][r_b];
            }
        }
        Ok(res)
    }

    pub fn sub(&self, second: &RealMatrix) -> Result<RealMatrix, String> {
        if self.rows != second.rows || self.cols != second.cols {
            return Err("incompatible indexes for sum".to_string());
        }
        let mut res = RealMatrix::new(self.rows, self.cols);
        for r_a in 0..self.rows {
            for r_b in 0..self.cols {
                res.m[r_a][r_b] = self.m[r_a][r_b] - second.m[r_a][r_b];
            }
        }
        Ok(res)
    }

    pub fn div(&self, second: &RealMatrix) -> Result<RealMatrix, String> {
        if second.rows != second.cols {
            return Err("the matrix to be inverted must be square".to_string());
        }
        if self.cols != second.rows {
            return Err("indexes are incompatible for division; must be: [a][n]*[n][n]".to_string());
        }
        let inverse = second.inverse()?;
        let res = self.multiply(&inverse)?;
        Ok(res)
    }

    pub fn equals(&self, second: &RealMatrix) -> bool {
        const EPSILON: f64 = 1.0e-12;
        if self.rows != second.rows || self.cols != second.cols {
            return false;
        }
        for row in 0..self.rows {
            for col in 0..self.cols {
                if (self.m[row][col] - second.m[row][col]).abs() > EPSILON {
                    return false;
                }
            }
        }
        true
    }

    pub fn reductor(&self, i: usize, j: usize, smaller: &mut RealMatrix) {
        let mut row_smaller = 0;
        for row_this in 0..self.rows {
            if row_this == i {
                // "i" is the row to eliminate
                continue; // skip the selected row
            }
            let mut col_smaller = 0;
            for col_this in 0..self.cols {
                if col_this == j {
                    // "j" is the col to eliminate
                    continue; // skip the selected row
                }
                smaller.m[row_smaller][col_smaller] = self.m[row_this][col_this]; // main task
                col_smaller += 1;
            }
            row_smaller += 1;
        }
    }

    pub fn inverse(&self) -> Result<RealMatrix, String> {
        if self.rows != self.cols {
            return Err("only square matrices can be inverted (if non singular)".to_string());
        }
        let n = self.rows;
        let mut cof = RealMatrix::new(n, n); // square !
        let mut smaller = RealMatrix::new(n - 1, n - 1);

        for row in 0..n {
            // build the cofactor matrix
            for col in 0..n {
                // cols==rows ! square !
                self.reductor(row, col, &mut smaller);
                let coefficient = (-1.0_f64).powi((row + 1 + col + 1) as i32);
                // each element of cof matrix is the det of the appropriate minor * -1^(i+j)
                cof.m[row][col] = coefficient * smaller.det()?;
            }
        }

        let trasp_cof = cof.transpose();
        let det_orig = self.det()?; // det of original matrix
        if 1e-18 > det_orig.abs() || det_orig.is_nan() {
            return Err("a square matrix can be inverted only if NON SINGULAR".to_string());
        }
        let mut result_mat = trasp_cof;
        for row in 0..n {
            // normalize the transposed of cofactor
            for col in 0..n {
                // cols==rows ! square !
                result_mat.m[row][col] /= det_orig; // normalize the transposed of cofactor
            }
        }
        Ok(result_mat) // already normalized
    }
    // algorithms  END

    // input, output and memory management  BEGIN
    pub fn show(&self, set_precision: usize) {
        let precision = if set_precision > 20 { 20 } else { set_precision };
        println!("\n\n");
        for row in 0..self.rows {
            for col in 0..self.cols {
                print!("\t{:.*}", precision, self.m[row][col]);
                if col == self.cols - 1 {
                    println!();
                }
            }
        }
    }

    // input each element of the matrix
    pub fn input(&mut self) {
        let stdin = io::stdin();
        let mut reader = stdin.lock();
        for row in 0..self.rows {
            // input the matrix
            for col in 0..self.cols {
                print!("Enter element[{}][{}]\t", row, col);
                io::stdout().flush().unwrap();
                let mut input_text = String::new();
                reader.read_line(&mut input_text).unwrap();
                if let Ok(val) = input_text.trim().parse::<f64>() {
                    self.m[row][col] = val;
                }
            }
        }
    }

    // insert a single element [row][col]
    pub fn insert(&mut self, what: f64, row: usize, col: usize) -> bool {
        if row >= self.rows || col >= self.cols {
            return false;
        }
        self.m[row][col] = what;
        true
    }

    pub fn get_at(&self, row: usize, col: usize) -> Result<f64, String> {
        // read at m[row][col]
        if row >= self.rows || col >= self.cols {
            return Err("the required position is not present in the matrix.".to_string());
        }
        Ok(self.m[row][col])
    }

    pub fn readfile(&mut self, path: &str) -> Result<(), io::Error> {
        // File format is NOT compulsory:
        //      The only constraint is that the number of elements must be rows*cols
        //      Entries can be divided both by spaces, or tabs (one or more of each)
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut tokens = Vec::new();

        for line in reader.lines() {
            let line = line?;
            for token in line.split_whitespace() {
                tokens.push(token.to_string());
            }
        }

        let mut token_idx = 0;
        for row in 0..self.rows {
            for col in 0..self.cols {
                if token_idx < tokens.len() {
                    if let Ok(val) = tokens[token_idx].parse::<f64>() {
                        self.m[row][col] = val;
                    }
                    token_idx += 1;
                }
            }
        }
        Ok(())
    }

    // validate an existing file
    pub fn validate(&self, path: &str) -> Result<bool, String> {
        // File format is NOT compulsory:
        //      The only constraint is that the number of elements must be rows*cols
        //      Entries can be divided both by spaces, or tabs (one or more of each)
        let file = File::open(path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);

        let mut elements = 0;
        for line in reader.lines() {
            let line = line.map_err(|e| e.to_string())?;
            for token in line.split_whitespace() {
                if token == "i" {
                    return Err("a Complex file was passed to a real instance: exit!".to_string());
                }
                elements += 1;
                if elements >= self.rows * self.cols {
                    break;
                }
            }
            if elements >= self.rows * self.cols {
                break;
            }
        }

        if elements != self.rows * self.cols {
            return Ok(false);
        }
        Ok(true)
    }

    pub fn writefile(&self, path: &str) -> Result<(), io::Error> {
        let mut file = File::create(path)?;
        for row in 0..self.rows {
            // fill a matrix with randomized entries
            for col in 0..self.cols {
                write!(file, "{}", self.m[row][col])?;
                if col == self.cols - 1 {
                    writeln!(file)?;
                } else {
                    write!(file, "\t")?;
                }
            }
        }
        Ok(())
    }

    pub fn generator(&mut self) {
        // Simple linear congruential generator or rand equivalent in Rust
        // For demonstration, using pseudo-random values mapped similarly
        let rand_reductor = 9.0 / (rand::random::<f64>() + 1.0); // Simplified generator mapping
        for row in 0..self.rows {
            // fill a matrix with randomized entries
            for col in 0..self.cols {
                self.m[row][col] = rand::random::<f64>() * rand_reductor;
            }
        }
    }
}