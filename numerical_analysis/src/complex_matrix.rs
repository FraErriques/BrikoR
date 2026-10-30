//  \class  Numerics::ComplexMatrix
//  \brief  a Linear Algebra class for Complex numbers
//  \author F. Erriques (Translated to Rust)
//  \since  Thu Dec 27 16:27:37 CET 2001 (Original)

use crate::complex::Complex;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

pub struct ComplexMatrix {
    rows: usize,
    cols: usize,
    m: Vec<Vec<Complex>>,
}

impl ComplexMatrix {
    //   Constructors and Destructors   BEGIN
    
    // fill with zeroes
    pub fn new(rows: usize, cols: usize) -> Self {
        let m = vec![vec![Complex::new(0.0, 0.0); cols]; rows];
        ComplexMatrix { rows, cols, m }
    }

    // validate and load from file
    pub fn from_file(rows: usize, cols: usize, path: &str) -> Result<Self, String> {
        let mut mat = ComplexMatrix::new(rows, cols);
        if !mat.validate(path)? {
            return Err("suggested dimensions (rows, cols) are inconsistent with the matrix contained in the data file".to_string());
        }
        mat.readfile(path).map_err(|e| e.to_string())?;
        Ok(mat)
    }

    // create in RAM and, if desired, dump on a file
    pub fn new_with_init(rows: usize, cols: usize, randomized: bool, path: Option<&str>) -> Result<Self, io::Error> {
        let mut mat = ComplexMatrix::new(rows, cols);
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
impl Clone for ComplexMatrix {
    fn clone(&self) -> Self {
        ComplexMatrix {
            rows: self.rows,
            cols: self.cols,
            m: self.m.clone(),
        }
    }
}

impl ComplexMatrix {
    // algorithms   BEGIN

    pub fn det_with_copy(&self) -> Result<(Complex, ComplexMatrix), String> {
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
                    copy.m[row][col] = copy.m[row][col] + k * copy.m[xrow][col];
                }
            }
        }
        // ######################## Kordell end   ##################################

        let mut det_val = Complex::new(1.0, 0.0); // init to product invariant
        for index in 0..n {
            det_val = det_val * copy.m[index][index]; // det on a triangular matrix
        }
        if n == 0 {
            return Err("\nzero-size matrix is NO NUMBER\n".to_string());
        }
        Ok((det_val, copy))
    }

    pub fn det(&self) -> Result<Complex, String> {
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
                let k = -copy.m[row][xrow] / copy.m[xrow][xrow]; // each time with a new coefficient
                for col in 0..n {
                    copy.m[row][col] = copy.m[row][col] + k * copy.m[xrow][col];
                }
            }
        }
        // ######################## Kordell end   ##################################

        let mut det_val = Complex::new(1.0, 0.0);
        for index in 0..n {
            det_val = det_val * copy.m[index][index];
        }
        if n == 0 {
            return Err("\nzero-size matrix is NO NUMBER\n".to_string());
        }
        Ok(det_val)
    }

    // transpose a matrix
    pub fn transpose(&self) -> ComplexMatrix {
        let mut copy = ComplexMatrix::new(self.cols, self.rows); // create [cols][rows] to transpose [row][cols]
        for row in 0..self.rows {
            for col in 0..self.cols {
                copy.m[col][row] = self.m[row][col];
            }
        }
        copy
    }

    // the scalar product between vectors (isomorphic obviously)
    pub fn scalar(&self, first: &[Complex], second: &[Complex]) -> Complex {
        let mut res = Complex::new(0.0, 0.0);
        for index in 0..first.len() {
            res = res + first[index] * second[index];
        }
        res
    }

    pub fn multiply(&self, second: &ComplexMatrix) -> Result<ComplexMatrix, String> {
        if self.cols != second.rows {
            return Err("incompatible indexes for product".to_string());
        }
        let trasp_second = second.transpose();
        let mut res = ComplexMatrix::new(self.rows, second.cols);
        for rA in 0..self.rows {
            for rB in 0..second.cols {
                res.m[rA][rB] = self.scalar(&self.m[rA], &trasp_second.m[rB]);
            }
        }
        Ok(res)
    }

    pub fn add(&self, second: &ComplexMatrix) -> Result<ComplexMatrix, String> {
        if self.rows != second.rows || self.cols != second.cols {
            return Err("incompatible indexes for sum".to_string());
        }
        let mut res = ComplexMatrix::new(self.rows, self.cols);
        for r_a in 0..self.rows {
            for r_b in 0..self.cols {
                res.m[r_a][r_b] = self.m[r_a][r_b] + second.m[r_a][r_b];
            }
        }
        Ok(res)
    }

    pub fn sub(&self, second: &ComplexMatrix) -> Result<ComplexMatrix, String> {
        if self.rows != second.rows || self.cols != second.cols {
            return Err("incompatible indexes for sum".to_string());
        }
        let mut res = ComplexMatrix::new(self.rows, self.cols);
        for rA in 0..self.rows {
            for rB in 0..self.cols {
                res.m[rA][rB] = self.m[rA][rB] - second.m[rA][rB];
            }
        }
        Ok(res)
    }

    pub fn div(&self, second: &ComplexMatrix) -> Result<ComplexMatrix, String> {
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

    pub fn equals(&self, second: &ComplexMatrix) -> bool {
        const EPSILON: f64 = 1.0e-12;
        if self.rows != second.rows || self.cols != second.cols {
            return false;
        }
        for row in 0..self.rows {
            for col in 0..self.cols {
                if (self.m[row][col] - second.m[row][col]).length() > EPSILON {
                    return false;
                }
            }
        }
        true
    }

    pub fn reductor(&self, i: usize, j: usize, smaller: &mut ComplexMatrix) {
        let mut row_smaller = 0;
        for row_this in 0..self.rows {
            if row_this == i {
                continue;
            }
            let mut col_smaller = 0;
            for col_this in 0..self.cols {
                if col_this == j {
                    continue;
                }
                smaller.m[row_smaller][col_smaller] = self.m[row_this][col_this];
                col_smaller += 1;
            }
            row_smaller += 1;
        }
    }

    pub fn inverse(&self) -> Result<ComplexMatrix, String> {
        if self.rows != self.cols {
            return Err("only square matrices can be inverted (if non singular)".to_string());
        }
        let n = self.rows;
        let mut cof = ComplexMatrix::new(n, n);
        let mut smaller = ComplexMatrix::new(n - 1, n - 1);

        for row in 0..n {
            for col in 0..n {
                self.reductor(row, col, &mut smaller);
                let coeff_pow = (-1.0_f64).powi((row + 1 + col + 1) as i32);
                let coefficient = Complex::new(coeff_pow, 0.0);
                cof.m[row][col] = coefficient * smaller.det()?;
            }
        }

        let trasp_cof = cof.transpose();
        let det_orig = self.det()?;
        let zero_complex = Complex::new(0.0, 0.0);
        if det_orig.equals(&zero_complex) {
            return Err("a square matrix can be inverted only if NON SINGULAR".to_string());
        }
        let mut result_mat = trasp_cof;
        for row in 0..n {
            for col in 0..n {
                result_mat.m[row][col] = result_mat.m[row][col] / det_orig;
            }
        }
        Ok(result_mat)
    }
    // algorithms  END

    // input, output and memory management  BEGIN
    pub fn show(&self, set_precision: usize) {
        let precision = if set_precision > 20 { 20 } else { set_precision };
        println!("\n\n");
        for row in 0..self.rows {
            for col in 0..self.cols {
                print!("\t{:.*} +i*( {:.*})", precision, self.m[row][col].re(), precision, self.m[row][col].im());
                if col == self.cols - 1 {
                    println!();
                }
            }
        }
    }

    pub fn input(&mut self) {
        let stdin = io::stdin();
        let mut reader = stdin.lock();
        for row in 0..self.rows {
            for col in 0..self.cols {
                print!("\n\tEnter Real part of Complex element[{}][{}]\t", row, col);
                io::stdout().flush().unwrap();
                let mut re_text = String::new();
                reader.read_line(&mut re_text).unwrap();
                let re = re_text.trim().parse::<f64>().unwrap_or(0.0);

                print!("\n\tEnter Imaginary part of Complex element[{}][{}]\t", row, col);
                io::stdout().flush().unwrap();
                let mut im_text = String::new();
                reader.read_line(&mut im_text).unwrap();
                let im = im_text.trim().parse::<f64>().unwrap_or(0.0);

                self.m[row][col] = Complex::new(re, im);
            }
        }
    }

    pub fn insert(&mut self, what: Complex, row: usize, col: usize) -> bool {
        if row >= self.rows || col >= self.cols {
            return false;
        }
        self.m[row][col] = what;
        true
    }

    pub fn readfile(&mut self, path: &str) -> Result<(), io::Error> {
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
                    let re = tokens[token_idx].parse::<f64>().unwrap_or(0.0);
                    token_idx += 1;
                    if token_idx < tokens.len() {
                        let i_token = &tokens[token_idx];
                        if i_token != "i" {
                            return Err(io::Error::new(io::ErrorKind::InvalidData, "no valid Complex data-file format"));
                        }
                        token_idx += 1;
                    }
                    let im = if token_idx < tokens.len() {
                        let val = tokens[token_idx].parse::<f64>().unwrap_or(0.0);
                        token_idx += 1;
                        val
                    } else {
                        0.0
                    };
                    self.m[row][col] = Complex::new(re, im);
                }
            }
        }
        Ok(())
    }

    pub fn validate(&self, path: &str) -> Result<bool, String> {
        let file = File::open(path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);

        let mut elements = 0;
        for line in reader.lines() {
            let line = line.map_err(|e| e.to_string())?;
            for token in line.split_whitespace() {
                if token == "i" {
                    elements -= 1; // skip flag of imaginary part
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
            for col in 0..self.cols {
                write!(file, "{} i {}", self.m[row][col].re(), self.m[row][col].im())?;
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
        let rand_reductor = 9.0 / (rand::random::<f64>() + 1.0);
        for row in 0..self.rows {
            for col in 0..self.cols {
                let re = rand::random::<f64>() * rand_reductor;
                let im = rand::random::<f64>() * rand_reductor;
                self.m[row][col] = Complex::new(re, im);
            }
        }
    }
}