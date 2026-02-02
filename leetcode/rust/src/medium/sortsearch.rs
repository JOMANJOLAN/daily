/// # 颜色分类
pub fn sort_colors(nums: &mut Vec<i32>) {
    // Solution 1
    // let mut cnts = [0; 3];
    // for &num in nums.iter() {
    //     cnts[num as usize] += 1;
    // }
    // nums[0..cnts[0]].fill(0);
    // nums[cnts[0]..cnts[0] + cnts[1]].fill(1);
    // nums[cnts[0] + cnts[1]..cnts[0] + cnts[1] + cnts[2]].fill(2);

    // Example
    let mut p0 = 0;
    let mut p1 = 0;
    for i in 0..nums.len() {
        let x = nums[i];
        nums[i] = 2;
        if x <= 1 {
            nums[p1] = 1;
            p1 += 1;
        }
        if x == 0 {
            nums[p0] = 0;
            p0 += 1;
        }
    }
}

/// # 前 K 个高频元素
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();
    for &num in &nums {
        *map.entry(num).or_insert(0) += 1;
    }
    let mut vec = map.into_iter().collect::<Vec<_>>();
    vec.sort_by(|(_, cnt1), (_, cnt2)| cnt2.cmp(cnt1));
    vec[0..k as usize].iter().map(|(num, _)| *num).collect()
}

/// # 数组中的第K个最大元素
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort_by(|m, n| n.cmp(m));
    nums[k as usize - 1]
}

/// # 寻找峰值
pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    // Solution 1
    // for i in 0..nums.len() {
    //     if (i == 0 || nums[i - 1] < nums[i]) && (i == nums.len() - 1 || nums[i + 1] < nums[i]) {
    //         return i as i32;
    //     }
    // }
    // unreachable!()

    // Example
    let mut ans = 0usize;
    for i in 0..nums.len() {
        if nums[i] > nums[ans] {
            ans = i;
        }
    }
    ans as i32
}

/// # 在排序数组中查找元素的第一个和最后一个位置
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Solution 1
    // match nums.binary_search(&target) {
    //     Ok(i) => {
    //         let [mut left, mut right] = [i; 2];
    //         while left != 0 && nums[left - 1] == target {
    //             left -= 1;
    //         }
    //         while right != nums.len() - 1 && nums[right + 1] == target {
    //             right += 1;
    //         }
    //         vec![left as i32, right as i32]
    //     }
    //     Err(_) => vec![-1_i32; 2],
    // }

    // Elegant Solution
    match nums.binary_search(&target) {
        Ok(i) => {
            let left = i - nums[..i]
                .iter()
                .rev()
                .take_while(|&&num| num == target)
                .count();
            let right = i + nums[i + 1..]
                .iter()
                .take_while(|&&num| num == target)
                .skip(1)
                .count();
            vec![left as i32, right as i32]
        }
        Err(_) => vec![-1_i32; 2],
    }
}

/// # 合并区间
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    intervals.sort_by_key(|bound| bound[0]);
    let mut intervals = intervals.into_iter();
    let mut vecs = vec![];
    let mut curr = intervals.next().unwrap();
    while let Some(mut temp) = intervals.next() {
        if curr[1] < temp[0] {
            std::mem::swap(&mut curr, &mut temp);
            vecs.push(temp);
        } else {
            curr[1] = curr[1].max(temp[1]);
        }
    }
    vecs.push(curr);
    vecs
}

/// # 搜索旋转排序数组
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    // Solution 1
    // let pivot = nums[0];
    // let mut left = 0;
    // let mut right = nums.len();
    // loop {
    //     let i = left + (right - left) / 2;
    //     if pivot <= nums[i] {
    //         if i + 1 >= nums.len() || nums[i] > nums[i + 1] {
    //             return if pivot <= target {
    //                 nums[..i + 1].binary_search(&target)
    //             } else {
    //                 nums[i + 1..].binary_search(&target).map(|idx| idx + i + 1)
    //             }
    //             .map(|idx| idx as i32)
    //             .unwrap_or(-1);
    //         }
    //         left = i + 1;
    //     } else {
    //         right = i - 1;
    //     }
    // }

    // Example
    let n = nums.len();
    let mut l = 0;
    let mut r = n;
    while l < r {
        let m = l + r >> 1;
        if nums[m] >= nums[0] {
            l = m + 1;
        } else {
            r = m;
        }
    }
    let k = l % n;
    l = 0;
    r = n;
    while l < r {
        let m = l + r >> 1;
        if nums[(m + k) % n] < target {
            l = m + 1;
        } else {
            r = m;
        }
    }
    let ans = (l + k) % n;
    if nums[ans] == target { ans as i32 } else { -1 }
}

/// # 搜索二维矩阵 II
pub fn search_matrix_ii(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut len = n;
    for i in 0..m {
        for j in 0..len {
            if matrix[i][j] == target {
                return true;
            } else if matrix[i][j] > target {
                len = j;
            }
        }
        if len == 0 {
            break;
        }
    }
    false
}
