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
