#![allow(non_snake_case)]

pub mod formatting;
pub mod initializers;
pub mod point;

use point::Point;

pub fn calculate_p_lambda(
    mut q: Vec<Point>,
    mut p: Vec<Vec<usize>>,
    p_lambda: &mut Vec<Vec<Vec<usize>>>,
    free: &[Vec<usize>],
    mut occ: Vec<Vec<usize>>,
) {
    if q.is_empty() {
        p_lambda.push(p);
        return;
    }
    let Point { x: a, y: b } = q.pop().unwrap();
    let preocc = if a == b {
        0
    } else if b - a == 1 {
        occ[b][a + 1] + occ[b - 1][a]
    } else {
        occ[b][a + 1] + occ[b - 1][a] - 2 * occ[b - 1][a + 1]
    };
    if free[b][a] < preocc {
        return;
    }
    let avail = free[b][a] - preocc;
    for i in 0..=avail {
        occ[b][a] = preocc + i;
        if b - a > 1 {
            occ[b][a] += occ[b - 1][a + 1];
        }
        p[b][a] = i;
        calculate_p_lambda(q.clone(), p.clone(), p_lambda, free, occ.clone());
    }
}

pub fn calculate_q_omega_i(n: usize, i: usize) -> Vec<Vec<Vec<usize>>> {
    let mut q: Vec<Vec<Vec<usize>>> = Vec::new();
    #[allow(non_snake_case)]
    let N: usize = n * (n + 1) / 2;
    let mut p = vec![false; N];
    if i <= n {
        evil_q(1, &mut p, n, N, &mut q, i);
    }
    q
}

fn evil_q(j: usize, p: &mut [bool], n: usize, N: usize, q: &mut [Vec<Vec<usize>>], i: usize) {
    if j > n {
        update(q, p, i, n, N);
        return;
    }
    evil_q(j + 1, p, n, N, q, i);
    p[j] = true;
    if !calc(p, i, n, N).iter().any(|&x| x) {
        evil_q(j + 1, p, n, N, q, i);
    }
    p[j] = false;
}

fn calc(p: &[bool], i: usize, n: usize, N: usize) -> Vec<bool> {
    let mut v = vec![true; 2 * i - 1];
    v.append(&mut vec![false; 2 * (n - i) + 1]);
    let mut e = 1;
    let mut s = 1;
    for _ in 1..N {
        if p[e] {
            if !v[e] || v[e + 1] {
                return Vec::new();
            }
            v[e] = false;
            v[e + 1] = true;
        }
        e += 1;
        if e > 2 * (s - 1) {
            s += 1;
            e = s;
        }
    }
    v
}

fn update(_q: &mut [Vec<Vec<usize>>], _p: &[bool], _i: usize, _n: usize, _N: usize) {
    todo!();
}
