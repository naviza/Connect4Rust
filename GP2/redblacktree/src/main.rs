use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
}

type Tree = Rc<RefCell<TreeNode<u32>>>;
type RedBlackTree= Option<Tree>;

struct TreeNode<T> {
    pub color: NodeColor,
    pub data: T,
    pub parent: RedBlackTree,
    left: RedBlackTree,
    right: RedBlackTree,
}

impl<T> TreeNode<T> {
	
	pub fn new_node(t: T) -> TreeNode<T> {
		TreeNode {
			color: NodeColor::Black,
			data: t,
			parent: None,
			left: None,
			right: None
		}
	}
}

fn new_rb(t: u32) -> RedBlackTree {
	Some(Rc::new(RefCell::new(TreeNode::new_node(t))))
}

//fn assign(parent: &mut RedBlackTree, child: &mut RedBlackTree) {
//	match child {
//		Some(node) => node.borrow_mut().left = parent.clone(),
//		None => println!("Error: Failed to assign parent to child")
//	};
//	match parent {
//		Some(node) => node.borrow_mut().left = child.clone(),
//		None => println!("Error: Failed to assign child to parent")
//	}
//}

fn rebalance(tree: &mut RedBlackTree) {
	match tree {
		Some(node) => {
			
			
			rebalance(&mut node.borrow_mut().left);
			rebalance(&mut node.borrow_mut().right);
		}
		None => {}
	};
}

fn insert(tree: &mut RedBlackTree, t: u32) -> RedBlackTree {
		match tree {
			Some(root) => {
				let node = insert(&mut root.borrow_mut().left, t);
				root.borrow_mut().left = node;
				match &mut root.borrow_mut().left {
					Some(child) => child.borrow_mut().parent = Some(root.clone()),
					None => println!("Error: Failed to insert node")
				};
				Some(root.clone())
			}
			None => new_rb(t)
		}
}

fn delete(tree: &mut RedBlackTree, t: u32) {
	match tree {
		Some(root) => {
			
		}
		None =>{}
	}
}

fn is_empty(tree: &RedBlackTree) -> bool {
	match tree {
		Some(_) => false,
		None => true
	}
}

fn size(tree: &RedBlackTree) -> u32 {
	match tree {
		Some(node) => {
			let mut count = 1;
			count += size(&node.borrow().left);
			count += size(&node.borrow().right);
			count
		}
		None => 0
	}
}

fn count_leaves(tree: &RedBlackTree) -> u32 {
	let mut count = 0;
	match tree {
		Some(node) => {
			let right = count_leaves(&node.borrow().right);
			let left = count_leaves(&node.borrow().left);
			if right == 0 && left == 0 { count = 1; }
			if right > 0 { count += right; }
			if left > 0 { count += left; }
			count
		}
		None => 0
	}
}

fn print_tree(tree: &RedBlackTree) {
	match tree {
		Some(leaf) => {
			println!("data: {}", leaf.borrow().data);
			print_tree(&leaf.borrow().left);
			print_tree(&leaf.borrow().right);
		}
		None => println!("End of Tree")
	};
}

fn main() {
    let mut t: RedBlackTree = new_rb(5);
    insert(&mut t, 10);
    insert(&mut t, 20);
    print_tree(&t);
    println!("Number of nodes: {}", size(&t));
    println!("Number of leaves: {}", count_leaves(&t));
}
