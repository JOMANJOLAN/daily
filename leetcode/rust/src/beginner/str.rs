/// # 反转字符串
pub fn reverse_string(s: &mut Vec<char>) {
    // Solution 1
    // s.reverse();

    // Solution 2
    let len = s.len();
    for i in 0..len / 2 {
        s.swap(i, len - 1 - i);
    }
}

/// # 整数反转
pub fn reverse_integer(x: i32) -> i32 {
    // Solution 1
    // let mut x = x;
    // let mut y = 0_i32;
    // while x != 0 {
    //     let n = x % 10;
    //     x /= 10;
    //     match y.checked_mul(10) {
    //         Some(z) => y = z,
    //         None => return 0,
    //     }
    //     match y.checked_add(n) {
    //         Some(z) => y = z,
    //         None => return 0,
    //     }
    // }
    // y

    // Solution 2
    let mut x = x;
    let mut y = 0_i32;
    while x != 0 {
        let n = x % 10;
        x /= 10;
        match y.checked_mul(10).and_then(|v| v.checked_add(n)) {
            Some(v) => y = v,
            None => return 0,
        }
    }
    y
}

/// # 字符串中的第一个唯一字符
pub fn first_uniq_char(s: String) -> i32 {
    // Solution 1
    // let mut record = [[-1_i32, 0]; 26];
    // for (i, b) in s.bytes().enumerate() {
    //     let idx = (b - b'a') as usize;
    //     if record[idx][0] == -1 {
    //         record[idx][0] = i as i32;
    //     }
    //     record[idx][1] += 1;
    // }
    // record
    //     .iter()
    //     .filter(|[_, count]| *count == 1)
    //     .min_by_key(|[idx, _]| *idx)
    //     .map(|[idx, _]| *idx)
    //     .unwrap_or(-1)

    // Example
    // let mut counts = [0; 26];
    // for byte in s.bytes() {
    //     counts[(byte - b'a') as usize] += 1;
    // }
    // for (i, byte) in s.bytes().enumerate() {
    //     if counts[(byte - b'a') as usize] == 1 {
    //         return i as i32;
    //     }
    // }
    // -1

    // Solution 2: merge solution 1 & example
    let mut counts = [0; 26];
    for byte in s.bytes() {
        counts[(byte - b'a') as usize] += 1;
    }
    s.bytes()
        .position(|b| counts[(b - b'a') as usize] == 1)
        .map_or(-1, |i| i as i32)
}

/// # 有效的字母异位词
pub fn is_anagram(s: String, t: String) -> bool {
    // Solution 1
    // let mut cnts = [0; 26];
    // for b in s.bytes() {
    //     cnts[(b - b'a') as usize] += 1;
    // }
    // for b in t.bytes() {
    //     cnts[(b - b'a') as usize] -= 1;
    // }
    // !cnts.iter().any(|&cnt| cnt != 0)

    // Example
    // if s.len() != t.len() {
    //     return false;
    // }
    // let mut count = [0; 26];
    // for c in s.bytes() {
    //     if c >= b'a' && c <= b'z' {
    //         count[(c - b'a') as usize] += 1;
    //     }
    // }
    // for c in t.bytes() {
    //     if c >= b'a' && c <= b'z' {
    //         count[(c - b'a') as usize] -= 1;
    //     }
    // }
    // count.iter().all(|&cnt| cnt == 0)

    // Solution 2: merge solution 1 & example
    if s.len() != t.len() {
        return false;
    }
    let mut cnts = [0; 26];
    for b in s.bytes() {
        if let b @ b'a'..b'z' = b {
            cnts[(b - b'a') as usize] += 1;
        }
    }
    for b in t.bytes() {
        if let b @ b'a'..b'z' = b {
            cnts[(b - b'a') as usize] += 1;
        }
    }
    !cnts.iter().any(|&cnt| cnt != 0)
}

/// # 验证回文串
pub fn is_palindrome(s: String) -> bool {
    let bytes = s.as_bytes();
    let mut left = 0;
    let mut right = bytes.len() - 1;
    loop {
        while left < right && !bytes[left].is_ascii_alphanumeric() {
            left += 1
        }
        while left < right && !bytes[right].is_ascii_alphanumeric() {
            right -= 1;
        }
        if !(left < right) {
            break;
        }
        if !bytes[left].eq_ignore_ascii_case(&bytes[right]) {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

/// # 字符串转换整数 (atoi)
pub fn my_atoi(s: String) -> i32 {
    // Solution 1
    let bytes = s.trim().as_bytes();
    if bytes.len() == 0 {
        return 0;
    }
    let mut sign = 1;
    let mut num = 0_i32;
    let mut i = 0;
    match bytes[i] {
        b'-' => {
            sign = -1;
            i += 1;
        }
        b'+' => {
            sign = 1;
            i += 1;
        }
        _ => (),
    }
    while i < bytes.len() && bytes[i] == b'0' {
        i += 1;
    }
    for &b in &bytes[i..] {
        if !b.is_ascii_digit() {
            break;
        }
        match num
            .checked_mul(10)
            .and_then(|n| n.checked_add(((b - b'0') as i32) * sign))
        {
            Some(n) => num = n,
            None => {
                num = if sign < 0 { i32::MIN } else { i32::MAX };
                return num;
            }
        }
    }
    num
}

/// # 实现 strStr()
pub fn str_str(haystack: String, needle: String) -> i32 {
    // Solution 1
    let text = haystack.as_bytes();
    let pattern = needle.as_bytes();

    if text.len() < pattern.len() {
        return -1;
    }

    let mut next = vec![0_usize; pattern.len()];
    let mut i = 0;
    let mut j = 1;

    while j < pattern.len() {
        while i > 0 && pattern[i] != pattern[j] {
            i = next[i - 1];
        }

        if pattern[i] == pattern[j] {
            i += 1;
        }

        next[j] = i;
        j += 1;
    }

    i = 0;
    j = 0;

    while i < text.len() {
        while j > 0 && text[i] != pattern[j] {
            j = next[j - 1];
        }

        if text[i] == pattern[j] {
            j += 1;
        }

        if j == pattern.len() {
            return (i - j + 1) as i32;
        }

        i += 1;
    }

    -1
}

/// # 外观数列
pub fn count_and_say(n: i32) -> String {
    // Solution 1
    let n = n - 1;

    let mut curr = vec![b'1'];
    let mut next = vec![];

    for _ in 0..n {
        let mut left = 0;
        let mut right = 0;
        while right < curr.len() {
            while right < curr.len() && curr[right] == curr[left] {
                right += 1;
            }
            let cnt = (right - left) as u8 + b'0';
            let num = curr[left];
            next.push(cnt);
            next.push(num);
            left = right;
        }
        (curr, next) = (next, curr);
        next.clear();
    }

    unsafe { String::from_utf8_unchecked(curr) }
}

/// # 最长公共前缀
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let bytes1 = strs.iter().min_by_key(|s| s.len()).unwrap().as_bytes();
    let mut prefix_len = bytes1.len();
    for s in strs.iter() {
        let bytes2 = s.as_bytes();
        prefix_len = bytes1[0..prefix_len]
            .iter()
            .zip(bytes2.iter())
            .take_while(|(b1, b2)| **b1 == **b2)
            .count();
        if prefix_len == 0 {
            break;
        }
    }
    unsafe { String::from_utf8_unchecked(bytes1[0..prefix_len].to_vec()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negtive_mod_test() {
        println!("{}", -3 % 10)
    }

    #[test]
    fn reverse_test() {
        println!("{}", reverse_integer(-321))
    }

    #[test]
    fn is_palindrome_test() {
        println!("{}", is_palindrome("0P".to_string()));
    }

    #[test]
    fn str_str_test() {
        str_str("sadbutsad".to_string(), "sad".to_string());
    }
}
