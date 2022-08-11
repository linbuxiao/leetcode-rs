/*
给你一个单链表的头节点 head ，请你判断该链表是否为回文链表。如果是，返回 true ；否则，返回 false 。
 
示例 1：
<img alt="" src="https://assets.leetcode.com/uploads/2021/03/03/pal1linked-list.jpg" style="width: 422px; height: 62px;" />
输入：head = [1,2,2,1]
输出：true

示例 2：
<img alt="" src="https://assets.leetcode.com/uploads/2021/03/03/pal2linked-list.jpg" style="width: 182px; height: 62px;" />
输入：head = [1,2]
输出：false

 
提示：

	链表中节点数目在范围[1, 10^5] 内
	0 <= Node.val <= 9

 
进阶：你能否用 O(n) 时间复杂度和 O(1) 空间复杂度解决此题？


link: https://leetcode.cn/problems/palindrome-linked-list/
*/

use std::vec;
use crate::util::list::ListNode;

struct Solution {}


impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode<i32>>>) -> bool {
		let mut arr: Vec<i32> = vec![];
		let mut p = head;
		while p != None {
			let box_node = p.unwrap();
			arr.push(box_node.val);
			p = box_node.next;
		}
		let mut arr_clone = arr.clone();
		arr_clone.reverse();
		if arr_clone == arr {
			return true
		}
		return false
    }
}

/*
test cases:
[1,2,2,1]
*/
#[cfg(test)]
mod tests {
    use crate::util::list::create_by_vec_i32;

    use super::{Solution};

	#[test]
	fn test_palindrome_linked_list() {
		let list = create_by_vec_i32(vec![1,2,2,1] );
		assert_eq!(true, Solution::is_palindrome(list));
	}
}