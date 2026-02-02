/// # 快乐数
pub fn is_happy(n: i32) -> bool {
    // Solution 1: elegant
    // use std::collections::HashMap;
    // use std::collections::hash_map::Entry::*;
    // let mut n = n;
    // let mut nexts = HashMap::new();
    // loop {
    //     match nexts.entry(n) {
    //         Occupied(o) => break *o.get() == 1,
    //         Vacant(v) => {
    //             let mut next = 0;
    //             while n != 0 {
    //                 next += (n % 10).pow(2);
    //                 n /= 10;
    //             }
    //             v.insert(next);
    //             n = next;
    //         }
    //     }
    // }

    // Solution 2: more elegant
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    let mut n = n;
    while seen.insert(n) {
        let mut next = 0;
        let mut curr = n;
        while curr != 0 {
            next += (curr % 10).pow(2);
            curr /= 10;
        }
        n = next;
    }
    n == 1
}

/// # 阶乘后的零
/// 阶乘分解为质数，2的数量总是多于5，所以只需要统计分解成的质数中的5的数量
pub fn trailing_zeroes(n: i32) -> i32 {
    let mut num = n;
    let mut cnt = 0;
    while num != 0 {
        num /= 5;
        cnt += num;
    }
    cnt
}
