/*
给你一个整数数组 nums，有一个大小为 k 的滑动窗口从数组的最左侧移动到数组的最右侧。你只可以看到在滑动窗口内的 k 个数字。滑动窗口每次只向右移动一位。
返回 滑动窗口中的最大值 。
 
示例 1：

输入：nums = [1,3,-1,-3,5,3,6,7], k = 3
输出：[3,3,5,5,6,7]
解释：
滑动窗口的位置                最大值
---------------               -----
[1  3  -1] -3  5  3  6  7       3
 1 [3  -1  -3] 5  3  6  7       3
 1  3 [-1  -3  5] 3  6  7       5
 1  3  -1 [-3  5  3] 6  7       5
 1  3  -1  -3 [5  3  6] 7       6
 1  3  -1  -3  5 [3  6  7]      7

示例 2：

输入：nums = [1], k = 1
输出：[1]

 
提示：

	1 <= nums.length <= 10^5
	-10^4 <= nums[i] <= 10^4
	1 <= k <= nums.length


link: https://leetcode.cn/problems/sliding-window-maximum/
*/

use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
		let k = k as usize;
		let mut queue: VecDeque<usize> = VecDeque::new();
		let mut result: Vec<i32> = vec![];
		for i in 0..nums.len() {
			let val = nums[i];
			while let Some(last) = queue.back() {
				if nums[*last] < val {
					queue.pop_back();
				} else {
					break;
				}
			}
			queue.push_back(i);
			if i >= k && *queue.front().unwrap() ==  i - k {
				queue.pop_front();
			}

			if i >= k - 1  {
				result.push(nums[*queue.front().unwrap()])
			}
		}

		result
    }
}

/*
test cases:
[1,3,-1,-3,5,3,6,7]
3
*/
#[cfg(test)]
mod tests {
    use super::Solution;

	#[test]
	fn test_sliding_window_maximum() {
		let wait_arr = vec![1,3,-1,-3,5,3,6,7];
		Solution::max_sliding_window(wait_arr, 3);
	}
}