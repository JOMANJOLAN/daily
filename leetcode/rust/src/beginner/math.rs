/// # Fizz Buzz
pub fn fizz_buzz(n: i32) -> Vec<String> {
    (1..=n)
        .map(|i| {
            let mod_3 = i % 3 == 0;
            let mod_5 = i % 5 == 0;
            match (mod_3, mod_5) {
                (false, false) => i.to_string(),
                (true, false) => "Fizz".to_string(),
                (false, true) => "Buzz".to_string(),
                (true, true) => "FizzBuzz".to_string(),
            }
        })
        .collect()
}

/// # 计数质数
pub fn count_primes(n: i32) -> i32 {
    // Solution 1
    // let mut vec = vec![true; n as usize + 2];
    // let mut cnt = 0;
    // for i in 2..n as usize {
    //     if vec[i] {
    //         let mut j = i;
    //         while j < vec.len() {
    //             vec[j] = false;
    //             j += i;
    //         }
    //         cnt += 1;
    //     }
    // }
    // cnt

    // Solution 2
    if n < 2 {
        return 0;
    }
    let n = n as usize;
    let sqrt = (n as f64).sqrt() as usize;
    let mut vec = vec![true; n];
    vec[0] = false;
    vec[1] = false;
    for i in 2..=sqrt {
        if vec[i] {
            let mut j = i * i;
            while j < n {
                vec[j] = false;
                j += i;
            }
        }
    }
    vec.iter().filter(|&&x| x).count() as i32
}

/// # 3的幂
pub fn is_power_of_three(n: i32) -> bool {
    // Solution 1
    // match n {
    //     ..3 => n == 1,
    //     _ => n == 3_i32.pow(n.ilog(3)),
    // }

    // Example
    n > 0 && 1162261467 % n == 0
}

/// # 罗马数字转整数
pub fn roman_to_int(s: String) -> i32 {
    // Solution 1
    // let s = s.as_bytes();
    // let mut sum = 0;
    // let mut i = 0;
    // while i < s.len() {
    //     match s[i] {
    //         // 1000
    //         b'M' => {
    //             i += 1;
    //             sum += 1000;
    //         }
    //         // 500
    //         b'D' => {
    //             i += 1;
    //             sum += 500;
    //         }
    //         // 100
    //         b'C' => {
    //             i += 1;
    //             match s.get(i) {
    //                 Some(b'M') => {
    //                     i += 1;
    //                     sum += 900;
    //                 }
    //                 Some(b'D') => {
    //                     i += 1;
    //                     sum += 400;
    //                 }
    //                 _ => sum += 100,
    //             }
    //         }
    //         // 50
    //         b'L' => {
    //             i += 1;
    //             sum += 50;
    //         }
    //         // 10
    //         b'X' => {
    //             i += 1;
    //             match s.get(i) {
    //                 Some(b'C') => {
    //                     i += 1;
    //                     sum += 90;
    //                 }
    //                 Some(b'L') => {
    //                     i += 1;
    //                     sum += 40;
    //                 }
    //                 _ => sum += 10,
    //             }
    //         }
    //         // 5
    //         b'V' => {
    //             i += 1;
    //             sum += 5;
    //         }
    //         // 1
    //         b'I' => {
    //             i += 1;
    //             match s.get(i) {
    //                 Some(b'X') => {
    //                     i += 1;
    //                     sum += 9;
    //                 }
    //                 Some(b'V') => {
    //                     i += 1;
    //                     sum += 4;
    //                 }
    //                 _ => sum += 1,
    //             }
    //         }
    //         _ => unreachable!(),
    //     }
    // }
    // sum

    // Example
    let mut last_str: Option<char> = None;
    let mut sum_of_str = 0;
    for one_str in s.chars().rev() {
        match one_str {
            'I' => {
                if last_str == Some('V') || last_str == Some('X') {
                    sum_of_str -= 1;
                } else {
                    sum_of_str += 1;
                }
                last_str = Some('I');
            }
            'V' => {
                sum_of_str += 5;
                last_str = Some('V');
            }
            'X' => {
                if last_str == Some('L') || last_str == Some('C') {
                    sum_of_str -= 10;
                } else {
                    sum_of_str += 10;
                }
                last_str = Some('X');
            }
            'L' => {
                sum_of_str += 50;
                last_str = Some('L');
            }
            'C' => {
                if last_str == Some('D') || last_str == Some('M') {
                    sum_of_str -= 100;
                } else {
                    sum_of_str += 100;
                }
                last_str = Some('C');
            }
            'D' => {
                sum_of_str += 500;
                last_str = Some('D');
            }
            'M' => {
                sum_of_str += 1000;
                last_str = Some('M');
            }
            _ => {
                println!("no str match");
            }
        }
    }
    sum_of_str
}
