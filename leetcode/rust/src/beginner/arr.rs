/// # 删除排序数组中的重复项
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut i = 0;
    let mut left = 0;
    let mut right = 0;
    while right < nums.len() {
        while right < nums.len() && nums[left] == nums[right] {
            right += 1;
        }
        nums[i] = nums[left];
        i += 1;
        left = right;
    }
    nums.truncate(i);
    nums.len() as i32
}

/// # 买卖股票的最佳时机 II
pub fn max_profit_ii(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    let mut left = 0;
    let mut right = 1;
    while right < prices.len() {
        if prices[right] > prices[left] {
            profit += prices[right] - prices[left];
        }
        left += 1;
        right += 1;
    }
    profit
}

/// # 旋转数组
pub fn rotate_arr(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    // nums.rotate_right(k as usize % len);
    let k = k as usize % len;
    let gcd = {
        let mut a = len;
        let mut b = k;
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    };
    let (i, j) = (gcd, len / gcd);
    for mut i in 0..i {
        let mut buf = nums[i];
        for _ in 0..j {
            i += k;
            i %= len;
            std::mem::swap(&mut buf, &mut nums[i]);
        }
    }
}

/// # 存在重复元素
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    // Solution 1
    // let mut set = std::collections::HashSet::new();
    // for &n in nums.iter() {
    //     if !set.insert(n) {
    //         return true;
    //     }
    // }
    // false

    // Solution 2
    let mut nums = nums;
    nums.sort();
    let mut left = 0;
    let mut right = 1;
    while right < nums.len() {
        if nums[left] == nums[right] {
            return true;
        }
        left += 1;
        right += 1;
    }
    false
}

/// # 只出现一次的数字
pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |i, &n| i ^ n)
}

/// # \* 两个数组的交集 II
pub fn intersect_ii(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    // Solution 1
    // use std::cmp::Ordering;

    // let (mut nums1, mut nums2) = (nums1, nums2);
    // nums1.sort();
    // nums2.sort();

    // let mut vec = Vec::new();
    // let (mut i1, mut i2) = (0, 0);
    // while i1 < nums1.len() && i2 < nums2.len() {
    //     let num1 = nums1[i1];
    //     let num2 = nums2[i2];
    //     match num1.cmp(&num2) {
    //         Ordering::Greater => {
    //             i2 += nums2.iter().take_while(|n| **n < num1).count();
    //         }
    //         Ordering::Less => {
    //             i1 += nums1.iter().take_while(|n| **n < num2).count();
    //         }
    //         Ordering::Equal => {
    //             let same = nums1[i1];
    //             let count1 = nums1[i1..].iter().take_while(|n| **n == same).count();
    //             let count2 = nums2[i2..].iter().take_while(|n| **n == same).count();
    //             let count = count1.min(count2);
    //             vec.resize(vec.len() + count, same);
    //             i1 += count1;
    //             i2 += count2;
    //         }
    //     }
    // }
    // vec

    // Solution 2
    let mut map = std::collections::HashMap::new();
    for &n in nums1.iter() {
        *map.entry(n).or_insert(0) += 1;
    }
    let mut vec = Vec::new();
    for &n in nums2.iter() {
        if let Some(v @ 1..) = map.get_mut(&n) {
            *v -= 1;
            vec.push(n);
        }
    }
    vec
}

/// # 加一
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    // Solution 1
    let mut digits = digits;
    let mut flag = 1;
    for i in (0..digits.len()).rev() {
        digits[i] += flag;
        flag = digits[i] / 10;
        digits[i] %= 10;
        if flag == 0 {
            break;
        }
    }
    if flag != 0 {
        digits.insert(0, flag);
    }
    digits
}

/// # 移动零
pub fn move_zeroes(nums: &mut Vec<i32>) {
    // Solution 1
    let mut left = 0;
    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums.swap(left, i);
            left += 1;
        }
    }
}

/// # 两数之和
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let map = nums
        .iter()
        .enumerate()
        .map(|(i, &n)| (n, i))
        .collect::<std::collections::HashMap<_, _>>();
    for (i, &n) in nums.iter().enumerate() {
        let m = target - n;
        if let Some(&j) = map.get(&m)
            && i != j
        {
            return vec![i as i32, j as i32];
        }
    }
    unreachable!()
}

/// # 有效的数独
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    // Solution 1
    let mut flags = [false; 10];
    for r in 0..9 {
        for c in 0..9 {
            if let ch @ '0'..='9' = board[r][c] {
                let i = ch as usize - '0' as usize;
                if flags[i] {
                    return false;
                }
                flags[i] = true
            }
        }
        flags.fill(false);
    }

    for c in 0..9 {
        for r in 0..9 {
            if let ch @ '0'..='9' = board[r][c] {
                let i = ch as usize - '0' as usize;
                if flags[i] {
                    return false;
                }
                flags[i] = true
            }
        }
        flags.fill(false);
    }

    let origins = [
        [0, 0],
        [0, 3],
        [0, 6],
        [3, 0],
        [3, 3],
        [3, 6],
        [6, 0],
        [6, 3],
        [6, 6],
    ];

    for o in origins {
        for r in 0..3 {
            let r = o[0] + r;
            for c in 0..3 {
                let c = o[1] + c;
                if let ch @ '0'..='9' = board[r][c] {
                    let i = ch as usize - '0' as usize;
                    if flags[i] {
                        return false;
                    }
                    flags[i] = true
                }
            }
        }
        flags.fill(false);
    }

    true
}

/// # 旋转图像
pub fn rotate_img(matrix: &mut Vec<Vec<i32>>) {
    for i in 0..(matrix.len() / 2) {
        for j in 0..(matrix.len() - i * 2 - 1) {
            let a = [i, i + j];
            let b = [i + j, matrix.len() - 1 - i];
            let c = [matrix.len() - 1 - i, matrix.len() - 1 - i - j];
            let d = [matrix.len() - 1 - i - j, i];

            let mut buf = matrix[d[0]][d[1]];
            for [x, y] in [a, b, c, d] {
                std::mem::swap(&mut buf, &mut matrix[x][y]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_profit_ii_test() {
        max_profit_ii(vec![7, 1, 5, 3, 6, 4]);
    }

    #[test]
    fn intersect_ii_test() {
        intersect_ii(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
    }
}
