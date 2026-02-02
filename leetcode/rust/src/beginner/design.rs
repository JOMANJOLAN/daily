/// # 打乱数组
pub struct Shuffle {
    original: Vec<i32>,
    nums: Vec<i32>,
}

impl Shuffle {
    pub fn new(nums: Vec<i32>) -> Self {
        Shuffle {
            original: nums.clone(),
            nums,
        }
    }

    pub fn reset(&self) -> Vec<i32> {
        self.original.clone()
    }

    pub fn shuffle(&mut self) -> Vec<i32> {
        use rand::seq::SliceRandom;
        self.nums.shuffle(&mut rand::rng());
        self.nums.clone()
    }
}

/// 最小栈
// // Solution 1
// pub struct MinStack {
//     vec: Vec<i32>,
//     min: i32,
// }

// impl MinStack {
//     pub fn new() -> Self {
//         Self {
//             vec: Vec::new(),
//             min: 0,
//         }
//     }

//     pub fn push(&mut self, val: i32) {
//         if self.vec.is_empty() {
//             self.min = val;
//         }
//         self.min = self.min.min(val);
//         self.vec.push(val);
//     }

//     pub fn pop(&mut self) {
//         self.vec.pop();
//         if !self.vec.is_empty() {
//             self.min = *self.vec.iter().min().unwrap();
//         }
//     }

//     pub fn top(&mut self) -> i32 {
//         *self.vec.last().unwrap()
//     }

//     pub fn get_min(&self) -> i32 {
//         self.min
//     }
// }

// Example
pub struct MinStack {
    mins_stack: Vec<i32>,
    stack: Vec<i32>,
    min: i32,
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            mins_stack: Vec::new(),
            min: i32::MAX,
            stack: Vec::new(),
        }
    }

    pub fn push(&mut self, val: i32) {
        if val <= self.min {
            self.mins_stack.push(val);
            self.min = val;
        }
        self.stack.push(val);
    }

    pub fn pop(&mut self) {
        if self.stack.len() == 0 {
            return;
        }
        let v = self.stack.pop().unwrap();
        if v == self.min {
            self.mins_stack.pop();
            if self.mins_stack.len() > 0 {
                self.min = self.mins_stack[self.mins_stack.len() - 1];
            } else {
                self.min = i32::MAX;
            }
        }
    }

    pub fn top(&self) -> i32 {
        if self.stack.is_empty() {
            0
        } else {
            self.stack[self.stack.len() - 1]
        }
    }

    pub fn get_min(&self) -> i32 {
        self.min
    }
}
