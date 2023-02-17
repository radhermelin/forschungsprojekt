#![warn(clippy::all, clippy::pedantic)]

use std::env;

#[allow(dead_code)]
static mut ITERATIONS: i32 = 0;

fn main() {
    let lambda: Vec<String> = env::args().collect();
    let lambda: Vec<usize> = lambda[1..]
        .iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let n = lambda.len();

    #[allow(non_snake_case)]
    let N = n * (n + 1) / 2;

    let mut p_lambda = Vec::new();
    let q = init_q(n);
    let p = init_p(N);
    let free = init_free(&lambda);
    let occ = init_occ(n);

    // println!("p_lambda: {p_lambda:?}");
    // println!("q:        {q:?}");
    // println!("p:        {p:?}");
    // println!("free:     {free:?}");
    // println!("occ:      {occ:?}");

    evil(q, p, &mut p_lambda, &free, occ);

    let p_res: Vec<String> = p_lambda.iter().map(|x| format_p(x)).collect();

    println!("p_lambda: {p_res:?}");
    println!("{}", p_lambda.len());
}

fn format_p(p: &[Vec<usize>]) -> String {
    let mut res = String::new();
    for (row_index, row) in p.iter().enumerate() {
        for (col_index, item) in row.iter().enumerate() {
            if *item != 0 && !res.is_empty() {
                let tmp = format!(" + {item}e_{}{}", col_index + 1, row_index + 1);
                res.push_str(&tmp);
            } else if *item != 0 {
                let tmp = format!("{item}e_{}{}", col_index + 1, row_index + 1);
                res.push_str(&tmp);
            }
        }
    }
    if res.is_empty() {
        return String::from("0");
    }
    res
}

fn init_p(n: usize) -> Vec<Vec<usize>> {
    let mut res = Vec::new();
    for i in 0..n {
        res.push(vec![0; i + 1]);
    }
    res
}

fn init_free(lambda: &Vec<usize>) -> Vec<Vec<usize>> {
    let mut res = Vec::new();
    for i in 0..lambda.len() {
        res.push(vec![0; i + 1]);
        for j in 0..res[i].len() {
            res[i][j] = lambda[j..=i].to_vec().iter().sum();
        }
    }
    res
}

fn init_occ(n: usize) -> Vec<Vec<usize>> {
    let mut res = Vec::new();
    for i in 0..n {
        res.push(vec![0; i + 1]);
    }
    res
}

fn init_q(n: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for diff in (0..n).rev() {
        for i in 0..n - diff {
            res.push((i, i + diff));
        }
    }
    res
}

fn evil(
    mut q: Vec<(usize, usize)>,
    mut p: Vec<Vec<usize>>,
    p_lambda: &mut Vec<Vec<Vec<usize>>>,
    free: &[Vec<usize>],
    mut occ: Vec<Vec<usize>>,
) {
    if q.is_empty() {
        p_lambda.push(p);
        return;
    }
    let (a, b) = q.pop().unwrap();
    let preocc = if a == b {
        0
    } else if b - a == 1 {
        // unsafe {
        //     ITERATIONS += 1;
        //     println!("{ITERATIONS}");
        // }
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
        evil(q.clone(), p.clone(), p_lambda, free, occ.clone());
    }
}
