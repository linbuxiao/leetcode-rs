/*
给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。
请注意 ，必须在不复制数组的情况下原地对数组进行操作。
 
示例 1:

输入: nums = [0,1,0,3,12]
输出: [1,3,12,0,0]

示例 2:

输入: nums = [0]
输出: [0]
 
提示:
<meta charset="UTF-8" />

	1 <= nums.length <= 10^4
	-2^31 <= nums[i] <= 2^31 - 1

 
进阶：你能尽量减少完成的操作次数吗？


link: https://leetcode.cn/problems/move-zeroes/
*/
#![allow(unused)]
struct Solution {}

impl Solution {
    pub fn move_zeroes_slow(nums: &mut Vec<i32>) {
		let mut zero_index = vec![];
		for i in 0..nums.len() {
			if nums[i] == 0 {
				zero_index.push(i);
			}
		}
		let len = zero_index.len();
		zero_index.reverse();
		for index in zero_index {
			nums.remove(index);
		}
		for _ in 0..len {
			nums.push(0);
		}
    }

	pub fn move_zeroes(nums: &mut Vec<i32>) {
		if nums.len() == 1 {
			return ;
		}
		let mut left = 0;
		let mut right = 1;
		let len = nums.len();
		while right < len {
			while left < len && nums[left] != 0 {
				left  = left + 1;
			}
			right = left;
			while right < len && (nums[right] == 0) {
				right = right + 1;
			}
			if left == len || right == len {
				return ;
			}
			let tmp = nums[right];
			nums[right] = nums[left];
			nums[left] = tmp;
		}
    }
}

/*
test cases:
[0,1,0,3,12]
*/
#[cfg(test)]
mod tests {
    use super::Solution;

	#[test]
	fn test_move_zeroes() {
		// let mut arr = vec![0,1,0,3,12];
		// Solution::move_zeroes(&mut arr);
		// assert_eq!(vec![1,3,12,0,0], arr);
		let mut arr_1 = vec![4,2,4,0,0,3,0,5,1,0];
		Solution::move_zeroes(&mut arr_1);
		assert_eq!(vec![4,2,4,3,5,1,0,0,0,0], arr_1);
	}
}