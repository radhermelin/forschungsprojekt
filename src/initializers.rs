use crate::Point;

pub fn init_p(n: usize) -> Vec<Vec<usize>> {
    let mut res = Vec::new();
    for i in 0..n {
        res.push(vec![0; i + 1]);
    }
    res
}

pub fn init_q(n: usize) -> Vec<Point> {
    let mut res = Vec::new();
    for diff in (0..n).rev() {
        for i in 0..n - diff {
            res.push(Point::from(i, i + diff));
        }
    }
    res
}

pub fn init_free(lambda: &[usize]) -> Vec<Vec<usize>> {
    let mut res = Vec::new();
    for i in 0..lambda.len() {
        res.push(vec![0; i + 1]);
        for j in 0..res[i].len() {
            res[i][j] = lambda[j..=i].to_vec().iter().sum();
        }
    }
    res
}

pub fn init_occ(n: usize) -> Vec<Vec<usize>> {
    let mut res = Vec::new();
    for i in 0..n {
        res.push(vec![0; i + 1]);
    }
    res
}

pub fn init_lambda(args: &[String]) -> Vec<usize> {
    args[1..]
        .iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}
