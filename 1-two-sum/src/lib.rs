use std::collections::HashMap;

pub struct OriginalSolution;

impl OriginalSolution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len_nums = nums.len();

        for x1 in 0..len_nums {
            for x2 in 0..len_nums {
                if x1 < x2 && nums[x1] + nums[x2] == target {
                    return vec![x1 as i32, x2 as i32];
                }
            }
        }
        vec![-1, -2]
    }
}

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for (index, &num) in nums.iter().enumerate() {
            let complement = target - num;

            if let Some(&complement_idx) = map.get(&complement) {
                if index as i32 > complement_idx {
                    return vec![complement_idx, index as i32];
                }
                return vec![index as i32, complement_idx];
            }

            map.insert(num, index as i32);
        }
        vec![-1, -2]
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    fn make_test(nums: Vec<i32>, target: i32, expected: Vec<i32>) {
        assert_eq!(Solution::two_sum(nums, target), expected);
    }

    #[test]
    fn oneonetwo() {
        let nums = vec![1, 1];
        let target = 2;

        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    }

    #[test]
    fn oneonetwo_2() {
        make_test(vec![1, 1], 2, vec![0, 1]);
    }

    #[test]
    fn onefourtwothree() {
        let nums = vec![1, 4, 2];
        let target = 3;

        assert_eq!(Solution::two_sum(nums, target), vec![0, 2])
    }

    #[test]
    fn onetwothreefive() {
        let nums = vec![1, 2, 3];
        let target = 5;

        assert_eq!(Solution::two_sum(nums, target), vec![1, 2])
    }

    #[test]
    fn example1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        assert_eq!(Solution::two_sum(nums, target), vec![0, 1])
    }

    #[test]
    fn example2() {
        let nums = vec![3, 2, 4];
        let target = 6;

        make_test(nums, target, vec![1, 2])
    }

    #[test]
    fn example3() {
        make_test(vec![3, 3], 6, vec![0, 1])
    }
}
