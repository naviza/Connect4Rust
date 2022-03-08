use std::cell::RefCell;
use std::rc::Rc;
use std::{thread, time};
use std::io;

type Tree = Rc<RefCell<TreeNode<u32>>>;
type AVLTree= Option<Tree>;
// type AVLTree= Option<Rc<RefCell<TreeNode<u32>>>>;

#[derive(Clone,PartialEq)]
struct TreeNode<T> {
    pub data: T,
    pub parent: AVLTree,
    left: AVLTree,
    right: AVLTree,
    height: u64
}

impl<T> TreeNode<T> {
	pub fn new_node(t: T) -> TreeNode<T> {
		TreeNode {
			data: t,
			parent: None,
			left: None,
			right: None,
            height: 1
		}
	}
}

fn count_leaves(tree: &AVLTree) -> u32 {
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

fn new_avl(t: u32, if_root : bool) -> AVLTree {
    if if_root {
        let node = TreeNode::new_node(t);
        let root = Some(Rc::new(RefCell::new(node)));
        return root;
    } else {
        Some(Rc::new(RefCell::new(TreeNode::new_node(t))))
    }
}

fn insert_node(tree: &mut AVLTree, t: u32) -> AVLTree {
	match tree {
		Some(root) => {
			let node;
            root.borrow_mut().height += 1;
			if root.borrow().data > t {
				node = insert_node(&mut root.borrow_mut().left, t);
				root.borrow_mut().left = node;
				match &mut root.borrow_mut().left {
					Some(child) => {
						child.borrow_mut().parent = Some(root.clone());
					},
					None => println!("Error: Failed to insert node")
				};
			} else if root.borrow().data < t {
				node = insert_node(&mut root.borrow_mut().right, t);
				root.borrow_mut().right = node;
				match &mut root.borrow_mut().right {
					Some(child) => {
						child.borrow_mut().parent = Some(root.clone());
					},
					None => println!("Error: Failed to insert node")
				};
			} else {
				// for equal, do nothing
			}
			Some(root.clone())
		}
		None => new_avl(t,false)
	}
}

fn find_node(tree: &mut AVLTree, t: u32) -> AVLTree {
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

fn count_node(tree: &AVLTree) -> u64 {
	match tree {
		None => {0}
		Some(node) => {
			let left:u64 = count_node(&node.borrow().left);
			let right:u64 = count_node(&node.borrow().right);
			1 + left + right
		}
	}
}

fn insert(tree: &mut AVLTree, t: u32) {
	let rtree = &mut insert_node(tree, t);
	let inserted_node = &mut find_node(rtree, t);
	// balance_tree(inserted_node) // does not work properly
}

fn find_node_help(tree: &AVLTree, t: u32) -> AVLTree {
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

fn right_rotate(grandparent_tree : &mut AVLTree) -> AVLTree {
	// make grandparent is right child of the parent. right child of parent is the left child of the grandparent
	match grandparent_tree {
		Some(grandparent) => {
			// let color = grandparent.borrow().color;
			let parent = grandparent.borrow().parent.clone();
			let copy = grandparent.borrow().left.clone();
			match copy {
				Some(left_child) => {
					grandparent.borrow_mut().left = left_child.borrow().right.clone();
					// grandparent.borrow_mut().color = left_child.borrow().color;
					left_child.borrow_mut().parent = parent;
					// left_child.borrow_mut().color = color;
					match &left_child.borrow().right {
						None => {}, // nothing to do if none
						Some(left_right_child) => {
							let mut left_right_holder = left_right_child.borrow_mut();
							left_right_holder.parent = Some(Rc::new(RefCell::new(grandparent.borrow().clone())));
						}
					}
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

fn find_min(tree : &mut AVLTree) -> AVLTree {
	match tree {
		Some(x) => {
			if x.borrow_mut().left.is_none() {
				//x in min
				return Some(x.clone());
			} else {
				return find_min(&mut x.borrow_mut().left.clone());
			}
		},
		None => {
			return None;
		}
	}
}

fn find_max(tree : &mut AVLTree) -> AVLTree {
	match tree {
		Some(x) => {
			if x.borrow_mut().right.is_none() {
				//x in max
				return Some(x.clone());
			} else {
				return find_min(&mut x.borrow_mut().right.clone());
			}
		},
		None => {
			return None;
		}
	}
}

fn delete(tree: &mut AVLTree, t: u32) -> AVLTree{
	delete_helper(tree,t)
	//still have to balance properly
}

fn delete_helper(tree: &mut AVLTree, t: u32) -> AVLTree {
	match tree {
		Some(root) => {
			let mut successor: AVLTree = None;
			if root.borrow().data == t {
				if root.borrow().left.is_none() && root.borrow().right.is_none() {
					return None;
				} else if root.borrow().left.is_none() {
					if let Some(child) = root.borrow().right.clone() {
						child.borrow_mut().parent = root.borrow().parent.clone();
					}
					return root.borrow().right.clone();
				} else if root.borrow().right.is_none() {
					if let Some(child) = root.borrow().left.clone() {
						child.borrow_mut().parent = root.borrow().parent.clone();
					}
					return root.borrow().left.clone();
				} else {
					let maximum = find_max(&mut root.borrow_mut().left);
					if let Some(max) = maximum {
						let temp =  max.borrow().data;
						if let Some(clear) = max.borrow_mut().parent.clone() {
							if clear.borrow().data != root.borrow().data {
								let x = find_node(&mut Some(root.clone()), clear.borrow().data);
								if let Some(y) = x.clone() {
									root.borrow_mut().data = temp;
									y.borrow_mut().right = None;
									y.borrow_mut().parent = Some(root.clone());
								}
							} else {
								root.borrow_mut().data = temp;
								let r = &mut Some(root.clone());
								let x = find_node(r, root.borrow().data);
								if let Some(y) = x.clone() {
									root.borrow_mut().data = temp;
									y.borrow_mut().left = None;
									y.borrow_mut().parent = Some(root.clone());
								}
							}
						}
					}
					return Some(Rc::new(RefCell::new(root.borrow().clone())));
				}
			} else if root.borrow().data > t {
				successor = delete(&mut root.borrow_mut().left.clone(), t);
				root.borrow_mut().left = successor;
			} else {
				successor = delete(&mut root.borrow_mut().right.clone(), t);
				root.borrow_mut().right = successor;
			}
			Some(Rc::new(RefCell::new(root.borrow().clone())))
		}
		None => {
			println!("The number entered was not found in the tree");
			return tree.clone();
		}
	}
}

fn left_rotate(node : &mut AVLTree) -> AVLTree{
	// must make grandparent is right child of the parent. right child of parent is the left child of the grandparent
	match node {
		Some(grandparent) => {
			let parent = grandparent.borrow().parent.clone();
			let copy = grandparent.borrow().right.clone();
			match copy {
				Some(right_child) => {
					grandparent.borrow_mut().right = right_child.borrow().left.clone();
					right_child.borrow_mut().parent = parent;
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

fn print_cmds() {
	println!("\n--------------------------------------------------------------------");
	println!("What do you want to do with the tree?");
	println!("Type i if you want to insert into the tree");
	println!("Type d if you want to delete data from the tree");
	println!("Type c if you want to know how many leaves are in the tree");
	println!("Type h if you want to know the height of the tree");
	println!("Type o if you want to print the tree in-order traversal");
	println!("Type e if you want to check if the tree is empty");
	println!("Type p if you want to print the tree itself with direction");
	println!("Type s if you want to know how many nodes the tree has");
	println!("Type exit to end the program");
	println!("--------------------------------------------------------------------");
	println!("Type your input : ");
}

fn count_height(tree: &AVLTree) -> u32 {
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

fn in_order_transversal(tree: &AVLTree) {
	match tree {
		Some(node) => {
			in_order_transversal(&node.borrow().left);
			println!("data = {}", node.borrow().data);
			in_order_transversal(&node.borrow().right);
		}
		None => println!("Null Leaf"),
	}
}

fn size(tree: &AVLTree) -> u32 {
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

fn print_tree(tree: &AVLTree, indent: usize) {
	match tree {
		Some(leaf) => {
			print!("{:indent$}","",indent = indent);
			println!("data: {},", leaf.borrow().data);
			println!("{:indent$}Left {{","",indent = indent+5);
			print_tree(&leaf.borrow().left,indent + 5);
			print!("\n{:indent$}}}","",indent = indent+5);
			println!("\n{:indent$}Right {{","",indent = indent+5);
			print_tree(&leaf.borrow().right, indent + 5);
			print!("\n{:indent$}}}\n","",indent = indent+5);
		}
		None => {
			 print!("{:indent$}","",indent = indent+5);
			print!("NULL Leaf");
		}
	};
}

fn is_empty(tree: &AVLTree) -> bool {
	match tree {
		Some(_) => false,
		None => true
	}
}

fn run() {
	let mut root : u32 = 0;
	let mut input_string = String::new();
	let mut looping = true;
	let mut tree : AVLTree;
	let mut trimmed = "";

	while looping {
		println!("What do you want your root for a new tree is (Enter a positive integer, or 'exit' to exit the program) - ");
		input_string = ("").to_string();
		io::stdin().read_line(&mut input_string).expect("failed to readline");

		trimmed = input_string.trim();

		match trimmed.parse::<u32>() {
			Ok(i) => {
				root = i;
				looping = false;
			},
			Err(..) => {
				if trimmed == "exit" {
					looping = false;
					println!("Ending the program");
				} else {
					println!("this was not an integer: {}", trimmed)
				}
			},
		};
	}

	
	if trimmed != "exit" {
		println!("Creating a new tree with the root data provided - {} ", root);
		tree = new_avl(root , true);

		while trimmed != "exit" {
			print_cmds();
			input_string = ("").to_string();
			
			io::stdin().read_line(&mut input_string).expect("failed to readline");
	
			trimmed = input_string.trim();
			match trimmed {
				"i" => {
					println!("What value do you want to insert into the tree:");
					input_string = ("").to_string();
			
					io::stdin().read_line(&mut input_string).expect("failed to readline");
			
					trimmed = input_string.trim();
	
					match trimmed.parse::<u32>() {
						Ok(i) => {
							tree = insert_node(&mut tree, i);
						},
						Err(..) => {
							println!("this was not an integer: {}", trimmed)
						},
					};
				},
	
				"d" => {
					println!("What value do you want to delete from the tree:");
					input_string = ("").to_string();
			
					io::stdin().read_line(&mut input_string).expect("failed to readline");
			
					trimmed = input_string.trim();
	
					match trimmed.parse::<u32>() {
						Ok(i) => {
							tree = delete(&mut tree, i);
						},
						Err(..) => {
							println!("this was not an integer: {}", trimmed)
						},
					};
				},
	
				"c" => println!("The amount of leaves this tree has is {}", count_leaves(&tree)),
	
				"h" => println!("The tree has a height of {}",count_height(&tree)),
	
				"o" => {
					println!("Printing in-order transversal...");
					in_order_transversal(&tree);
				},
	
				"e" => {
					println!("Checking if the tree is empty...");
					if is_empty(&tree) {
						println!("The tree is empty");
					} else {
						println!("The tree is not empty");
					}
				},
	
				"p" => {
					println!("Printing the tree...");
					print_tree(&tree, 0);
				},

				"s" => {
					println!("Printing the size of the tree...");
					println!("The tree has {} nodes", size(&tree));
				},
	
				"exit" => println!("Exiting the program"),
	
				_ => println!("The command you entered is not found, please try again")
			}
		}
	}
}

fn main() {
	run();
}
