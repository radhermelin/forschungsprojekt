#![warn(clippy::all, clippy::pedantic)]

use forschungsprojekt::calculate_p_lambda;
use forschungsprojekt::formatting::format_lambda;
use forschungsprojekt::initializers::{init_free, init_lambda, init_q, init_zero_vec};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let lambda = init_lambda(&args);
    let n = lambda.len();
    #[allow(non_snake_case)]
    let N = n * (n + 1) / 2;

    let mut p_lambda = Vec::new();
    let q = init_q(n);
    let mut p = init_zero_vec(N);
    let free = init_free(&lambda);
    let mut occ = init_zero_vec(n);

    calculate_p_lambda(&q, &mut p, &mut p_lambda, &free, &mut occ);

    let p_res = format_lambda(&p_lambda);

    println!("p_lambda: {p_res:?}");
    println!("{}", p_lambda.len());
}
