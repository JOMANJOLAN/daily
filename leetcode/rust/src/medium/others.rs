/// # 两整数之和
pub fn get_sum(a: i32, b: i32) -> i32 {
    if a == 0 {
        b
    } else {
        get_sum((a & b) << 1, a ^ b)
    }
}

/// # 逆波兰表达式求值
pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = vec![];
    for token in &tokens {
        let num = match token.parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                let num2 = stack.pop().unwrap();
                let num1 = stack.pop().unwrap();
                match token.as_str() {
                    "+" => num1 + num2,
                    "-" => num1 - num2,
                    "*" => num1 * num2,
                    "/" => num1 / num2,
                    _ => unreachable!(),
                }
            }
        };
        stack.push(num);
    }
    stack.pop().unwrap()
}

/// # 多数元素
pub fn majority_element(nums: Vec<i32>) -> i32 {
    // Solution 1
    // use std::collections::HashMap;
    // let mut map = HashMap::new();
    // for &num in &nums {
    //     let cnt = map.entry(num).or_insert(0);
    //     *cnt += 1;
    //     if *cnt > nums.len() / 2 {
    //         return num;
    //     }
    // }
    // unreachable!()

    // Example
    let mut cnt = 1;
    let mut res = nums[0];
    for i in 1..nums.len() {
        if res == nums[i] {
            cnt += 1;
        } else {
            cnt -= 1;
            if cnt == 0 {
                res = nums[i + 1];
            }
        }
    }
    res
}

/// # 任务调度器
pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    let mut map = [0; 26];
    for &task in &tasks {
        let i = task as usize - 'A' as usize;
        map[i] += 1;
    }
    let maxcnt = *map.iter().max().unwrap();
    let mut slots = (maxcnt - 1) * (n + 1);
    for &cnt in &map {
        if cnt == maxcnt {
            slots += 1;
        }
    }
    slots.max(tasks.len() as i32)
}
