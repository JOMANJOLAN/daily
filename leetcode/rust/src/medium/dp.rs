/// # 跳跃游戏
pub fn can_jump(nums: Vec<i32>) -> bool {
    // Solution 1: timeout
    // fn scan(nums: &[i32], index: usize) -> bool {
    //     use std::cmp::Ordering::*;
    //     match index.cmp(&(nums.len() - 1)) {
    //         Greater => false,
    //         Equal => true,
    //         Less => {
    //             for i in 1..=nums[index] as usize {
    //                 if scan(nums, index + i) {
    //                     return true;
    //                 }
    //             }
    //             false
    //         }
    //     }
    // }
    // scan(&nums, 0)

    // Solution 2: very slow
    // let mut dp = vec![vec![false; nums.len()]; nums.len()];
    // for i in 0..nums.len() {
    //     for j in i..=i + (nums[i] as usize) {
    //         if j < nums.len() {
    //             dp[i][j] = true;
    //         } else {
    //             break;
    //         }
    //     }
    // }
    // let target = nums.len() - 1;
    // let mut curr = target;
    // for i in (1..nums.len()).rev() {
    //     if dp[i][target] {
    //         curr = i;
    //     }
    //     if dp[i - 1][curr] && dp[curr][target] {
    //         dp[i - 1][target] = true;
    //     }
    // }
    // dp[0][target]

    // Example
    let mut max = nums[0];
    for (i, d) in nums.iter().enumerate().skip(1).map(|(i, &d)| (i as i32, d)) {
        if i > max {
            return false;
        }
        max = max.max(i + d);
    }
    true
}

/// # 不同路径
pub fn unique_paths(m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let mut dp = vec![vec![0; n]; m];
    dp[0][0] = 1;
    for i in 0..m {
        for j in 0..n {
            if i != 0 {
                dp[i][j] += dp[i - 1][j];
            }
            if j != 0 {
                dp[i][j] += dp[i][j - 1];
            }
        }
    }
    dp[m - 1][n - 1]
}

/// # 零钱兑换
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let amount = amount as usize;
    let mut dp = vec![amount as i32 + 1; amount + 1];
    dp[0] = 0;
    for i in 1..=amount {
        for &coin in &coins {
            let coin = coin as usize;
            if i >= coin {
                dp[i] = dp[i].min(dp[i - coin] + 1);
            }
        }
    }
    if dp[amount] > amount as i32 {
        -1
    } else {
        dp[amount]
    }
}

/// # 最长递增子序列
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut vec = vec![nums[0]];
    for &num in &nums {
        if let Err(i) = vec.binary_search(&num) {
            match vec.get_mut(i) {
                Some(v) => *v = num,
                None => vec.push(num),
            }
        }
    }
    vec.len() as i32
}
