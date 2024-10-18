# 1. Two sums

Given an array of integers `nums` and target `target`, return the indices of the two numbers in `nums` such that they add up to `target`.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.

## Initial solution

My initial solution was to use a nested loop to iterate over the array and find the two numbers that add up to the target. The time complexity of this solution is $O(n^2)$ because we have to iterate over the array twice.

```rust
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
```

This does work, but it is slower than it could be. It is often the case when traversing a nested loop that we want to store the results as we are traversing the array the first time.  

In this case, I will use a hash map to store a mapping from the value in the array to the index of that value. 

### Example

Given 
- `nums = [2, 7, 11, 15]`
- `target = 9`

We expect the solution to be `[0, 1]` because 

```nums[0] + nums[1] = 2 + 7 = 9```

#### Step by step

1. Initialize hash map `map`. 
2. Observe the first element in the array `nums[0] = 2`.
3. Add the element to the hash map with the index as the value. `map[2] = 0`.
4. Observe the second element in the array `nums[1] = 7`, and note that in order to reach the target, we need `target - nums[1] = 9 - 7 = 2`.
5. Check if `2` is in the hash map. 
6. It is, and the value (eg the index in the original array containing `2`) is `0`.
7. Return the indices `[0, 1]`.

## Optimized solution

```rust 
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
```
