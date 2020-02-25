use std::cmp::max;

fn max_value(grid: Vec<Vec<i32>>) -> i32 {
    let mut dp: Vec<Vec<i32>> = vec![];
    let yl = grid.len();
    let xl = grid[0].len();

    for _ in 0..yl {
        let mut tmp = vec![];
        for _ in 0..xl {
            tmp.push(0);
        }
        dp.push(tmp);
    }

    for (i, x) in grid.iter().enumerate() {
        for (j, y) in x.iter().enumerate() {
            if i == 0 && j == 0 {
                dp[i][j] = *y
            } else if i == 0 {
                dp[i][j] += *y + dp[i][j - 1]
            } else if j == 0 {
                dp[i][j] += *y + dp[i - 1][j]
            } else {
                dp[i][j] += *y + max(dp[i - 1][j], dp[i][j - 1])
            }
        }
    }
    dp[yl-1][xl-1]
}

fn main() {
    let grid = vec![
        vec![1, 3, 1],
        vec![1, 5, 1],
        vec![4, 2, 1]
    ];
    assert_eq!(max_value(grid), 12);

    let grid = vec![
        vec![1, 3, 1],
    ];
    assert_eq!(max_value(grid), 5);

    let grid = vec![
        vec![1],
    ];
    assert_eq!(max_value(grid), 1);
}