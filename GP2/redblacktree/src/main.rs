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

fn right_rotate(grandparent_tree : &mut RedBlackTree) -> RedBlackTree{
	// make grandparent is right child of the parent. right child of parent is the left child of the grandparent
	match grandparent_tree {
		Some(grandparent) => {
			let parent = grandparent.borrow_mut().parent.clone();
			grandparent.borrow_mut().parent = grandparent.borrow_mut().left.clone();
			match grandparent.borrow_mut().parent.clone() {
				Some(left_child) => {
					grandparent.borrow_mut().left = left_child.borrow_mut().right.clone();
					left_child.borrow_mut().right = left_child.borrow_mut().parent.clone();
					left_child.borrow_mut().parent = parent;
					grandparent.borrow_mut().parent.clone()
				},
				None => panic!("there should be a left child")
			}
		},
		None=> panic!("Should be a grandparent")
	}
}

fn left_rotate(node : &mut RedBlackTree) -> RedBlackTree{
	// must make grandparent is right child of the parent. right child of parent is the left child of the grandparent
	match node {
		Some(grandparent) => {
			let parent = grandparent.borrow_mut().parent.clone();
			grandparent.borrow_mut().parent = grandparent.borrow_mut().right.clone();
			match grandparent.borrow_mut().parent.clone() {
				Some(right_child) => {
					grandparent.borrow_mut().right = right_child.borrow_mut().left.clone();
					right_child.borrow_mut().left = right_child.borrow_mut().parent.clone();
					right_child.borrow_mut().parent = parent;
					grandparent.borrow_mut().parent.clone()
				},
				None => panic!("there should be a right child")
			}
		},
		None=> panic!("Should be a grandparent")
	}
}


fn check_rotation(node : &mut RedBlackTree) -> i32{
	match node.clone() {
		Some(i_node) => {
			let x = i_node.borrow_mut().parent.clone(); 
			match i_node.borrow_mut().parent.clone() {
				Some(ref parent) =>{
					let y = parent.borrow_mut().parent.clone();
					match y.clone() {
						Some(grandparent) => {
							if grandparent.borrow_mut().left == i_node.borrow_mut().parent.clone() {
								if parent.borrow_mut().left == node.clone() {
									return 1;
								} else {
									return 2;
								}
							} else {
								if parent.borrow_mut().right == node.clone() {
									return 3;
								} else {
									return 4;
								}
							}
						},
						None => {
							//inserted node's parent is the the root
							panic!("If there is not grandparent, simple recoloring will work");
						}
					}
				}
				None => {
					// the inserted node's parent is none, meaning inserted node is the root
					// This is not possible as in balance_tree() we check if the inserted node is the root
					panic!("inserted node is the root, which is not possible!");
				}
			}
		},
		None => {
			// means the inserted node is none, which is not possible
			//println!("Something went wrong when inserting the node in the tree");
			panic!("Something went wrong when inserting the node in the tree")
		},
	}
}

fn balance_tree(tree: &mut RedBlackTree) -> RedBlackTree{
	match tree.clone() {
		Some(node) => {
			match node.borrow_mut().parent.clone() {
				Some(parent) => {
					if parent.borrow().color.is_red() {
						match parent.borrow_mut().parent.clone() {
							Some(grandparent) => {
								if grandparent.borrow_mut().left == node.borrow_mut().parent.clone() {
									//check right
									if let Some(uncle) = grandparent.borrow_mut().right.clone() {
										if uncle.borrow_mut().color.is_red() {
											//change to black
											uncle.borrow_mut().color.flip_color();
											parent.borrow_mut().color.flip_color();
											grandparent.borrow_mut().color = NodeColor::Red;
											return balance_tree(&mut uncle.borrow_mut().parent); // FIX
										}
									} 
								} else {
									if let Some(uncle) = grandparent.borrow_mut().left.clone() {
										if uncle.borrow_mut().color.is_red() {
											//change to black
											uncle.borrow_mut().color.flip_color();
											parent.borrow_mut().color.flip_color();
											grandparent.borrow_mut().color = NodeColor::Red;
											return balance_tree(&mut uncle.borrow_mut().parent); // FIX
										}
									}
								}
								let rotation = check_rotation(tree);

								let mut new_tree : RedBlackTree;
								match rotation {
									//left left = 1 right rotate
									1=> new_tree = right_rotate(&mut parent.borrow_mut().parent),
									
									2 => {
										// left right
										new_tree = left_rotate(&mut node.borrow_mut().parent);
										if let Some(i) = new_tree {
											new_tree = right_rotate(&mut i.borrow_mut().parent);
										}
									},
									// right right = 1 left rotate
									3 => new_tree = left_rotate(&mut parent.borrow_mut().parent),
									4 => {
										// right left
										new_tree = right_rotate(&mut node.borrow_mut().parent);
										if let Some(i) = new_tree {
											new_tree = left_rotate(&mut i.borrow_mut().parent);
										}
									}
									_ => panic!("Error in checking rotations")
								}
								balance_tree(&mut new_tree)
							},
							None => {
								//parent is root
								parent.borrow_mut().color.flip_color();
								//panic!("Should not get here");
								return node.borrow().parent.clone();
							}
						}
					} else {
						tree.clone()
					}
				},
				None => {
					//tree is the root
					node.borrow_mut().color = NodeColor::Black;
					tree.clone()
				}
			}  
		},
		None => {
			// inserted Node is none means there was an error in adding element into the tree
			panic!("Error in inserting the node into the tree")
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
