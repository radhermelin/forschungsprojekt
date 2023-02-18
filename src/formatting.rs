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

pub fn format_p_lambda(p_lambda: &[Vec<Vec<usize>>]) -> Vec<String> {
    p_lambda.iter().map(|x| format_p(x)).collect()
}
