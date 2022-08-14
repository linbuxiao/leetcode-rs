/*
给你链表的头结点 head ，请将其按 升序 排列并返回 排序后的链表 。


 
示例 1：
<img alt="" src="https://assets.leetcode.com/uploads/2020/09/14/sort_list_1.jpg" style="width: 450px;" />
输入：head = [4,2,1,3]
输出：[1,2,3,4]

示例 2：
<img alt="" src="https://assets.leetcode.com/uploads/2020/09/14/sort_list_2.jpg" style="width: 550px;" />
输入：head = [-1,5,3,4,0]
输出：[-1,0,3,4,5]

示例 3：

输入：head = []
输出：[]

 
提示：

	链表中节点的数目在范围 [0, 5 * 10^4] 内
	-10^5 <= Node.val <= 10^5

 
进阶：你可以在 O(n log n) 时间复杂度和常数级空间复杂度下，对链表进行排序吗？


link: https://leetcode.cn/problems/sort-list/
*/

use std::{i32::MIN};

struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}


impl ListNode {
	fn set_next(&mut self, val: i32) {
		let new_node = Box::new(ListNode{val, next: None});
		self.next = Some(new_node);
	}
}

type SortListNode = Option<Box<ListNode>>;

impl Solution {
	pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		let mut arr = Self::get_arr(head.clone());
		arr.sort_unstable();
		Self::get_list(arr)
    }

	fn get_arr(head: Option<Box<ListNode>>) -> Vec<i32> {
		let mut result = vec![];
		let mut p = head;
		while let Some(node) = p {
			result.push(node.val);
			p = node.next;
		}
		result
	}

	fn get_list(mut arr: Vec<i32>) -> Option<Box<ListNode>> {
		if arr.is_empty() {
			return None ;
		}
		let mut prev = None ;
		while let Some(last) = arr.pop() {
			let mut node = ListNode::new(last);
			node.next = prev;
			prev = Some(Box::new(node));
		}
		prev
	}
}

/*
test cases:
[4,2,1,3]
*/
#[cfg(test)]
mod tests {
    use super::Solution;

	#[test]
	fn test_sort_list() {
		todo!()
	}
}