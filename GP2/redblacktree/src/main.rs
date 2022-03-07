use std::cell::RefCell;
use std::rc::Rc;

#[derive(Copy, Clone, Debug, PartialEq)]
enum NodeColor {
	Red,
	Black,
	DoubleBlack,
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
	fn is_black(self) -> bool {
		match self {
			NodeColor::Black => true,
			_ => false,
		}
	}
	
	fn is_red(self) -> bool{
		match self {
			NodeColor::Red => true,
			_ => false,
		}
	}
	
	fn is_doubleblack(self) -> bool {
		match self {
			NodeColor::DoubleBlack => true,
			_ => false
		}
	}
}

fn new_rb(t: u32,if_root : bool) -> RedBlackTree {
	if if_root {
		let mut node = TreeNode::new_node(t);
		node.color = NodeColor::Black;
		Some(Rc::new(RefCell::new(node)))
	} else {
		Some(Rc::new(RefCell::new(TreeNode::new_node(t))))
	}
}

fn right_rotate(grandparent_tree : &mut RedBlackTree) -> RedBlackTree {
	// make grandparent is right child of the parent. right child of parent is the left child of the grandparent
	match grandparent_tree {
		Some(grandparent) => {
			let color = grandparent.borrow().color;
			let parent = grandparent.borrow().parent.clone();
			let copy = grandparent.borrow().left.clone();
			match copy {
				Some(left_child) => {
					match &left_child.borrow_mut().right {
						Some(x) =>{
							x.borrow_mut().parent = left_child.borrow().parent.clone(); // currently the grandparent
						},
						None => {},
					}

					grandparent.borrow_mut().left = left_child.borrow().right.clone();
				
					grandparent.borrow_mut().color = left_child.borrow().color;
					left_child.borrow_mut().parent = parent;
					left_child.borrow_mut().color = color;
					grandparent.borrow_mut().parent = Some(Rc::new(RefCell::new(left_child.borrow().clone())));
					left_child.borrow_mut().right = Some(Rc::new(RefCell::new(grandparent.borrow().clone())));
					if let Some(value) = left_child.borrow().parent.clone() { value.borrow_mut().right = Some(Rc::new(RefCell::new(left_child.borrow().clone()))); }
					Some(Rc::new(RefCell::new(left_child.borrow().clone())))
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
			let color = grandparent.borrow().color;
			let parent = grandparent.borrow().parent.clone();
			let copy = grandparent.borrow().right.clone();
			match copy {
				Some(right_child) => {
					match &right_child.borrow_mut().left {
						Some(x) =>{
							x.borrow_mut().parent = right_child.borrow().parent.clone(); // currently the grandparent
						},
						None => {},
					}
					grandparent.borrow_mut().right = right_child.borrow().left.clone();
					grandparent.borrow_mut().color = right_child.borrow().color;
					right_child.borrow_mut().parent = parent;
					right_child.borrow_mut().color = color;
					grandparent.borrow_mut().parent = Some(Rc::new(RefCell::new(right_child.borrow().clone())));
					right_child.borrow_mut().left = Some(Rc::new(RefCell::new(grandparent.borrow().clone())));
					if let Some(value) = right_child.borrow().parent.clone() { value.borrow_mut().left = Some(Rc::new(RefCell::new(right_child.borrow().clone()))); }
					Some(Rc::new(RefCell::new(right_child.borrow().clone())))
				},
				None => panic!("there should be a right child")
			}
		},
		None=> panic!("Should be a grandparent")
	}
}


fn check_rotation(node : &mut RedBlackTree) -> (i32, i32) {
	let mut rotations: (i32, i32) = (0, 0);
	match node.clone() {
		Some(i_node) => {
			let x = i_node.borrow().parent.clone();
			match x {
				Some(parent) =>{
					let y = parent.borrow().parent.clone();
					match y {
						Some(grandparent) => {
							if let Some(value) = grandparent.borrow().left.clone() {
								if value.borrow().data == parent.borrow().data { rotations.0 = 1; }
							};
							if let Some(value) = grandparent.borrow().right.clone() {
								if value.borrow().data == parent.borrow().data { rotations.0 = 2; }
							}
							if let Some(value) = parent.borrow().left.clone() {
								if value.borrow().data == i_node.borrow().data { rotations.1 = 1; }
							}
							if let Some(value) = parent.borrow().right.clone() {
								if value.borrow().data == i_node.borrow().data { rotations.1 = 2; }
							}
							rotations
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
			let mut node_copy = node.borrow().parent.clone();
			match node_copy {
				Some(ref parent) => {
					if !parent.borrow().color.is_red() {
						return balance_tree(&mut node_copy);
					}
					let parent_copy = parent.borrow().parent.clone();
					match parent_copy {
						Some(grandparent) => {
							let mut side = 0;
							if let Some(value) = grandparent.borrow().left.clone() {
								if value.borrow().data == parent.borrow().data { side = 1; }
							};
							if let Some(value) = grandparent.borrow().right.clone() {
								if value.borrow().data == parent.borrow().data { side = 2; }
							};
							
							let grandparent_copy: RedBlackTree;
							if side == 1 { grandparent_copy = grandparent.borrow().right.clone(); } 
							else if side == 2 { grandparent_copy = grandparent.borrow().left.clone(); }
							else { grandparent_copy = None };
								
							if let Some(uncle) = grandparent_copy {
								if uncle.borrow().color.is_red() {
									uncle.borrow_mut().color = NodeColor::Black;
									parent.borrow_mut().color = NodeColor::Black;
									grandparent.borrow_mut().color = NodeColor::Red;
									return balance_tree(&mut uncle.borrow_mut().parent);
								}
							}

							let rotation = check_rotation(&mut tree.clone());

							let mut new_tree : RedBlackTree;
							match rotation {
								//left left = 1 right rotate
								(1, 1) => {
									println!("left left");
									// TODO Fix this
									if let Some(sib) = parent.borrow().right.clone() { sib.borrow_mut().parent = parent.borrow().parent.clone(); }
									new_tree = parent.borrow().parent.clone(); 
									new_tree = right_rotate(&mut new_tree);
								}
								(1, 2) => {
									// left right
									println!("left right");
									// TODO Fix this
									//new_tree = node.borrow().parent.clone();
									//if let Some(child) = node.borrow().left.clone() { new_tree = child.borrow_mut().left.clone(); 
									new_tree = node.borrow().parent.clone();
									new_tree = left_rotate(&mut new_tree);
									if let Some(i) = new_tree {
										new_tree = right_rotate(&mut i.borrow().parent.clone());
									}
								},
								// right right = 1 left rotate
								(2, 2) => {
									println!("right right");
									// TODO FIX this
									if let Some(sib) = parent.borrow().left.clone() { sib.borrow_mut().parent = parent.borrow().parent.clone(); }
									new_tree = parent.borrow().parent.clone();
									new_tree = left_rotate(&mut new_tree);
								},
								(2, 1) => {
									// right left
									println!("right left");
									// TODO FIX this
									//new_tree = node.borrow().parent.clone();
									//if let Some(child) = node.borrow().right.clone() { child.borrow_mut().parent = node.borrow().parent.clone(); }
									new_tree = node.borrow().parent.clone();
									new_tree = right_rotate(&mut new_tree);
									if let Some(i) = new_tree {
										new_tree = left_rotate(&mut i.borrow().parent.clone());
									}
								}
								_ => panic!("Error in checking rotations")
							}
							find_root(&mut new_tree)
						},
						None => {
							//parent is root
							//panic!("Should not get here");
							balance_tree(&mut node.borrow().parent.clone())
						}
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

fn find_root(tree: &mut RedBlackTree) -> RedBlackTree {
	match tree{
		Some(node) => {
			if node.borrow().parent.is_none() { tree.clone() } else { find_root(&mut node.borrow().parent.clone()) }
		},
		None => panic!("No root found")
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
					None => panic!("Error: Failed to insert node")
				};
			} else {
				node = insert_node(&mut root.borrow_mut().right, t);
				root.borrow_mut().right = node;
				match &mut root.borrow_mut().right {
					Some(child) => {
						child.borrow_mut().parent = Some(root.clone());
					},
					None => panic!("Error: Failed to insert node")
				};
			}
			Some(root.clone())

		}
		None => new_rb(t,false)
	}
}

fn doubleblack_fix(tree: &mut RedBlackTree, fix: &mut RedBlackTree, side: i32) -> RedBlackTree {
	match tree {
		Some(node) => {
			println!("--");
			print_tree(&node.borrow().parent,0);
			println!("--");
			let mut node_copy: RedBlackTree = None;
			if side == 2 { let node_copy = node.borrow().left.clone(); } else if side == 1 { let node_copy = node.borrow().right.clone(); }
			if let Some(value) = node_copy { value.borrow_mut().color = NodeColor::Black; }
			
			node_copy = None;
			if side == 2 { let node_copy = node.borrow().right.clone(); } else if side == 1 { let node_copy = node.borrow().left.clone(); }
			match node_copy {
				Some(sib) => {
					let mut new_tree: RedBlackTree = None;
					if sib.borrow().color.is_red() {
						println!("Here1");
						node.borrow_mut().color = NodeColor::Red;
						sib.borrow_mut().color = NodeColor::Black;
						new_tree = sib.borrow().parent.clone();
						if side == 2 {
							new_tree = left_rotate(&mut new_tree);
							if let Some(child) = new_tree {
								if let Some(grandchild) = child.borrow().left.clone() { return doubleblack_fix(&mut child.borrow().left.clone(), &mut grandchild.borrow().left.clone(), side); }
							}
						} else if side == 1 {
							new_tree = right_rotate(&mut new_tree);
							if let Some(child) = new_tree {
								if let Some(grandchild) = child.borrow().right.clone() { return doubleblack_fix(&mut child.borrow().right.clone(), &mut grandchild.borrow().right.clone(), side); }
							}
						}
					}
					
					let mut rotations: (i32, i32) = (side, 0);
					if let Some(right_child) = sib.borrow().right.clone() { if right_child.borrow().color.is_red() { rotations.1 = 2; }}
					if let Some(left_child) = sib.borrow().left.clone() { if left_child.borrow().color.is_red() { rotations.1 = 1; }}
					match rotations {
						(1, 1) => {
							println!("Here2.1");
							new_tree = sib.borrow().parent.clone(); 
							new_tree = right_rotate(&mut new_tree);
						},
						(1, 2) => {
							println!("Here2.2");
							new_tree = node.borrow().left.clone();
							new_tree = left_rotate(&mut new_tree);
							if let Some(i) = new_tree {
								new_tree = right_rotate(&mut i.borrow().parent.clone());
							}
						},
						(2, 1) => {
							println!("Here2.3");
							new_tree = node.borrow().right.clone();
							new_tree = right_rotate(&mut new_tree);
							if let Some(i) = new_tree {
								new_tree = left_rotate(&mut i.borrow().parent.clone());
							}
						},
						(2, 2) => {
							println!("Here2.4");
							new_tree = sib.borrow().parent.clone();
							new_tree = left_rotate(&mut new_tree);
						},
						(_, 0) => {
							println!("Here3");
							sib.borrow_mut().color = NodeColor::Red;
							if node.borrow().color.is_black() { 
								return doubleblack_fix(&mut node.borrow().parent.clone(), &mut Some(Rc::new(RefCell::new(node.borrow().clone()))), side); 
							} else { 
								node.borrow_mut().color = NodeColor::Black;
								return tree.clone();
							}
						},
						_ => { return panic!("Unexpected scenario") },
					};
					return new_tree;
				},
				None => {
					let db_node: RedBlackTree = node.borrow().parent.clone();
					let mut child = 0;
					if let Some(parent) = db_node {
						if node.borrow().data < parent.borrow().data { child = 2; } else if node.borrow().data > parent.borrow().data { child = 1; }
					}
					println!("TEST");
					println!("Value is: {:?}", node.borrow().data);
					return doubleblack_fix(&mut node.borrow().parent.clone(), &mut Some(Rc::new(RefCell::new(node.borrow().clone()))), child);
				},
			};
			
		},
		None =>{//child is the root
			match fix {
				Some(root) => {
					println!("ROOT");
					root.borrow_mut().color = NodeColor::Black;
					fix.clone()
				},
				None => panic!("Entire tree deleted")
			}
		}
	}
}

fn delete(tree: &mut RedBlackTree, t: u32) -> RedBlackTree {
	match tree {
		Some(root) => {
			let mut successor: RedBlackTree = None;
			if root.borrow().data == t {
				if root.borrow().left.is_none() && root.borrow().right.is_none() {
					return None;
				} else if root.borrow().left.is_none() {
					if let Some(child) = root.borrow().right.clone() {
						if !child.borrow().color.is_red() || !root.borrow().color.is_red() {
							child.borrow_mut().color = NodeColor::Black;
						} else {
							child.borrow_mut().color = NodeColor::DoubleBlack;
						}
						child.borrow_mut().parent = root.borrow().parent.clone();
					}
					return root.borrow().right.clone();
				} else {
					if let Some(child) = root.borrow().right.clone() {
						if !child.borrow().color.is_red() || !root.borrow().color.is_red() {
							child.borrow_mut().color = NodeColor::Black;
						} else {
							child.borrow_mut().color = NodeColor::DoubleBlack;
						}
						child.borrow_mut().parent = root.borrow().parent.clone();
					}
					return root.borrow().left.clone();
				}
			} else if root.borrow().data > t {
				successor = delete(&mut root.borrow_mut().left.clone(), t);
				root.borrow_mut().left = successor;
				match root.borrow().left.clone() {
					Some(x) if x.borrow().color.is_doubleblack() => return doubleblack_fix(&mut x.borrow().parent.clone(), &mut Some(Rc::new(RefCell::new(x.borrow().clone()))), 2),
					None => return doubleblack_fix(&mut Some(Rc::new(RefCell::new(root.borrow().clone()))), &mut None, 2),
					Some(_) => {},
				}
			} else {
				successor = delete(&mut root.borrow_mut().right.clone(), t);
				root.borrow_mut().right = successor;
				match root.borrow().right.clone() {
					Some(x) if x.borrow().color.is_doubleblack() => return doubleblack_fix(&mut x.borrow().parent.clone(), &mut Some(Rc::new(RefCell::new(x.borrow().clone()))), 1),
					None => return doubleblack_fix(&mut Some(Rc::new(RefCell::new(root.borrow().clone()))), &mut None, 1),
					Some(_) => {},
				};
			}
			Some(Rc::new(RefCell::new(root.borrow().clone())))
		}
		None => tree.clone()
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

fn print_tree(tree: &RedBlackTree,indent: usize) {
	match tree {
		Some(leaf) => {
			print!("{:indent$}","",indent = indent);
			println!("data: {}, color: {:?},", leaf.borrow().data, leaf.borrow().color);
			print!("{:indent$}Left{{\n","",indent = indent+5);
			print_tree(&leaf.borrow().left,indent + 5);
			print!("}}, \n{:indent$}Right{{\n","",indent = indent+5);
			print_tree(&leaf.borrow().right, indent + 5);
			print!("}}");
		}
		None => {
			 print!("{:indent$}","",indent = indent+5);
			print!("NULL Leaf");
		}
	};
}

fn test_tree() {
	println!("Start");
	let mut t: RedBlackTree = new_rb(21,true);
	println!("Tree created");
	t = insert(&mut t, 32);
	t = insert(&mut t, 3);
	t = insert(&mut t, 35);
	t = insert(&mut t, 18);
	t = insert(&mut t, 2);
	t = insert(&mut t, 8);
	t = insert(&mut t, 6);
	t = insert(&mut t, 19);
	//t = delete(&mut t, 19);
	println!("Printing Tree");
	print_tree(&t,0);
	//println!();
	//println!("Number of nodes: {}", size(&t));
	//println!("Number of leaves: {}", count_leaves(&t));
	//println!();
	//println!("Printing tree in-order transversal");
	//in_order_transversal(&t);
	println!();
}

fn main() {
	test_tree();
}