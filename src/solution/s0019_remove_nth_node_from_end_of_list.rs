/*
给你一个链表，删除链表的倒数第 n 个结点，&并且返回链表的头结点。
 
示例 1：
<img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/remove_ex1.jpg" style="width: 542px; height: 222px;" />
输入：head = [1,2,3,4,5], n = 2
输出：[1,2,3,5]

示例 2：

输入：head = [1], n = 1
输出：[]

示例 3：

输入：head = [1,2], n = 1
输出：[1]

 
提示：

	链表中结点的数目为 sz
	1 <= sz <= 30
	0 <= Node.val <= 100
	1 <= n <= sz

 
进阶：你能尝试使用一趟扫描实现吗？


link: https://leetcode.cn/problems/remove-nth-node-from-end-of-list/
*/

use std::rc::Rc;

use crate::util::list::ListNode;

struct Solution {}

// Definition for singly-linked list.

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
		let mut dummy = Box::new(ListNode{ val: 0, next: head });
		// 做一份可变引用，以供删除
		let mut slow = dummy.as_mut();
		// 做一份不可变引用
		let mut fast = &slow.clone();
		// fast 先走 n 步
		let mut nn = n.clone();
		while nn > 0 {
			fast = fast.next.as_ref().unwrap().as_ref();
			nn = nn - 1;
		}
		// 一起走，直到快的到底
		while fast.next.is_some() {
			fast = fast.next.as_ref().unwrap().as_ref();
			slow = slow.next.as_mut().unwrap().as_mut();
		}
		slow.remove_next();
		dummy.next
    }
}

/*
test cases:
[1,2,3,4,5]
2
*/
#[cfg(test)]
mod tests {
    use super::{Solution, ListNode};

	#[test]
	fn test_remove_nth_node_from_end_of_list() {
		let mut list = ListNode::new(1);
		list.push(2);
		list.push(3);
		list.push(4);
		list.push(5);
		let head = Some(Box::new(list));
		Solution::remove_nth_from_end(head.clone(), 2);
		println!("{:?}", head.clone());
	}
}