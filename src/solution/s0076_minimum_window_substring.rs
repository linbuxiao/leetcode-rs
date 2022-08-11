/*
给你一个字符串 s 、一个字符串 t 。返回 s 中&涵盖 t 所有字符的最小子串。如果 s 中不存在涵盖 t 所有字符的子串，则返回空字符串 "" 。

注意：

    对于 t 中重复字符，我们寻找的子字符串中该字符数量必须不少于 t 中该字符数量。
    如果 s 中存在这样的子串，我们保证它是唯一的答案。


示例 1：

输入：s = "ADOBECODEBANC", t = "ABC"
输出："BANC"

示例 2：

输入：s = "a", t = "a"
输出："a"

示例 3:

输入: s = "a", t = "aa"
输出: ""
解释: t 中两个字符 'a' 均应包含在 s 的子串中，
因此没有符合条件的子字符串，返回空字符串。

提示：

    1 <= s.length, t.length <= 10^5
    s 和 t 由英文字母组成


进阶：你能设计一个在 o(n) 时间内解决此问题的算法吗？

link: https://leetcode.cn/problems/minimum-window-substring/
*/
use std::collections::HashMap;
struct Solution {}

impl Solution {
    pub fn min_window_over_time(s: String, t: String) -> String {
        if s.len() < t.len() {
            return "".to_string();
        }
        let mut target_arr: Vec<Vec<usize>> = vec![vec![]; t.len()];
        for (i, k) in s.char_indices() {
            if t.contains(k) {
                for (j, c) in t.char_indices() {
                    if c == k {
                        target_arr[j].push(i);
                    }
                }
            }
        }
        let mut result: Vec<Vec<usize>> = vec![];
        for v in target_arr.clone() {
            if v.len() == 0 {
                return "".to_string();
            }
            if result.len() == 0 {
                for k in &v {
                    result.push(vec![*k])
                }
            } else {
                let mut tmp_result: Vec<Vec<usize>> = vec![];
                for arr in result.iter_mut() {
                    for x in &v {
                        let mut arr_tmp = arr.clone();
                        if arr_tmp.contains(&x) {
                            continue;
                        }
                        arr_tmp.push(*x);
                        tmp_result.push(arr_tmp);
                    }
                }
                result = tmp_result;
            }
        }
        let mut min_len = s.len();
        let mut start = 0;
        let mut end = s.len() - 1;
        let mut flag = false;
        for arr in result.iter_mut() {
            if arr.len() < t.len() {
                continue;
            }
            flag = true;
            arr.sort();
            let tmp_len = arr[arr.len() - 1] - arr[0] + 1;
            if tmp_len < min_len {
                min_len = tmp_len;
                start = arr[0];
                end = arr[arr.len() - 1];
                if min_len == t.len() {
                    break;
                }
            }
        }
        if !flag {
            return "".to_string();
        }
        s[start..end + 1].to_string()
    }

    pub fn min_window(s: String, t: String) -> String {
        let default_value = "".to_string();
        if s.len() < t.len() {
            return default_value;
        }
        let mut left = 0;
        let mut right = 0;
        let mut r = default_value.clone();
        let mut target_map: HashMap<char, usize> = HashMap::new();
        for c in t.chars() {
            match target_map.get(&c) {
                Some(n) => {
                    target_map.insert(c.clone(), n + 1);
                }
                None => {
                    target_map.insert(c.clone(), 1);
                }
            }
        }
        let mut use_map: HashMap<char, usize> = HashMap::new();
        let f = s.as_bytes()[0] as char;
        use_map.insert(f, 1);
        while right < s.len() || left > right {
            let is_meet = {
                let m = use_map.clone();
                let mut result = true;
                for (k, v) in &target_map {
                    let n = m.get(k).unwrap_or(&0);
                    if n < v {
                        result = false;
                        break;
                    }
                }
                result
            };

            if is_meet {
                let new_str = &s[left..right + 1];
                if r.len() > 0 && new_str.len() < r.len() || r.len() == 0 && new_str.len() <= s.len()
                {
                    r = new_str.to_string();
                }
                let left_char = s.as_bytes()[left] as char;
                left = left + 1;
                if let Some(n) = use_map.get(&left_char) {
                    use_map.insert(left_char, n - 1);
                };
            } else {
                right = right + 1;
                if right == s.len() {
                    break;
                }
                let v = s.as_bytes()[right] as char;
                match use_map.get(&v) {
                    Some(n) => {
                        use_map.insert(v, n + 1);
                    }
                    None => {
                        use_map.insert(v, 1);
                    }
                };
            }
        }
        r
    }
}

/*
test cases:
"ADOBECODEBANC"
"ABC"
*/
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_minimum_window_substring() {
        assert_eq!(
            "BANC".to_string(),
            Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string())
        );
        assert_eq!(
            "aa".to_string(),
            Solution::min_window("aa".to_string(), "aa".to_string())
        );
        assert_eq!(
            "abc".to_string(),
            Solution::min_window("abc".to_string(), "bca".to_string())
        );
        assert_eq!(
            "cwae".to_string(),
            Solution::min_window("cabwefgewcwaefgcf".to_string(), "cae".to_string())
        );
        assert_eq!(
            "".to_string(),
            Solution::min_window("babb".to_string(), "baba".to_string())
        );
        assert_eq!(
            "aabcabbcab".to_string(),
            Solution::min_window("aacbaccccaabcabbcab".to_string(), "bcbbacaaab".to_string())
        );
    }
}
