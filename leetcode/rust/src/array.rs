/// # 16. 最接近的三数之和
pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let mut ans = nums[0..3].iter().sum::<i32>();
    for i in 0..nums.len() - 2 {
        let mut left = i + 1;
        let mut right = nums.len() - 1;
        while left < right {
            let sum = nums[left] + nums[right] + nums[i];
            if (sum - target).abs() < (ans - target).abs() {
                ans = sum;
            }
            use std::cmp::Ordering::*;
            match sum.cmp(&target) {
                Greater => right -= 1,
                Less => left += 1,
                Equal => return target,
            }
        }
    }
    ans
}

/// # 18. 四数之和
pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    if nums.len() < 4 {
        return vec![];
    }
    let target = target as i64;
    let mut nums = nums;
    nums.sort_unstable();
    use std::collections::HashSet;
    let mut anss = HashSet::new();
    for i in 0..(nums.len() - 3) {
        for j in (i + 1)..(nums.len() - 2) {
            let mut left = j + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let sum = (nums[i] + nums[j]) as i64 + (nums[left] + nums[right]) as i64;
                use std::cmp::Ordering::*;
                match sum.cmp(&target) {
                    Greater => right -= 1,
                    Less => left += 1,
                    Equal => {
                        anss.insert(vec![nums[i], nums[j], nums[left], nums[right]]);
                        left += 1;
                        right -= 1;
                    }
                }
            }
        }
    }
    anss.into_iter().collect()
}

/// # 27. 移除元素
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut cnt = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
            nums[cnt] = nums[i];
            cnt += 1;
        }
    }
    cnt as i32
}

/// # 31. 下一个排列
pub fn next_permutation(nums: &mut Vec<i32>) {
    for i in (0..nums.len() - 1).rev() {
        if nums[i] < nums[i + 1] {
            for j in (i + 1..nums.len()).rev() {
                if nums[i] < nums[j] {
                    nums.swap(i, j);
                    nums[i + 1..].reverse();
                    return;
                }
            }
        }
    }
    nums.reverse();
}

/// # 33. 搜索旋转排序数组
pub fn search_33(nums: Vec<i32>, target: i32) -> i32 {
    // Solution 1: Out of bounds
    // let n = nums.len();
    // let first = nums[0];
    // let last = nums[n - 1];
    // let mut left = 0;
    // let mut right = n - 1;
    // while left <= right {
    //     let i = left + (right - left) / 2;
    //     use std::cmp::Ordering::*;
    //     match nums[i].cmp(&target) {
    //         Equal => return i as i32,
    //         Greater => {
    //             if first <= nums[i] {
    //                 if first <= target {
    //                     return nums[0..i]
    //                         .binary_search(&target)
    //                         .map(|i| i as i32)
    //                         .unwrap_or(-1);
    //                 }
    //                 left = i + 1;
    //             } else {
    //                 right = i - 1;
    //             }
    //         }
    //         Less => {
    //             if nums[i] <= last {
    //                 if target <= last {
    //                     return nums[i..n]
    //                         .binary_search(&target)
    //                         .map(|j| (i + j) as i32)
    //                         .unwrap_or(-1);
    //                 }
    //                 right = i - 1;
    //             } else {
    //                 left = i + 1;
    //             }
    //         }
    //     }
    // }
    // -1

    // Example
    let n = nums.len();
    let mut l = 0;
    let mut r = n - 1;
    while l <= r {
        let mid = l + (r - l) / 2;
        if nums[mid] == target {
            return mid as i32;
        }
        if nums[0] <= nums[mid] {
            if nums[0] <= target && target < nums[mid] {
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        } else {
            if nums[mid] < target && target <= nums[n - 1] {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
    }
    -1
}

/// # 35. 搜索插入位置
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let i = match nums.binary_search(&target) {
        Ok(i) => i,
        Err(i) => i,
    };
    i as i32
}

/// # 39. 组合总和
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    // // Solution 1
    // use std::collections::HashSet;
    // fn combine(candidates: &Vec<i32>, target: i32, ans: &mut HashSet<Vec<i32>>, vec: Vec<i32>) {
    //     for &candidate in candidates {
    //         use std::cmp::Ordering::*;
    //         match target.cmp(&candidate) {
    //             Greater => {
    //                 let mut vec = vec.clone();
    //                 vec.push(candidate);
    //                 combine(candidates, target - candidate, ans, vec);
    //             }
    //             Equal => {
    //                 let mut vec = vec;
    //                 vec.push(candidate);
    //                 vec.sort_unstable();
    //                 ans.insert(vec);
    //                 break;
    //             }
    //             Less => break,
    //         }
    //     }
    // }
    // let mut candidates = candidates;
    // candidates.sort_unstable();
    // let mut ans = HashSet::new();
    // combine(&candidates, target, &mut ans, vec![]);
    // ans.into_iter().collect()

    // Example
    fn dfs(nums: &Vec<i32>, i: usize, target: i32, buf: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if i == nums.len() {
            return;
        }
        if target == 0 {
            ans.push(buf.clone());
            return;
        }
        dfs(nums, i + 1, target, buf, ans);
        if target >= nums[i] {
            buf.push(nums[i]);
            dfs(nums, i, target - nums[i], buf, ans);
            buf.pop();
        }
    }
    let mut ans = vec![];
    let mut buf = vec![];
    dfs(&candidates, 0, target, &mut buf, &mut ans);
    ans
}

/// # 40. 组合总数 II
pub fn combination_sum_ii(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    // Solution 1 Timeout
    // fn dfs(
    //     nums: &Vec<i32>,
    //     i: usize,
    //     target: i32,
    //     buf: &mut Vec<i32>,
    //     ans: &mut HashSet<Vec<i32>>,
    // ) {
    //     if target == 0 && !ans.contains(buf) {
    //         ans.insert(buf.clone());
    //     }
    //     if i == nums.len() {
    //         return;
    //     }
    //     if target >= nums[i] {
    //         buf.push(nums[i]);
    //         dfs(nums, i + 1, target - nums[i], buf, ans);
    //         buf.pop();
    //     } else {
    //         return;
    //     }
    //     dfs(nums, i + 1, target, buf, ans);
    // }
    // let mut candidates = candidates;
    // candidates.sort_unstable(); // [1, 2, 2, 2, 5]
    // use std::collections::HashSet;
    // let mut ans = HashSet::new();
    // let mut buf = vec![];
    // dfs(&candidates, 0, target, &mut buf, &mut ans);
    // ans.into_iter().collect()

    // Example
    fn dfs(
        nums: &Vec<(i32, usize)>,
        i: usize,
        target: i32,
        buf: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            ans.push(buf.clone());
            return;
        }
        if i == nums.len() || target < nums[i].0 {
            return;
        }
        dfs(nums, i + 1, target, buf, ans);
        let n = nums[i].1.min((target / nums[i].0) as usize);
        for j in 1..=n {
            buf.push(nums[i].0);
            dfs(nums, i + 1, target - j as i32 * nums[i].0, buf, ans);
        }
        for _ in 1..=n {
            buf.pop();
        }
    }
    let mut nums: Vec<(i32, usize)> = Vec::new();
    let mut ans: Vec<Vec<i32>> = Vec::new();
    let mut buf: Vec<i32> = Vec::new();
    let mut candidates = candidates;
    candidates.sort_unstable();
    let mut i = 0;
    while i < candidates.len() {
        let num = candidates[i];
        let cnt = candidates[i..]
            .iter()
            .take_while(|&&cdd| cdd == num)
            .count();
        i += cnt;
        nums.push((num, cnt));
    }
    dfs(&nums, 0, target, &mut buf, &mut ans);
    ans
}

/// # 45. 跳跃游戏 II
pub fn jump_ii(nums: Vec<i32>) -> i32 {
    // Solution 1: Timeout
    // fn dfs(nums: &Vec<i32>, i: i32, path: &mut i32, jmp: i32) {
    //     if i as usize >= nums.len() {
    //         return;
    //     }
    //     if i as usize == nums.len() - 1 {
    //         *path = jmp.min(*path);
    //     }
    //     for j in 1..=nums[i as usize] {
    //         dfs(nums, i + j, path, jmp + 1);
    //     }
    // }
    // let mut path = i32::MAX;
    // dfs(&nums, 0, &mut path, 0);
    // path

    // Example
    let n = nums.len() as i32;
    let mut end = 0;
    let mut steps = 0;
    let mut maxpos = 0;
    for i in 0..(n - 1) {
        maxpos = maxpos.max(i + nums[i as usize]);
        if i == end {
            end = maxpos;
            steps += 1;
        }
    }
    steps
}

/// # 47. 全排列 II
pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn dfs(
        nums: &Vec<i32>,
        used: &mut Vec<bool>,
        depth: usize,
        buf: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        if depth == nums.len() {
            ans.push(buf.clone());
            return;
        }
        for i in 0..nums.len() {
            if used[i] || (0 < i && nums[i - 1] == nums[i] && !used[i - 1]) {
                continue;
            }
            buf[depth] = nums[i];
            used[i] = true;
            dfs(nums, used, depth + 1, buf, ans);
            used[i] = false;
        }
    }
    let mut nums = nums;
    nums.sort_unstable();
    let mut used = vec![false; nums.len()];
    let mut buf = vec![0; nums.len()];
    let mut ans = vec![];
    dfs(&nums, &mut used, 0, &mut buf, &mut ans);
    ans
}

/// # 54. 螺旋矩阵
pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut row = matrix.len();
    let mut col = matrix[0].len();
    let mut ans = vec![];
    let mut r;
    let mut c;
    let mut i = 0;
    while 2 <= row && 2 <= col {
        [r, c] = [i; 2];
        for _ in 0..(col - 1) {
            ans.push(matrix[r][c]);
            c += 1;
        }
        for _ in 0..(row - 1) {
            ans.push(matrix[r][c]);
            r += 1;
        }
        for _ in 0..(col - 1) {
            ans.push(matrix[r][c]);
            c -= 1;
        }
        for _ in 0..(row - 1) {
            ans.push(matrix[r][c]);
            r -= 1;
        }
        i += 1;
        row -= 2;
        col -= 2;
    }
    if row != 0 && col != 0 {
        [r, c] = [i; 2];
        if row == 1 {
            for _ in 0..col {
                ans.push(matrix[i][c]);
                c += 1;
            }
        } else {
            for _ in 0..row {
                ans.push(matrix[r][c]);
                r += 1;
            }
        }
    }
    ans
}

/// # 57. 插入区间
pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let mut start = new_interval[0];
    let mut end = new_interval[1];
    let mut inserted = false;
    for interval in intervals {
        let istart = interval[0];
        let iend = interval[1];
        if start < istart {
            if end < istart {
                if !inserted {
                    ans.push(vec![start, end]);
                    inserted = true;
                }
                ans.push(interval);
            } else if end <= iend {
                end = iend;
                ans.push(vec![start, end]);
                inserted = true;
            }
        } else if start <= iend {
            start = istart;
            if end <= iend {
                end = iend;
                ans.push(vec![start, end]);
                inserted = true;
            }
        } else {
            ans.push(interval);
        }
    }
    if !inserted {
        ans.push(vec![start, end]);
    }
    ans
}

/// # 59. 螺旋矩阵 II
pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut ans = vec![vec![0; n]; n];
    let mut i = 1;
    let mut side = n;
    let mut layer = 0;
    let mut r;
    let mut c;
    while 2 <= side {
        [r, c] = [layer; 2];
        for _ in 0..(side - 1) {
            ans[r][c] = i;
            c += 1;
            i += 1;
        }
        for _ in 0..(side - 1) {
            ans[r][c] = i;
            r += 1;
            i += 1;
        }
        for _ in 0..(side - 1) {
            ans[r][c] = i;
            c -= 1;
            i += 1;
        }
        for _ in 0..(side - 1) {
            ans[r][c] = i;
            r -= 1;
            i += 1;
        }
        layer += 1;
        side -= 2;
    }
    if side != 0 {
        [r, c] = [layer; 2];
        ans[r][c] = i;
    }
    ans
}

/// # 63. 不同路径 II
pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    // Solution 1: Timeout
    // fn dfs(grid: &Vec<Vec<i32>>, m: usize, n: usize, ans: &mut i32) {
    //     if 0 < grid[m][n] {
    //         return;
    //     }
    //     if [m, n] == [0; 2] {
    //         *ans += 1;
    //         return;
    //     }
    //     if 0 < m {
    //         dfs(grid, m - 1, n, ans);
    //     }
    //     if 0 < n {
    //         dfs(grid, m, n - 1, ans);
    //     }
    // }
    // let m = obstacle_grid.len() - 1;
    // let n = obstacle_grid[0].len() - 1;
    // let mut ans = 0;
    // dfs(&obstacle_grid, m, n, &mut ans);
    // ans

    // Solution 2
    let m = obstacle_grid.len();
    let n = obstacle_grid[0].len();
    if 0 < obstacle_grid[m - 1][n - 1] {
        return 0;
    }
    let mut dp = vec![vec![0; n]; m];
    dp[m - 1][n - 1] = 1;
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if 0 < obstacle_grid[i][j] {
                continue;
            }
            if i + 1 < m {
                dp[i][j] += dp[i + 1][j];
            }
            if j + 1 < n {
                dp[i][j] += dp[i][j + 1];
            }
        }
    }
    dp[0][0]
}

/// # 64. 最小路径和
pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;
    let m = grid.len();
    let n = grid[0].len();
    for (i, j) in (0..m)
        .rev()
        .map(|i| (0..n).rev().map(move |j| (i, j)))
        .flatten()
        .skip(1)
    {
        grid[i][j] += std::cmp::min(
            grid.get(i + 1).map(|vec| vec[j]).unwrap_or(i32::MAX),
            grid[i].get(j + 1).copied().unwrap_or(i32::MAX),
        );
    }
    grid[0][0]
}

/// # 74. 搜索矩阵
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let i = match matrix.binary_search_by_key(&target, |vec| vec[0]) {
        Ok(i) => i,
        Err(i) => {
            if i == 0 {
                return false;
            } else {
                i - 1
            }
        }
    };
    matrix[i].binary_search(&target).is_ok()
}

/// # 80. 删除有序数组中的重复项 II
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut left = 0;
    let mut i = 0;
    while i < nums.len() {
        let num = nums[i];
        let mut cnt = 0;
        while i < nums.len() {
            if nums[i] == num {
                i += 1;
                cnt += 1;
            } else {
                break;
            }
        }
        if 1 < cnt {
            nums[left] = num;
            left += 1;
            nums[left] = num;
            left += 1;
        } else {
            nums[left] = num;
            left += 1;
        }
    }
    left as i32
}

/// # 81. 搜索旋转排序数组 II
pub fn search_81(nums: Vec<i32>, target: i32) -> bool {
    if nums.is_empty() {
        return false;
    }
    let n = nums.len();
    let mut last = n - 1;
    while last != 0 && nums[0] == nums[last] {
        last -= 1;
    }
    let mut l = 0;
    let mut r = last;
    while l <= r {
        let mid = l + (r - l) / 2;
        if nums[mid] == target {
            return true;
        }
        if nums[0] <= nums[mid] {
            if nums[0] <= target && target < nums[mid] {
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        } else {
            if nums[mid] < target && target <= nums[last] {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
    }
    false
}

/// # 90. 子集 II
pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn dfs(nums: &Vec<i32>, pre: bool, i: usize, buf: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if i == nums.len() {
            ans.push(buf.clone());
            return;
        }
        dfs(nums, false, i + 1, buf, ans);
        if !pre || (0 < i && nums[i - 1] == nums[i]) {
            return;
        }
        buf.push(nums[i]);
        dfs(nums, true, i + 1, buf, ans);
        buf.pop();
    }
    let mut nums = nums;
    nums.sort_unstable();
    let mut buf = vec![];
    let mut ans = vec![];
    dfs(&nums, false, 0, &mut buf, &mut ans);
    ans
}
