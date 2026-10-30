///  \brief  Real Riemann Integration Utilities (Rectangles and Trapezia)
///  \author F. Erriques (Translated to Rust)

use common::common_data::Crash;

pub type FunctionalForm = fn(f64) -> f64;

pub fn rettangoli(
    inf: f64,
    sup: f64,
    mut card_partiz: f64,
    functional_form: FunctionalForm,
    _called_from_trapezi: bool,
) -> Result<f64, Crash> {
    if (sup - inf).abs() < 1.0e-18 {
        return Ok(0.0); // zero measure on such a small Dx.
    }
    if card_partiz < 1.0 {
        card_partiz = 1.0;
    }
    let dx = (sup - inf) / card_partiz;
    if dx <= 0.0 {
        return Err(Crash::new("wrong extrema: dx has to be positive."));
    }
    let mut res = 0.0;
    let mut step_ordinal: u64 = 0;
    let mut x = inf;
    while x < sup {
        step_ordinal += 1;
        if (step_ordinal as f64) > card_partiz {
            break;
        }
        res += functional_form(x) * dx;
        x += dx;
    }
    Ok(res)
}

pub fn correttivo_al_contorno(
    inf: f64,
    sup: f64,
    card_partiz: f64,
    functional_form: FunctionalForm,
) -> f64 {
    let dx = (sup - inf) / card_partiz;
    (1.0 / 2.0) * dx * (functional_form(sup) - functional_form(inf))
}

pub fn trapezi(
    inf: f64,
    sup: f64,
    card_partiz: f64,
    functional_form: FunctionalForm,
) -> Result<f64, Crash> {
    let mut res = rettangoli(inf, sup, card_partiz, functional_form, true)?;
    res += correttivo_al_contorno(inf, sup, card_partiz, functional_form);
    Ok(res)
}