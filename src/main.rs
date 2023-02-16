#![warn(clippy::all, clippy::pedantic)]

fn main() {}

// Initializes p as a zero vector in R^(n(n+1)/2)
#[allow(dead_code)]
fn init_p(n: usize) -> Vec<Vec<usize>> {
    let mut res = Vec::new();
    for i in 0..n {
        res.push(vec![0; i + 1]);
    }
    res
}

// Initializes triangular grid which holds the points for the paths
#[allow(dead_code)]
fn init_free(lambda: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut res = Vec::new();
    for i in 0..lambda.len() {
        res.push(vec![0; i + 1]);
        res[i][i] = lambda[i];
    }
    res
}

// Initializes q as triangular grid of index tuples
#[allow(dead_code)]
fn init_q(n: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for diff in 0..n {
        for i in 0..n - diff {
            res.push((i, i + diff));
        }
    }
    res
}
