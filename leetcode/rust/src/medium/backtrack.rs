/// # 电话号码的字母组合
pub fn letter_combinations(digits: String) -> Vec<String> {
    fn generate(map: &[&[u8]], digits: &[u8], buf: &mut [u8], depth: usize, vec: &mut Vec<String>) {
        if depth == digits.len() {
            unsafe { vec.push(String::from_utf8_unchecked(buf.to_vec())) };
            return;
        }
        for &note in map[(digits[depth] - b'2') as usize] {
            buf[depth] = note;
            generate(map, digits, buf, depth + 1, vec);
        }
    }
    let map: [&'static [u8]; _] = [
        b"abc", b"def", b"ghi", b"jkl", b"mno", b"pqrs", b"tuv", b"wxyz",
    ];
    let digits = digits.as_bytes();
    let mut buf = vec![0; digits.len()];
    let mut vec = vec![];
    generate(&map, digits, &mut buf, 0, &mut vec);
    vec
}

/// # 括号生成
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    fn generate(left: usize, right: usize, depth: usize, buf: &mut [u8], vec: &mut Vec<String>) {
        if left == 0 && right == 0 {
            unsafe { vec.push(String::from_utf8_unchecked(buf.to_vec())) };
            return;
        }
        if 0 < left {
            buf[depth] = b'(';
            generate(left - 1, right, depth + 1, buf, vec);
        }
        if right > left {
            buf[depth] = b')';
            generate(left, right - 1, depth + 1, buf, vec);
        }
    }
    let n = n as usize;
    let mut buf = vec![0; n * 2];
    let mut vec = vec![];
    generate(n, n, 0, &mut buf, &mut vec);
    vec
}

/// # 全排列
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn generate(nums: &mut [i32], depth: usize, vec: &mut Vec<Vec<i32>>) {
        if nums.len() <= depth {
            vec.push(nums.to_vec());
            return;
        }
        for i in depth..nums.len() {
            nums.swap(i, depth);
            generate(nums, depth + 1, vec);
            nums.swap(i, depth);
        }
    }
    let mut nums = nums;
    let mut vec = vec![];
    generate(&mut nums, 0, &mut vec);
    vec
}

/// # 子集
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn generate(
        nums: &[i32],
        depth: usize,
        count: usize,
        buf: &mut [i32],
        vec: &mut Vec<Vec<i32>>,
    ) {
        if nums.len() <= depth {
            vec.push(buf[0..count].to_vec());
            return;
        }
        generate(nums, depth + 1, count, buf, vec);
        buf[count] = nums[depth];
        generate(nums, depth + 1, count + 1, buf, vec);
    }
    let mut buf = vec![0; nums.len()];
    let mut vec = vec![];
    generate(&nums, 0, 0, &mut buf, &mut vec);
    vec
}

/// # 单词搜索
pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    fn bfs(board: &mut Vec<Vec<char>>, word: &[char], depth: usize, [i, j]: [usize; 2]) -> bool {
        if board[i][j] == word[depth] {
            if depth == word.len() - 1 {
                return true;
            }
            let chr = board[i][j];
            board[i][j] = '\0';
            if i != 0 {
                if bfs(board, word, depth + 1, [i - 1, j]) {
                    board[i][j] = chr;
                    return true;
                }
            }
            if i != board.len() - 1 {
                if bfs(board, word, depth + 1, [i + 1, j]) {
                    board[i][j] = chr;
                    return true;
                }
            }
            if j != 0 {
                if bfs(board, word, depth + 1, [i, j - 1]) {
                    board[i][j] = chr;
                    return true;
                }
            }
            if j != board[0].len() - 1 {
                if bfs(board, word, depth + 1, [i, j + 1]) {
                    board[i][j] = chr;
                    return true;
                }
            }
            board[i][j] = chr;
        }
        false
    }
    let mut board = board;
    let word = word.chars().collect::<Vec<_>>();
    let m = board.len();
    let n = board[0].len();
    for i in 0..m {
        for j in 0..n {
            if bfs(&mut board, &word, 0, [i, j]) {
                return true;
            }
        }
    }
    false
}
