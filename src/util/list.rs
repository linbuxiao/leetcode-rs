#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T> {
  pub val: T,
  pub next: Option<Box<ListNode<T>>>
}

impl<T> ListNode<T> {
  #[inline]
  fn new(val: T) -> Self {
    ListNode {
      next: None,
      val
    }
  }

  fn get_last(&mut self) -> &mut Self {
    if let Some(ref mut x) = self.next {
        return x.get_last()
    }
    self
  }

  fn set_next(&mut self, node: Self) {
    self.next = Some(Box::new(node));
  }

  fn push(&mut self, val: T) {
    let new_node = ListNode::new(val);
    self.get_last().set_next(new_node);
  }
}

pub fn create_by_vec_i32<T: Copy>(arr: Vec<T>) -> Option<Box<ListNode<T>>> {
    if arr.len() == 0 {
        return None
    }
    let mut iter = arr.iter();
    let mut list = ListNode::new(*iter.next().unwrap());
    while let Some(v) = iter.next() {
        list.push(*v);
    };
    Some(Box::new(list))
  }

mod tests {
    use super::*;

    #[test]
    fn test_create_by_vec_i32() {
        let arr = vec![1,2,2,1];
        let list = create_by_vec_i32(arr);
        print!("{:?}", list);
    }
}