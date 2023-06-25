fn islands_in_matrix(matrix: &mut Vec<Vec<u8>>) -> i32 {
    fn clear(d: &mut Vec<Vec<u8>>, r: usize, c: usize) {
        if d[r][c] == 0 {
            return;
        }
        d[r][c] = 0;

        let (m, n) = (d.len(), d[0].len());

        if r > 0 {
            clear(d, r - 1, c);
        }
        if r < m - 1 {
            clear(d, r + 1, c);
        }

        if c > 0 {
            clear(d, r, c - 1);
        }
        if c < n - 1 {
            clear(d, r, c + 1);
        }
    }

    let mut ans = 0;
    for r in 0..matrix.len() {
        for c in 0..matrix[r].len() {
            if matrix[r][c] == 1 {
                ans += 1;
                clear(matrix, r, c);
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_islands_in_matrix() {
        let mut matrix = vec![
            vec![1, 0, 0, 0, 1],
            vec![0, 0, 0, 1, 1],
            vec![0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0],
        ];

        assert_eq!(islands_in_matrix(&mut matrix), 4);
    }
}
