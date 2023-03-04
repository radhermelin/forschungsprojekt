#![allow(non_snake_case)]

pub mod formatting;
pub mod initializers;
pub mod point;

use point::Point;
use std::collections::HashMap;
use std::iter::zip;

pub fn calculate_p_lambda(
    q: &[Point],
    p: &mut Vec<Vec<usize>>,
    p_lambda: &mut Vec<Vec<Vec<usize>>>,
    free: &[Vec<usize>],
    occ: &mut [Vec<usize>],
) {
    let Some(&Point { x: a, y: b }) = q.last() else {
        p_lambda.push(p.clone());
        return;
    };

    let preocc = set_preocc(a, b, occ);

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
        calculate_p_lambda(&q[..(q.len() - 1)], p, p_lambda, free, occ);
    }
}

fn set_preocc(a: usize, b: usize, occ: &[Vec<usize>]) -> usize {
    if a == b {
        0
    } else if b - a == 1 {
        occ[b][a + 1] + occ[b - 1][a]
    } else {
        occ[b][a + 1] + occ[b - 1][a] - 2 * occ[b - 1][a + 1]
    }
}

pub fn calculate_q_omega_i(n: usize, i: usize) -> Vec<Vec<Vec<usize>>> {
    let mut q: HashMap<Vec<bool>, Vec<bool>> = HashMap::new();
    #[allow(non_snake_case)]
    let N: usize = n * (n + 1) / 2;
    let mut p = vec![false; N];
    if i <= n {
        evil_q(1, &mut p, n, N, &mut q, i);
    }
    todo!() // conversion hashmap to vector
}

fn evil_q(
    j: usize,
    p: &mut [bool],
    n: usize,
    N: usize,
    q: &mut HashMap<Vec<bool>, Vec<bool>>,
    i: usize,
) {
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
    v.extend(vec![false; 2 * (n - i) + 1]);
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

fn update(q: &mut HashMap<Vec<bool>, Vec<bool>>, p: &[bool], i: usize, n: usize, N: usize) {
    let r = calc(p, i, n, N);
    if let Some(value) = q.get(&r) {
        if smaller_bit_vec(p, value) {
            q.insert(r, p.to_vec());
        }
    }
}

fn smaller_bit_vec(left: &[bool], right: &[bool]) -> bool {
    for (&l, r) in zip(left, right) {
        if l ^ r {
            return l;
        }
    }
    return false;
}
