/*
将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。 
 
示例 1：
<img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/merge_ex1.jpg" style="width: 662px; height: 302px;" />
输入：l1 = [1,2,4], l2 = [1,3,4]
输出：[1,1,2,3,4,4]

示例 2：

输入：l1 = [], l2 = []
输出：[]

示例 3：

输入：l1 = [], l2 = [0]
输出：[0]

 
提示：

	两个链表的节点数目范围是 [0, 50]
	-100 <= Node.val <= 100
	l1 和 l2 均按 非递减顺序 排列


link: https://leetcode.cn/problems/merge-two-sorted-lists/
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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		let mut p0 = list1.clone();
		let mut p1 = list2.clone();
		let mut arr = vec![];
		while let Some(ref p0_val) = p0 {
			match p1 {
				Some(ref p1_val) => {
					// small push
					if p0_val.val < p1_val.val {
						arr.push(p0_val.val);
						p0 = p0_val.next.clone();
					} else {
						arr.push(p1_val.val);
						p1 = p1_val.next.clone();
					}
				}
				None => {
					// list push all p0 value
					arr.push(p0_val.val);
					p0 = p0_val.next.clone();					
				}
			}
		}

		// p1 maybe have some value
		while let Some(ref node) = p1 {
			arr.push(node.val);
			p1 = node.next.clone();
		}

		let mut prev = None ;
		while let Some(v) = arr.pop() {
			let mut new_node = ListNode::new(v);
			new_node.next = prev;
			prev = Some(Box::new(new_node));
		}
		prev
    }
}

/*
test cases:
[1,2,4]
[1,3,4]
*/
#[cfg(test)]
mod tests {
    use super::Solution;

	#[test]
	fn test_merge_two_sorted_lists() {

		todo!()
	}
}