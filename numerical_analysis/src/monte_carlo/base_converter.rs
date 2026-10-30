///  \class  Common::MonteCarlo::BaseConverter
///  \brief  a base conversion utility class[cite: 5, 6]
///  \author F. Erriques (Translated to Rust)

use std::collections::HashMap;

pub struct BaseConverter {
    base64_figures: String,
    base64_map: HashMap<char, usize>,
}

impl BaseConverter {
    pub fn new(want_show_dbg: bool) -> Self {
        // ASCII codes of involved symbols: [48,57], [65,90], [97,126][cite: 5]
        let mut symbols = String::with_capacity(66);
        for c in 48..=57 {
            symbols.push(c as u8 as char);
        }
        for c in 65..=90 {
            symbols.push(c as u8 as char);
        }
        for c in 97..=126 {
            symbols.push(c as u8 as char);
        }

        if want_show_dbg {
            println!("{}", symbols);
        }

        let mut map = HashMap::new();
        for (c, ch) in symbols.chars().enumerate() {
            map.insert(ch, c);
        }

        if want_show_dbg {
            for (k, v) in &map {
                println!("{}  {}", k, v);
            }
        }

        BaseConverter {
            base64_figures: symbols,
            base64_map: map,
        }
    }

    pub fn base_other_converter_to(&self, decimal_integer: u64, destination_base: u16) -> String {
        let base_f = destination_base as f64;
        let mut sb = String::from("b");
        sb.push_str(&destination_base.to_string());
        sb.push('_');

        let mut abacus_left_most_position = (decimal_integer as f64).ln() / base_f.ln();
        if abacus_left_most_position.is_nan() || abacus_left_most_position.is_infinite() {
            abacus_left_most_position = 0.0;
        }
        let mut dividendo = decimal_integer as f64;
        let mut pos = abacus_left_most_position.floor() as i32;

        while pos >= 0 {
            let denom = base_f.powi(pos);
            let quotient = dividendo / denom;
            let q_int = quotient.floor() as usize;
            let resto = dividendo - (q_int as f64) * denom;

            if let Some(ch) = self.base64_figures.chars().nth(q_int) {
                sb.push(ch);
            }
            dividendo = resto;
            pos -= 1;
        }
        sb
    }

    pub fn base10_converter_to(&self, base_other_par: &str, origin_base: u16) -> u64 {
        let clean_version = if let Some(idx) = base_other_par.find('_') {
            &base_other_par[idx + 1..]
        } else {
            base_other_par
        };

        let mut decimal_quantity: u64 = 0;
        let len = clean_version.chars().count();
        let mut abacus_left_most_position = (len as i32) - 1;

        for ch in clean_version.chars() {
            if abacus_left_most_position < 0 {
                break;
            }
            let figure_val = *self.base64_map.get(&ch).unwrap_or(&0) as u64;
            let position_qty = (origin_base as f64).powi(abacus_left_most_position) as u64;
            decimal_quantity += figure_val * position_qty;
            abacus_left_most_position -= 1;
        }
        decimal_quantity
    }
}