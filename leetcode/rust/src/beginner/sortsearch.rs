/// # 合并两个有序数组
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    // // Solution 1
    // use std::cmp::Ordering;
    // let mut i = m as usize;
    // let mut j = n as usize;
    // let mut k = nums1.len();
    // while 0 < j && 0 < i {
    //     match nums1[i - 1].cmp(&nums2[j - 1]) {
    //         Ordering::Greater => {
    //             nums1[k - 1] = nums1[i - 1];
    //             i -= 1;
    //             k -= 1;
    //         }
    //         Ordering::Less => {
    //             nums1[k - 1] = nums2[j - 1];
    //             j -= 1;
    //             k -= 1;
    //         }
    //         Ordering::Equal => {
    //             nums1[k - 1] = nums1[i - 1];
    //             nums1[k - 2] = nums2[j - 1];
    //             i -= 1;
    //             j -= 1;
    //             k -= 2;
    //         }
    //     }
    // }
    // // not nessaserry
    // // while 0 < i {
    // //     nums1[k - 1] = nums1[i - 1];
    // //     i -= 1;
    // //     k -= 1;
    // // }
    // while 0 < j {
    //     nums1[k - 1] = nums2[j - 1];
    //     j -= 1;
    //     k -= 1;
    // }

    // Example
    let m = m as usize;
    let n = n as usize;
    let mut p1 = m - 1;
    let mut p2 = n - 1;
    let mut p = m + n - 1;

    while p2 < n {
        if p1 < m && nums1[p1] > nums2[p2] {
            nums1[p] = nums1[p1];
            p1 -= 1;
        } else {
            nums1[p] = nums2[p2];
            p2 -= 1;
        }
        p -= 1;
    }
}

/// # 第一个错误的版本
pub struct BadVersionApi;

impl BadVersionApi {
    #[allow(non_snake_case)]
    pub fn isBadVersion(&self, _version: i32) -> bool {
        false
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        // Solution 1
        let mut lower = 1;
        let mut upper = n;
        while lower < upper {
            let version = lower + (upper - lower) / 2;
            if self.isBadVersion(version) {
                upper = version;
            } else {
                lower = version + 1;
            }
        }
        lower
    }
}
