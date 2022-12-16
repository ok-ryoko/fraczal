#![cfg(test)]

use rust_decimal::Decimal;

fn round_dp(a: f64, n_places: u32) -> Decimal {
    Decimal::from_f64_retain(a)
        .unwrap()
        .round_dp(n_places)
}

pub(crate) fn dp_eq(a: f64, b: f64, n_places: u32) -> bool {
    round_dp(a, n_places) == round_dp(b, n_places)
}

fn round_sf(a: f64, n_sig_figs: u32) -> Decimal {
    Decimal::from_f64_retain(a)
        .unwrap()
        .round_sf(n_sig_figs)
        .unwrap()
}

pub(crate) fn sf_eq(a: f64, b: f64, n_sig_figs: u32) -> bool {
    round_sf(a, n_sig_figs) == round_sf(b, n_sig_figs)
}
