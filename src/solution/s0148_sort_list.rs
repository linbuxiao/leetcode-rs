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

use std::i32::MIN;

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
	fn exchange_node(&mut self) {
		if let Some(mut b) = self.next.clone() {
			if let Some(mut c) = b.next {
				b.next = c.next.clone();
				c.next = Some(b);
				self.next = Some(c);
			}
		}
	}

	fn is_last(&self) -> bool {
		let cp = self.clone().next;
		if cp.is_none() {
			return false;
		};
		if cp.next.is_none() || cp.next.unwrap().next.is_none() {
			return true ;
		}
		false
	}
}
impl Solution {
    pub fn sort_list_bubbling(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		if head.is_none() {
			return None
		}
		let mut dummy = Box::new(ListNode{val: MIN, next: head});
		let mut p = dummy.as_mut();
		while p.next.clone().is_some() {
			let mut f = true;
			while p.next.clone().is_some() && p.next.clone().unwrap().next.is_some() {
				if let Some(b) = p.next.clone() {
					if let Some(c) = b.clone().next {
						if b.as_ref().val > c.as_ref().val {
							f = false;
							p.exchange_node();
						}
					}
				}
				p  = p.next.as_mut().unwrap().as_mut();
			}
			if f { break; }
			p = dummy.as_mut();
		}
		dummy.next
    }

	pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		let cp = head.clone();
		if cp.is_none() {
			return None ;
		};
		if cp.unwrap().is_last() {
			let mut dummy = Box::new(ListNode{val: 0, next: head});
			dummy.exchange_node();
			return dummy.next;
		}


		todo!()
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