/*
设计一个支持 push ，pop ，top 操作，并能在常数时间内检索到最小元素的栈。
实现 MinStack 类:

	MinStack() 初始化堆栈对象。
	void push(int val) 将元素val推入堆栈。
	void pop() 删除堆栈顶部的元素。
	int top() 获取堆栈顶部的元素。
	int getMin() 获取堆栈中的最小元素。

 
示例 1:

输入：
["MinStack","push","push","push","getMin","pop","top","getMin"]
[[],[-2],[0],[-3],[],[],[],[]]
输出：
[null,null,null,null,-3,null,0,-2]
解释：
MinStack minStack = new MinStack();
minStack.push(-2);
minStack.push(0);
minStack.push(-3);
minStack.getMin();   --> 返回 -3.
minStack.pop();
minStack.top();      --> 返回 0.
minStack.getMin();   --> 返回 -2.

 
提示：

	-2^31 <= val <= 2^31 - 1
	pop、top 和 getMin 操作总是在 非空栈 上调用
	push, pop, top, and getMin最多被调用 3 * 10^4 次


link: https://leetcode.cn/problems/min-stack/
*/

struct Solution {}

struct MinStack {
	min_stack: Vec<i32>,
	data: Vec<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
		Self { min_stack: Vec::new(), data: Vec::new() }
    }
    
    fn push(&mut self, val: i32) {
		self.data.push(val);
		if self.min_stack.is_empty() || val <= self.get_min() {
			self.min_stack.push(val);
		}
    }
    
    fn pop(&mut self) {
		let d = self.data.pop();
		if let Some(val) = d {
			if val == self.get_min() {
				self.min_stack.pop();
			}
		} 
    }
    
    fn top(&self) -> i32 {
		let len = self.data.len();
		self.data[len - 1]
    }
    
    fn get_min(&self) -> i32 {
		let len = self.min_stack.len();
		self.min_stack[len - 1]
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

/*
test cases:
["MinStack","push","push","push","getMin","pop","top","getMin"]
[[],[-2],[0],[-3],[],[],[],[]]
*/
#[cfg(test)]
mod tests {
    use super::Solution;

	#[test]
	fn test_min_stack() {

		todo!()
	}
}