/*
给你单链表的头节点 head ，请你反转链表，并返回反转后的链表。
<div class="original__bRMd">
<div>
 
示例 1：
<img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/rev1ex1.jpg" style="width: 542px; height: 222px;" />
输入：head = [1,2,3,4,5]
输出：[5,4,3,2,1]

示例 2：
<img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/rev1ex2.jpg" style="width: 182px; height: 222px;" />
输入：head = [1,2]
输出：[2,1]

示例 3：

输入：head = []
输出：[]

 
提示：

	链表中节点的数目范围是 [0, 5000]
	-5000 <= Node.val <= 5000

 
进阶：链表可以选用迭代或递归方式完成反转。你能否用两种方法解决这道题？
</div>
</div>


link: https://leetcode.cn/problems/reverse-linked-list/
*/

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
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		let mut cur = head.clone();
		let mut arr = vec![];
		while let Some(node) = cur {
			arr.insert(0, node.val);
			cur = node.next;
		}
		let mut p = None ;
		while let Some(v) = arr.pop() {
			let mut new_node =ListNode::new(v);
			new_node.next = p;
			p = Some(Box::new(new_node));
		}
		p
    }
}

/*
test cases:
[1,2,3,4,5]
*/
#[cfg(test)]
mod tests {
    use super::Solution;

	#[test]
	fn test_reverse_linked_list() {

		todo!()
	}
}