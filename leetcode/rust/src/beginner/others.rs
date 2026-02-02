/// # 位1的个数
pub fn hamming_weight(n: i32) -> i32 {
    // Solution 1
    let mut n = n;
    let mut cnt = 0;
    while n != 0 {
        if (n & 1) == 1 {
            cnt += 1;
        }
        n >>= 1;
    }
    cnt
}

/// # 汉明距离
pub fn hamming_distance(x: i32, y: i32) -> i32 {
    let mut n = x ^ y;
    let mut cnt = 0;
    while n != 0 {
        if (n & 1) == 1 {
            cnt += 1;
        }
        n >>= 1;
    }
    cnt
}

/// # 颠倒二进制位
pub fn reverse_bits(n: i32) -> i32 {
    // Solution 1
    (0..size_of::<i32>() * 8).fold([0, n], |[m, n], _| [(m << 1) | (n & 1), n >> 1])[0]
}

/// # 杨辉三角
pub trait Generate {
    fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        match num_rows {
            1 => vec![vec![1]],
            2 => vec![vec![1], vec![1, 1]],
            n => {
                let mut vecs = Self::generate(n - 1);
                let vec = {
                    let last = vecs.last().unwrap();
                    let mut vec = Vec::with_capacity(n as usize);
                    vec.push(1);
                    for i in 1..last.len() {
                        vec.push(last[i - 1] + last[i]);
                    }
                    vec.push(1);
                    vec
                };
                vecs.push(vec);
                vecs
            }
        }
    }
}

/// # 有效的括号
pub fn is_valid(s: String) -> bool {
    // Solution 1
    let s = s.as_bytes();
    let mut stack = Vec::new();
    for &b in s {
        match b {
            b'{' | b'[' | b'(' => stack.push(b),
            b'}' | b']' | b')' => {
                let m = match stack.pop() {
                    Some(p) => match b {
                        b'}' => p == b'{',
                        b']' => p == b'[',
                        b')' => p == b'(',
                        _ => unreachable!(),
                    },
                    None => false,
                };
                if !m {
                    return false;
                }
            }
            _ => unreachable!(),
        }
    }
    stack.is_empty()
}

/// # 缺失数字
pub fn missing_number(nums: Vec<i32>) -> i32 {
    // Solution 1
    // let mut vec = vec![true; nums.len() + 1];
    // for &num in &nums {
    //     vec[num as usize] = false;
    // }
    // vec.iter().position(|&v| v).unwrap() as i32

    // Example
    let n = nums.len() as i32;
    let expected_sum = n * (n + 1) / 2;
    let actual_sum: i32 = nums.iter().sum();
    expected_sum - actual_sum
}
