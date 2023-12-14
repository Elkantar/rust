use std::cmp::min;

pub fn edit_distance(source: &str, target: &str) -> usize {
    let len1 = source.len();
    let len2 = target.len();

    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        for j in 0..=len2 {
            if i == 0 {
                dp[i][j] = j;
            } else if j == 0 {
                dp[i][j] = i;
            } else if source.as_bytes()[i - 1] == target.as_bytes()[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + min(
                    dp[i - 1][j],
                    min(
                        dp[i][j - 1],
                        dp[i - 1][j - 1]
                    )
                );
            }
        }
    }

    dp[len1][len2]
}
