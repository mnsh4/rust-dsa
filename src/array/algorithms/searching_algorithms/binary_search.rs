// Given an array of integers nums which is sorted in ascending order, and an integer target, write a function to search target in nums. If target exists, then return its index. Otherwise, return -1.

// You must write an algorithm with O(log n) runtime complexity.

// Example 1:
// Input: nums = [-1,0,3,5,9,12], target = 9
// Output: 4
// Explanation: 9 exists in nums and its index is 4

// Example 2:
// Input: nums = [-1,0,3,5,9,12], target = 2
// Output: -1
// Explanation: 2 does not exist in nums so return -1

impl Solution {
    pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32;

        while left < right {
            let mid = left + (right - left) / 2;

            if nums[mid as usize] == target {
                return mid as i32; 
            } else if nums[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        -1 
    }

    // pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    //     let mut is_asc = true;
    //     if nums.len() > 1 {
    //         is_asc = nums[0] < nums[nums.len() - 1];
    //     }
    //     let mut left = 0;
    //     let mut right = nums.len();

    //     while left < right {
    //         let mid = left + (right - left) / 2;

    //         if is_asc {
    //             match target.cmp(&nums[mid]) {
    //                 Ordering::Less => right = mid,
    //                 Ordering::Equal => return mid as i32,
    //                 Ordering::Greater => left = mid + 1,
    //             }
    //         } else {
    //             match target.cmp(&nums[mid]) {
    //                 Ordering::Less => left = mid + 1,
    //                 Ordering::Equal => return mid as i32,
    //                 Ordering::Greater => right = mid,
    //             }
    //         }
    //     }
    //     -1
    // }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search_found() {
        let arr = vec![-1, 0, 3, 5, 9, 12];
        let target = &9;

        assert_eq!(binary_search(&arr, target), Some(4));
    }

    #[test]
    fn test_binary_search_not_found() {
        let arr = vec![-1, 0, 3, 5, 9, 12];
        let target = &2; 

        assert_eq!(binary_search(&arr, target), None);
    }
}
