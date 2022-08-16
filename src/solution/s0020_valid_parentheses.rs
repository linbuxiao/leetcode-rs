/*
给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串 s ，判断字符串是否有效。
有效字符串需满足：
<ol>
	左括号必须用相同类型的右括号闭合。
	左括号必须以正确的顺序闭合。
</ol>
 
示例 1：

输入：s = "()"
输出：true

示例 2：

输入：s = "()[]{}"
输出：true

示例 3：

输入：s = "(]"
输出：false

示例 4：

输入：s = "([)]"
输出：false

示例 5：

输入：s = "{[]}"
输出：true
 
提示：

	1 <= s.length <= 10^4
	s 仅由括号 '()[]{}' 组成


link: https://leetcode.cn/problems/valid-parentheses/
*/

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
		let mut m = HashMap::from([
			(")", "("),
			("}", "{"),
			("]", "["),
		]);
		let mut list = vec![];
		for c in s.chars() {
			let w = c.to_string();
			let s = w.as_str();
			if s == "(" || s == "{" || s == "[" {
				list.push(w);
			} else {
				let p = m.get(s).unwrap().clone();
				match list.pop() {
					Some(v) => {
						if v.as_str() != p {
							return false;
						}
					}
					None => {
						return false ;
					}
				}
			}
		}
		if list.len() > 0 {
			return false ;
		}
		true
    }
}

/*
test cases:
"()"
*/
#[cfg(test)]
mod tests {
    use super::Solution;

	#[test]
	fn test_valid_parentheses() {

		todo!()
	}
}