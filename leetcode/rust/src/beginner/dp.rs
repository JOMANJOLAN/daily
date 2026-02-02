/// # 爬楼梯
pub trait ClimbStairs {
    // Fibonacci
    fn climb_stairs(n: i32) -> i32 {
        // Solution 1
        // match n {
        //     1 => 1,
        //     2 => 2,
        //     n => Self::climb_stairs(n - 1) + Self::climb_stairs(n - 2),
        // }

        // // Example
        (0..n).fold((0, 1), |(a, b), _| (b, a + b)).1
    }
}

/// # 买卖股票的最佳时机
pub fn max_profit(prices: Vec<i32>) -> i32 {
    // Solution 1
    let init = [-prices[0], 0];
    prices.iter().fold(init, |[a, b], &p| {
        let m = a.max(-p);
        let n = b.max(p + a);
        [m, n]
    })[1]
}

/// # 最大子序和
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    // Solution 1
    let init = [0, i32::MIN];
    nums.iter().fold(init, |[sum, max], &num| {
        let sum = num.max(sum + num);
        let max = max.max(sum);
        [sum, max]
    })[1]
}

/// # 打家劫舍
pub fn rob(nums: Vec<i32>) -> i32 {
    // Solution 1
    let init = [0, 0];
    let [yes, no] = nums.iter().fold(init, |[yes, no], &money| {
        let y = no + money;
        let n = no.max(yes);
        [y, n]
    });
    yes.max(no)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn climb_stairs_test() {
        struct A;
        impl ClimbStairs for A {}
        println!("{}", A::climb_stairs(45));
    }
}
