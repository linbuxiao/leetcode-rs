/*
给定一个经过编码的字符串，返回它解码后的&字符串。
编码规则为: k[encoded_string]，表示其中方括号内部的 encoded_string 正好重复 k 次。注意 k 保证为正整数。
你可以认为输入字符串总是有效的；输入字符串中没有额外的空格，且输入的方括号总是符合格式要求的。
此外，你可以认为原始数据不包含数字，所有的数字只表示重复的次数 k ，例如不会出现像 3a 或 2[4] 的输入。
 
示例 1：

输入：s = "3[a]2[bc]"
输出："aaabcbc"

示例 2：

输入：s = "3[a2[c]]"
输出："accaccacc"

示例 3：

输入：s = "2[abc]3[cd]ef"
输出："abcabccdcdcdef"

示例 4：

输入：s = "abc3[cd]xyz"
输出："abccdcdcdxyz"

 
提示：

	1 <= s.length <= 30
	<meta charset="UTF-8" />s 由小写英文字母、数字和方括号<meta charset="UTF-8" /> '[]' 组成
	s 保证是一个 有效 的输入。
	s 中所有整数的取值范围为<meta charset="UTF-8" /> [1, 300] 


link: https://leetcode.cn/problems/decode-string/
*/

use std::time;

struct Solution {}

impl Solution {
    pub fn decode_string(s: String) -> String {
		let mut p = String::new();
		let mut next_index: usize = 0;
		for (i, c) in s.char_indices() {
			if i < next_index { continue; }
			if c.is_ascii_digit() {
				let mut number_index = i;
				let mut cc = c;
				let mut number_string = String::new();
				while cc.is_ascii_digit() {
					let cc_string = cc.to_string();
					number_string = number_string + &cc_string;
					number_index = number_index + 1;
					cc = s.as_bytes()[number_index] as char;
				}
				let times = number_string.to_string().parse::<usize>().unwrap();
				// 此时为数字，接着会出现 '[]'，找到这个范围并再次传递给下一个函数
				let start = number_index + 1;
				let end = Self::get_end(s.clone(), start);
				let mut next_str = Self::cut_str(s.clone(), start, end);
				// next_str = (&next_str.strip_prefix("[").unwrap()).to_string();
				// next_str = (&next_str.strip_suffix("]").unwrap()).to_string();
				let next_result = Self::decode_string(next_str);
				for _ in 0..times {
					p = p + &next_result;
				}
				next_index = end + 1;
			} else {
				let s = c.to_string();
				p = p + &s;
			}
		}
		p
    }

	pub fn get_end(s: String, i: usize) -> usize {
		let mut times = 1;
		let start = i + 1;
		let b = s.as_bytes();
		for i in start..b.len() {
			let c = b[i] as char;
			let x = c.to_string();
			let y = x.as_str();
			if y == "[" {
				times = times + 1;
			} else if y == "]" {
				times = times - 1;
				if times == 0 {
					return i
				}
			}
		}
		0
	}

	pub fn cut_str(s: String, start: usize, end: usize) -> String {
		let b = s.as_bytes();
		let new_b = &b[start..end];
		let mut result = String::new();
		for i in 0..new_b.len() {
			let c = new_b[i] as char;
			let x = c.to_string();
			result = result + &x;
		}
		result
	}
}

/*
test cases:
"3[a]2[bc]"
*/
#[cfg(test)]
mod tests {
    use super::Solution;

	#[test]
	fn test_decode_string() {
		let result = Solution::decode_string("3[a]2[bc]".to_string());
		assert_eq!(result, "aaabcbc".to_string());
		let result_1 = Solution::decode_string("100[leetcode]".to_string());
		assert_eq!(result_1, "leetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcode".to_string())
	}
}