use std::cmp;
pub fn edit_distance(source: &str, target: &str) -> usize {
    let m = source.len(); // 10
    let n = target.len(); // 10
    let mut matrix = vec![vec![0;n+1];m+1]; // [ []10 ]10

    for i in 0..=n {
        matrix[0][i] = i;
    }
    for j in 0..=m {
        matrix[j][0] = j;
    }
    for row in 1..=m {
        for col in 1..=n {
            if source.chars().nth(row - 1) == target.chars().nth(col - 1) {
                matrix[row][col] = matrix[row - 1][col - 1];
            } else {
                matrix[row][col] =
                    1 +
                    cmp::min(
                        matrix[row][col - 1],
                        cmp::min(matrix[row - 1][col], matrix[row - 1][col - 1])
                    );
            }
        }
    }

    matrix[m][n]
}

pub fn expected_variable(receives: &str, expected: &str) -> Option<String> {
    let c = receives
        .chars()
        .filter(|c| c.is_ascii_uppercase())
        .collect::<String>();
    if !receives.contains("_") || c.len() == 0 {
        return None;
    }
    let p = ((expected.len() - edit_distance(&receives.to_lowercase(), &expected.to_lowercase())) * 100) / expected.len();
    if p <= 50 {
        return None;
    }
    Some(format!("%{}", p))
}
