#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl ListNode {
	pub fn remove_next(&mut self) {
		if let Some(node) = &self.next {
			self.next = node.next.to_owned()
		} else {
			self.next = None
		}
	}

	pub fn get_last(&mut self) -> &mut Self {
		if let Some(ref mut x) = self.next {
			return x.get_last()
		}
		return self
	}

	pub fn set_next(&mut self, node: Self) {
		self.next = Some(Box::new(node));
	}

	pub fn push(&mut self, val: i32) {
		let new_node = Self::new(val);
		self.get_last().set_next(new_node);
	}
}