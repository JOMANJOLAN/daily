/// # 三数之和
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    // Solution 1
    // use std::cmp::Ordering;
    // let mut nums = nums;
    // let mut vecs = vec![];
    // nums.sort();
    // let mut i = 0;
    // while i < nums.len() {
    //     if 0 <= nums[i] {
    //         break;
    //     }
    //     let target = -nums[i];
    //     let mut left = i + 1;
    //     let mut right = nums.len() - 1;
    //     while left < right {
    //         match target.cmp(&(nums[left] + nums[right])) {
    //             Ordering::Greater => {
    //                 left += 1;
    //             }
    //             Ordering::Less => {
    //                 right -= 1;
    //             }
    //             Ordering::Equal => {
    //                 vecs.push(vec![nums[i], nums[left], nums[right]]);
    //                 let numl = nums[left];
    //                 while left < right && nums[left] == numl {
    //                     left += 1;
    //                 }
    //                 let numr = nums[right];
    //                 while left < right && nums[right] == numr {
    //                     right -= 1;
    //                 }
    //             }
    //         }
    //     }
    //     let numi = nums[i];
    //     while i < nums.len() && nums[i] == numi {
    //         i += 1;
    //     }
    // }
    // vecs

    // Example
    let mut nums = nums;
    nums.sort();
    let mut result = Vec::new();
    for i in 0..nums.len().saturating_sub(2) {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        let target = -nums[i];
        let mut left = i + 1;
        let mut right = nums.len().saturating_sub(1);
        if nums[i] + nums[left] + nums[left + 1] > 0 {
            break;
        }
        if nums[i] + nums[right] + nums[right - 1] < 0 {
            continue;
        }
        while left < right {
            let sum = nums[left] + nums[right];
            if sum == target {
                result.push(vec![nums[i], nums[left], nums[right]]);
                left += 1;
                while left < right && nums[left] == nums[left - 1] {
                    left += 1;
                }
                right -= 1;
                while left < right && nums[right] == nums[right + 1] {
                    right -= 1;
                }
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }
    result
}

/// # 矩阵置0
pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    // Solution 1
    // let m = matrix.len();
    // let n = matrix[0].len();
    // let mut cols = vec![false; n];
    // for i in 0..m {
    //     let mut flag = false;
    //     for j in 0..n {
    //         if matrix[i][j] == 0 {
    //             flag = true;
    //             cols[j] = true;
    //         }
    //     }
    //     if flag {
    //         matrix[i].fill(0);
    //     }
    // }
    // for i in 0..m {
    //     for j in 0..n {
    //         if cols[j] {
    //             matrix[i][j] = 0;
    //         }
    //     }
    // }

    // Example
    let m = matrix.len();
    let n = matrix[0].len();
    // first row
    let mut fr = false;
    // first column
    let mut fc = false;
    for i in 0..n {
        if matrix[0][i] == 0 {
            fr = true;
            break;
        }
    }
    for i in 0..m {
        if matrix[i][0] == 0 {
            fc = true;
            break;
        }
    }
    for i in 1..m {
        for j in 1..n {
            if matrix[i][j] == 0 {
                matrix[i][0] = 0;
                matrix[0][j] = 0;
            }
        }
    }
    for i in 1..m {
        for j in 1..n {
            if matrix[i][0] == 0 {
                matrix[i].fill(0);
            } else if matrix[0][j] == 0 {
                matrix[i][j] = 0;
            }
        }
    }
    if fr {
        matrix[0].fill(0);
    }
    if fc {
        matrix.iter_mut().for_each(|row| row[0] = 0);
    }
}

/// # 字母异位词分组
/// Solution 1
// pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
//     let mut strs = strs;
//     strs.sort_by_key(|s| s.len());
//     let mut strs = std::collections::VecDeque::from(strs);
//     let mut vecs = vec![];
//     while !strs.is_empty() {
//         let patt = strs.pop_front().unwrap();
//         let mut vec = vec![];
//         let mut i = 0;
//         if !strs.is_empty() {
//             let mut pcnts = [0; 26];
//             for &b in patt.as_bytes() {
//                 pcnts[(b - b'a') as usize] += 1;
//             }
//             while i < strs.len() && strs[i].len() == patt.len() {
//                 let mut cnts = [0; 26];
//                 for &b in strs[i].as_bytes() {
//                     cnts[(b - b'a') as usize] += 1;
//                 }
//                 if pcnts.iter().zip(cnts.iter()).all(|(&a, &b)| a == b) {
//                     vec.push(strs.remove(i).unwrap());
//                 } else {
//                     i += 1;
//                 }
//             }
//         }
//         vec.push(patt);
//         vecs.push(vec);
//     }
//     vecs
// }

/// # 字母异位词分组
/// Solution 2
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    fn mapcnts(s: &str) -> [u16; 26] {
        let bytes = s.as_bytes();
        let mut ret = [0; 26];
        for b in bytes.into_iter().cloned() {
            let offset = b - b'a';
            ret[offset as usize] += 1;
        }
        ret
    }
    use std::collections::HashMap;
    let mut map: HashMap<[u16; 26], Vec<String>> = HashMap::new();
    for str in strs {
        let cnts = mapcnts(&str);
        map.entry(cnts).or_default().push(str);
    }
    map.into_values().collect()
}

/// # 无重复字符的最大子串
pub fn length_of_longest_substring(s: String) -> i32 {
    // Solution 1
    if s.is_empty() {
        return 0;
    }
    let s = s.as_bytes();
    let mut idxs = [false; 256];
    let mut max = 0;
    let mut left = 0;
    let mut right = 0;
    while right < s.len() {
        let b = s[right] as usize;
        if idxs[b] {
            max = max.max(right - left);
            while s[left] != s[right] {
                idxs[s[left] as usize] = false;
                left += 1;
            }
            left += 1;
        }
        idxs[b] = true;
        right += 1;
    }
    max.max(right - left) as i32

    // Example
    // if s.is_empty() {
    //     return 0;
    // }
    // use std::collections::HashMap;
    // let bs = s.as_bytes();
    // let len = bs.len();
    // let mut res = 1;
    // let mut l = 0;
    // let mut r = 0;
    // let mut hm = HashMap::new();
    // hm.insert(bs[0], 0);
    // while l < len && r < len {
    //     while r < len - 1 {
    //         r += 1;
    //         if hm.contains_key(&bs[r]) {
    //             break;
    //         }
    //         hm.insert(bs[r], r);
    //         let distant = r - l + 1;
    //         if distant > res {
    //             res = distant;
    //         }
    //     }

    //     let pos = hm.get(&bs[r]).copied().unwrap();
    //     //l->pos, remove
    //     while l <= pos {
    //         hm.remove(&bs[l]);
    //         l += 1;
    //     }
    //     //insert to r
    //     hm.insert(bs[r], r);
    // }
    // return res as i32;
}

/// # 最长回文子串
pub fn longest_palindrome(s: String) -> String {
    fn is_palindrome(s: &[u8]) -> bool {
        for i in 0..s.len() / 2 {
            if s[i] != s[s.len() - 1 - i] {
                return false;
            }
        }
        true
    }
    let s = s.as_bytes();
    for len in (1..=s.len()).rev() {
        let mut i = 0;
        while let Some(s) = s.get(i..i + len) {
            if is_palindrome(s) {
                return unsafe { String::from_utf8_unchecked(s.to_vec()) };
            }
            i += 1;
        }
    }
    unreachable!()
}

/// # 递增的三元子序列
pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    let n = nums.len();
    if n < 3 {
        return false;
    }
    let mut first = nums[0];
    let mut second = i32::MAX;
    for i in 1..n {
        if nums[i] > second {
            return true;
        }
        if nums[i] > first {
            second = nums[i];
        } else {
            first = nums[i];
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_of_longest_substring_test() {
        length_of_longest_substring("abba".to_string());
    }
}
