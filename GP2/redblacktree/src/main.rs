use std::cell::RefCell;
use std::rc::Rc;

#[derive(Copy, Clone, Debug, PartialEq)]
enum NodeColor {
	Red,
	Black,
}

type Tree = Rc<RefCell<TreeNode<u32>>>;
type RedBlackTree= Option<Tree>;

#[derive(Clone,PartialEq)]
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
			color: NodeColor::Red,
			data: t,
			parent: None,
			left: None,
			right: None
		}
	}
}

impl NodeColor {
	fn is_red(self) -> bool{
		match self {
			NodeColor::Black => false,
			NodeColor::Red => true,
		}
	}

	fn flip_color(self) -> Self{
		match self {
			NodeColor::Black => NodeColor::Red,
			NodeColor::Red => NodeColor::Black,
		}
	}
}

fn new_rb(t: u32,if_root : bool) -> RedBlackTree {
	if if_root {
		let node = TreeNode::new_node(t);
		node.color.flip_color();
		let root = Some(Rc::new(RefCell::new(node)));
		return root;
	} else {
		Some(Rc::new(RefCell::new(TreeNode::new_node(t))))
	}
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

fn violations(tree: &mut RedBlackTree) {

}

fn right_rotate() {

}

fn left_rotate() {

}

fn find_parent() {

}

fn find_grandparent() {

}

fn find_uncle() {

}

fn balance_tree(tree: &mut RedBlackTree) -> RedBlackTree{
	match tree {
		Some(node) => {
			match node.borrow_mut().parent.clone() {
				Some(parent) => {
					if parent.borrow_mut().color.is_red() {
						match parent.borrow_mut().parent.clone() {
							Some(grandparent) => {
								if grandparent.borrow_mut().left == node.borrow_mut().parent {
									//check right
									match grandparent.borrow_mut().right.clone() {
										Some(uncle) => {
											if uncle.borrow_mut().color.is_red() {
												//change to black
												uncle.borrow_mut().color.flip_color();
												parent.borrow_mut().color.flip_color();
												grandparent.borrow_mut().color = NodeColor::Red;
												balance_tree(&mut uncle.borrow_mut().parent)
											} else {
												// uncle is black
												
												
												
												None
											}
										},
										None => None
									}
								} else {
									match grandparent.borrow_mut().left.clone() {
										Some(uncle) => {
											if uncle.borrow_mut().color.is_red() {
												//change to black
												uncle.borrow_mut().color.flip_color();
												parent.borrow_mut().color.flip_color();
												grandparent.borrow_mut().color = NodeColor::Red;
												balance_tree(&mut uncle.borrow_mut().parent)
											} else {
												None
											}
										},
										None => None
									}
								}
							},
							None => {
								//parent is root
								None
							}
						}
					} else {
						None
					}
				},
				None => {
					//root
					None
				}
			}  
		},
		None => {
			None
		}
	}
}


fn in_order_transversal(tree: &RedBlackTree) {
	match tree {
		Some(node) => {
			in_order_transversal(&node.borrow().left);
			println!("{}", node.borrow().data);
			in_order_transversal(&node.borrow().right);
		}
		None => println!("Null Leaf"),
	}
}

fn find_node(tree: &mut RedBlackTree, t: u32) -> RedBlackTree {
	match tree {
		Some(root) => {
			if root.borrow().data == t {
				tree.clone()
			}else if root.borrow().data < t{
				find_node(&mut root.borrow_mut().right, t)
			} else {
				find_node(&mut root.borrow_mut().left, t)
			}
		},
		None =>{
			println!("Node does not exist");
			None
		}
	}
}
fn insert(tree: &mut RedBlackTree, t: u32) -> RedBlackTree {
	let rtree = &mut insert_node(tree, t);
	let inserted_node = &mut find_node(rtree, t);
	balance_tree(inserted_node)
}

fn insert_node(tree: &mut RedBlackTree, t: u32) -> RedBlackTree {
	match tree {
		Some(root) => {
			let node;
			if root.borrow().data > t {
				node = insert_node(&mut root.borrow_mut().left, t);
				root.borrow_mut().left = node;
				match &mut root.borrow_mut().left {
					Some(child) => {
						child.borrow_mut().parent = Some(root.clone());
					},
					None => println!("Error: Failed to insert node")
				};
			} else {
				node = insert_node(&mut root.borrow_mut().right, t);
				root.borrow_mut().right = node;
				match &mut root.borrow_mut().right {
					Some(child) => {
						child.borrow_mut().parent = Some(root.clone());
					},
					None => println!("Error: Failed to insert node")
				};
			}
			Some(root.clone())

		}
		None => new_rb(t,false)
	}
}

fn delete(tree: &mut RedBlackTree, t: u32) {
	match tree {
		Some(root) => {
			if root.borrow().data == t {
				
			} else {
				delete(&mut root.borrow_mut().left, t);
				delete(&mut root.borrow_mut().right, t);
			}
		}
		None => {}
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

fn count_height(tree: &RedBlackTree) -> u32 {
	let mut height = 1;
	match tree {
		Some(node) => {
			let left = count_height(&node.borrow().left);
			let right = count_height(&node.borrow().right);
			if left >= right {
				height += left;
			} else {
				height += right;
			}
			height
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
		None => println!("NULL Leaf")
	};
}

fn test_tree() {
	let mut t: RedBlackTree = new_rb(5,true);
	insert(&mut t, 10);
	insert(&mut t, 20);
	println!("Printing Tree");
	print_tree(&t);
	println!();
	println!("Number of nodes: {}", size(&t));
	println!("Number of leaves: {}", count_leaves(&t));
	println!();
	println!("Printing tree in-order transversal");
	in_order_transversal(&t);
	println!();
}

fn main() {
	test_tree();
}
