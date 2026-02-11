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

/// # 120. 三角形最小路径和
pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    // Solution 1: Timeout
    // fn dfs(triangle: &Vec<Vec<i32>>, depth: usize, i: usize, sum: i32, ans: &mut i32) {
    //     if depth == triangle.len() {
    //         *ans = sum.min(*ans);
    //         return;
    //     }
    //     dfs(triangle, depth + 1, i, sum + triangle[depth][i], ans);
    //     dfs(triangle, depth + 1, i + 1, sum + triangle[depth][i], ans);
    // }
    // let mut ans = i32::MAX;
    // dfs(&triangle, 0, 0, 0, &mut ans);
    // ans

    let n = triangle.len();
    let mut dp = Vec::with_capacity(n);
    for i in 0..n {
        dp.push(vec![0; i + 1]);
    }
    dp[0][0] = triangle[0][0];
    for i in 1..n {
        dp[i][0] = dp[i - 1][0] + triangle[i][0];
        for j in 1..i {
            dp[i][j] = std::cmp::min(dp[i - 1][j - 1], dp[i - 1][j]) + triangle[i][j];
        }
        dp[i][i] = dp[i - 1][i - 1] + triangle[i][i];
    }
    *dp[n - 1].iter().min().unwrap()
}

/// # 128. 最长连续序列
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    // Solution 1: Too slow
    // use std::collections::HashMap;
    // let mut map = nums
    //     .iter()
    //     .copied()
    //     .map(|num| (num, true))
    //     .collect::<HashMap<_, _>>();
    // let mut maxcnt = 0;
    // for &num in &nums {
    //     use std::collections::hash_map::Entry::*;
    //     if let Occupied(mut o) = map.entry(num)
    //         && *o.get()
    //     {
    //         *o.get_mut() = false;
    //     }
    //     let mut cnt = 1;
    //     let mut value = num + 1;
    //     while let Occupied(mut o) = map.entry(value)
    //         && *o.get()
    //     {
    //         *o.get_mut() = false;
    //         value += 1;
    //         cnt += 1;
    //     }
    //     let mut value = num - 1;
    //     while let Occupied(mut o) = map.entry(value)
    //         && *o.get()
    //     {
    //         *o.get_mut() = false;
    //         value -= 1;
    //         cnt += 1;
    //     }
    //     maxcnt = maxcnt.max(cnt);
    // }
    // maxcnt

    // Solution 2
    let mut set = nums
        .iter()
        .copied()
        .collect::<std::collections::HashSet<_>>();
    let mut maxcnt = 0;
    for &num in &nums {
        if set.remove(&num) {
            let mut cnt = 1;
            let mut value = num + 1;
            while set.remove(&value) {
                cnt += 1;
                value += 1;
            }
            let mut value = num - 1;
            while set.remove(&value) {
                cnt += 1;
                value -= 1;
            }
            maxcnt = maxcnt.max(cnt);
        }
    }
    maxcnt
}

/// # 130. 被围绕的区域
pub fn solve(board: &mut Vec<Vec<char>>) {
    fn dfs(board: &mut Vec<Vec<char>>, [i, j]: [usize; 2]) {
        let m = board.len();
        let n = board[0].len();
        if board[i][j] == 'O' {
            board[i][j] = '+';
        } else {
            return;
        }
        if 0 != i {
            dfs(board, [i - 1, j]);
        }
        if i != m - 1 {
            dfs(board, [i + 1, j]);
        }
        if 0 != j {
            dfs(board, [i, j - 1]);
        }
        if j != n - 1 {
            dfs(board, [i, j + 1]);
        }
    }
    let m = board.len();
    let n = board[0].len();
    for i in 0..m {
        dfs(board, [i, 0]);
        dfs(board, [i, n - 1]);
    }
    for j in 0..n {
        dfs(board, [0, j]);
        dfs(board, [m - 1, j]);
    }
    for i in 0..m {
        for j in 0..n {
            match board[i][j] {
                'X' | 'O' => board[i][j] = 'X',
                '+' => board[i][j] = 'O',
                _ => unreachable!(),
            }
        }
    }
}

/// # 134. 加油站
pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    // Solution 1: Timeout
    // let n = gas.len();
    // 'outer: for i in 0..n {
    //     let mut cnt = 0;
    //     for j in i..i + n {
    //         let j = j % n;
    //         cnt += gas[j];
    //         cnt -= cost[j];
    //         if cnt < 0 {
    //             continue 'outer;
    //         }
    //     }
    //     return i as i32;
    // }
    // -1

    // Solution 2
    let n = gas.len();
    let mut start = 0;
    let mut len = 0;
    let mut cnt = 0;
    while start < n && len < n {
        let i = (start + len) % n;
        cnt += gas[i] - cost[i];
        len += 1;
        while cnt < 0 && len != 0 {
            let i = start % n;
            cnt -= gas[i] - cost[i];
            start += 1;
            len -= 1;
        }
    }
    if len == n && 0 <= cnt {
        start as i32
    } else {
        -1
    }
}

/// # 137. 只出现一次的数字 II
pub fn single_number(nums: Vec<i32>) -> i32 {
    // Example
    let mut a = 0;
    let mut b = 0;
    for &num in &nums {
        b = !a & (b ^ num);
        a = !b & (a ^ num);
    }
    b
}

/// # 139. 单词拆分
pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    // Solution 1: Timeout
    // fn dfs(s: &[u8], word_dict: &Vec<String>) -> bool {
    //     if s.len() == 0 {
    //         return true;
    //     }
    //     for word in word_dict {
    //         let word = word.as_bytes();
    //         if let Some(s) = s.strip_prefix(word)
    //             && dfs(s, word_dict)
    //         {
    //             return true;
    //         }
    //     }
    //     false
    // }
    // let s = s.as_bytes();
    // dfs(s, &word_dict)

    // Example
    let s = s.as_bytes();
    let word_dict = word_dict
        .iter()
        .map(|word| word.as_bytes())
        .collect::<std::collections::HashSet<_>>();
    let n = s.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;
    for i in 1..=n {
        for j in 0..i {
            if dp[j] && word_dict.contains(&s[j..i]) {
                dp[i] = true;
                break;
            }
        }
    }
    dp[n]
}

/// # 152. 乘积最大子数组
pub fn max_product(nums: Vec<i32>) -> i32 {
    // Example
    nums[1..]
        .iter()
        .copied()
        .fold([nums[0]; 3], |[max, min, ans], num| {
            let [max, min] = [
                *[max * num, min * num, num].iter().max().unwrap(),
                *[max * num, min * num, num].iter().min().unwrap(),
            ];
            [max, min, ans.max(max)]
        })[2]
}

/// # 153. 寻找旋转排序数组中的最小值
pub fn find_min(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut l = 0;
    let mut r = n - 1;
    while l < r {
        let i = l + (r - l) / 2;
        if nums[i] < nums[r] {
            r = i;
        } else {
            l = i + 1;
        }
    }
    nums[l]
}

/// # 164. 最大间距
pub fn maximum_gap(nums: Vec<i32>) -> i32 {
    // Solution 1
    // let n = nums.len();
    // if n < 2 {
    //     return 0;
    // }
    // let mut nums = nums;
    // nums.sort();
    // let mut max = i32::MIN;
    // for i in 1..n {
    //     max = max.max(nums[i] - nums[i - 1]);
    // }
    // max

    // Example 1
    // let mut nums = nums;
    // let n = nums.len();
    // if n < 2 {
    //     return 0;
    // }
    // let mut nums = &mut nums;
    // let mut buf = &mut vec![0; n];
    // let max = *nums.iter().max().unwrap();
    // let mut exp = 1;
    // while max >= exp {
    //     let mut cnt = [0; 10];
    //     for &num in nums.iter() {
    //         let digit = ((num / exp) % 10) as usize;
    //         cnt[digit] += 1;
    //     }
    //     for i in 1..10 {
    //         cnt[i] += cnt[i - 1];
    //     }
    //     for &num in nums.iter().rev() {
    //         let digit = ((num / exp) % 10) as usize;
    //         cnt[digit] -= 1;
    //         buf[cnt[digit]] = num;
    //     }
    //     (nums, buf) = (buf, nums);
    //     exp *= 10;
    // }
    // let mut ans = i32::MIN;
    // for i in 1..n {
    //     ans = ans.max(nums[i] - nums[i - 1]);
    // }
    // ans

    // Example 2
    let n = nums.len();
    if n < 2 {
        return 0;
    }
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    for &num in &nums {
        min = num.min(min);
        max = num.max(max);
    }
    if min == max {
        return 0;
    }
    let diff = (max - min) as usize;
    let size = diff.div_ceil(n);
    let cnt = diff / size + 1;
    let mut buckets = vec![None::<[i32; 2]>; cnt];
    for &num in &nums {
        let i = (num - min) as usize / size;
        let [min, max] = buckets[i].get_or_insert([num; 2]);
        *min = num.min(*min);
        *max = num.max(*max);
    }
    let mut ans = 0;
    let [_, mut prev_max] = buckets[0].unwrap();
    for i in 1..cnt {
        if let Some([curr_min, curr_max]) = buckets[i] {
            ans = ans.max(curr_min - prev_max);
            prev_max = curr_max;
        }
    }
    ans
}

/// # 167. 两数之和 II - 输入有序数组
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let n = numbers.len();
    let mut l = 0;
    let mut r = n - 1;
    loop {
        use std::cmp::Ordering::*;
        let sum = numbers[l] + numbers[r];
        match sum.cmp(&target) {
            Equal => return vec![l as i32 + 1, r as i32 + 1],
            Less => l += 1,
            Greater => r -= 1,
        }
    }
}

/// # 179. 最大数
pub fn largest_number(nums: Vec<i32>) -> String {
    // Example
    let nums = nums.iter().map(i32::to_string).collect::<Vec<_>>();
    let mut nums = nums.iter().map(String::as_bytes).collect::<Vec<_>>();
    nums.sort_by(|num1, num2| {
        let s1 = num1.iter().chain(num2.iter()).copied();
        let s2 = num2.iter().chain(num1.iter()).copied();
        for (b1, b2) in s1.zip(s2) {
            if b1 != b2 {
                // Notice the sequence of b1 and b2
                return b2.cmp(&b1);
            }
        }
        std::cmp::Ordering::Equal
    });
    if nums[0] == b"0" {
        return String::from("0");
    }
    unsafe { String::from_utf8_unchecked(nums.join(&[][..])) }
}

/// # 209. 长度最小的子数组
pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    // Solution 1
    let n = nums.len();
    let mut l = 0;
    let mut r = 0;
    let mut sum = 0;
    let mut min = None;
    while r < n {
        while r < n && sum < target {
            sum += nums[r];
            r += 1;
        }
        if sum < target {
            break;
        }
        while l < r && target <= sum {
            sum -= nums[l];
            l += 1;
        }
        let diff = r - l + 1;
        let min = min.get_or_insert(diff);
        *min = diff.min(*min);
    }
    min.unwrap_or(0) as i32
}

/// # 213. 打家劫舍 II
pub fn rob(nums: Vec<i32>) -> i32 {
    match nums.len() {
        0 => 0,
        1 => nums[0],
        2 => nums[0].max(nums[1]),
        n => {
            let [y1, n1] = nums[2..n]
                .iter()
                .fold([nums[1], 0], |[y, n], &num| [n + num, y.max(n)]);
            let [y2, n2] = nums[1..n - 1]
                .iter()
                .fold([nums[0], 0], |[y, n], &num| [n + num, y.max(n)]);
            *[y1, n1, y2, n2].iter().max().unwrap()
        }
    }
}
