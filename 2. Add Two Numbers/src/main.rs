#[derive(PartialEq, Eq)]
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

impl std::fmt::Debug for ListNode {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.val)?;
		let mut iter = &self.next;
		while let Some(cur_node) = iter {
			write!(f, " -> ")?;
			write!(f, "{}", cur_node.val)?;
			iter = &cur_node.next;
		}
		Ok(())
	}
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
	let mut iter1 = l1.unwrap();
	let mut iter2 = l2.unwrap();
    let mut carry = 0i32;
    let mut root = Box::new(ListNode::new(0));

    let (new_node, new_carry) = make_node(iter1.val, iter2.val, carry);
    carry = new_carry;
    root.next.get_or_insert(new_node);

    let mut cur_node_ref = &mut root.next;

    while iter1.next.is_some() || iter2.next.is_some() || carry > 0 {
    	if let Some(cur_node) = cur_node_ref {
			iter1 = iter1.next.or(Some(Box::new(ListNode::new(0)))).unwrap();
			iter2 = iter2.next.or(Some(Box::new(ListNode::new(0)))).unwrap();
		    let (new_node, new_carry) = make_node(iter1.val, iter2.val, carry);
		    carry = new_carry;
		    cur_node.next.get_or_insert(new_node);
		    cur_node_ref = &mut cur_node.next;
    	}
    }

    root.next
}

fn make_node(num1: i32, num2: i32, carry: i32) -> (Box<ListNode>, i32) {
	let (sum, new_carry) = add_with_carry(num1, num2, carry);
	(Box::new(ListNode::new(sum)), new_carry)
}

fn add_with_carry(num1: i32, num2: i32, carry: i32) -> (i32, i32) {
	let mut sum = num1 + num2 + carry;
	let mut new_carry = 0i32;
	if sum >= 10 {
		sum -= 10;
		new_carry += 1;
	}
	(sum, new_carry)
}

fn main() {
	let l1: Option<Box<ListNode>>;
	{
		let node13 = Box::new(ListNode::new(3));
		let mut node12 = Box::new(ListNode::new(4));
		let mut node11 = Box::new(ListNode::new(2));
		node12.next = Some(node13);
		node11.next = Some(node12);
		l1 = Some(node11);
	}

	let l2: Option<Box<ListNode>>;
	{
		let node23 = Box::new(ListNode::new(4));
		let mut node22 = Box::new(ListNode::new(6));
		let mut node21 = Box::new(ListNode::new(5));
		node22.next = Some(node23);
		node21.next = Some(node22);
		l2 = Some(node21);
	}

    println!("l1 = {:?}", l1);
    println!("l2 = {:?}", l2);
    println!("sum = {:?}", add_two_numbers(l1, l2));
    println!("5 + 5 = {:?}", add_two_numbers(Some(Box::new(ListNode::new(5))), Some(Box::new(ListNode::new(5)))));
}
