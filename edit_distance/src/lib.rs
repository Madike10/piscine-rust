pub fn edit_distance(source: &str, target: &str) -> usize {
    let source = source.chars().collect::<Vec<char>>();
    let target = target.chars().collect::<Vec<char>>();
    let mut dp = vec![vec![0; target.len() + 1]; source.len() + 1];

    for i in 0..source.len() + 1 {
        dp[i][0] = i;
    }

    for j in 0..target.len() + 1 {
        dp[0][j] = j;
    }
    for i in 1..source.len() + 1 {
        for j in 1..target.len() + 1 {
            if source[i - 1] == target[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j - 1].min(dp[i - 1][j].min(dp[i][j - 1]));
            }
        }
    }

    // Return the calculated edit distance
    dp[source.len()][target.len()]
}


// And its output:


// $ cargo run
// It's necessary to make 2 change(s) to alignment, to get assignment
// $
