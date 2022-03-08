use std::cell::RefCell;
use std::rc::Rc;
use std::{thread, time};
use std::io;

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
	fn is_black(self) -> bool {
		match self {
			NodeColor::Black => true,
			NodeColor::Red => false,
		}
	}
	
	fn is_red(self) -> bool{
		match self {
			NodeColor::Black => false,
			NodeColor::Red => true,
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

/*
	Find functions
*/
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

fn find_min(tree : &mut RedBlackTree) -> RedBlackTree {
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

fn find_max(tree : &mut RedBlackTree) -> RedBlackTree {
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

/*
	Balancing Functions
*/
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
									if side == 1 { grandparent.borrow_mut().left = Some(Rc::new(RefCell::new(parent.borrow().clone()))); } else { grandparent.borrow_mut().right = Some(Rc::new(RefCell::new(parent.borrow().clone()))); }
									grandparent.borrow_mut().color = NodeColor::Red;
									let mut temp = uncle.borrow().parent.clone();
									return balance_tree(&mut temp);
								}
							}
						},
						None => {
							//parent is root
							//panic!("Should not get here");
							let mut temp = node.borrow().parent.clone();
							return balance_tree(&mut temp);
						}
					}
					
				},
				None => {
					//tree is the root
					node.borrow_mut().color = NodeColor::Black;
					let temp = tree.clone();
					return temp;
				}
			}
		},
		None => {
			// inserted Node is none means there was an error in adding element into the tree
			panic!("Error in inserting the node into the tree");
		}
	};
	
	let rotation = check_rotation(&mut tree.clone());
	let mut new_tree : RedBlackTree = None;
	match rotation {
		//left left = 1 right rotate
		(1, 1) => {
			if let Some(node) = tree.clone() {
				if let Some(parent) = node.borrow().parent.clone() {
					if let Some(sib) = parent.borrow().right.clone() {sib.borrow_mut().parent = parent.borrow().parent.clone(); }
				}
				if let Some(parent) = node.borrow().parent.clone() { new_tree = parent.borrow().parent.clone(); } 
			}
			new_tree = right_rotate(&mut new_tree);
		}
		(1, 2) => {
			let mut temp: RedBlackTree = None;
			if let Some(node) = tree {
				if let Some(child) = node.borrow().left.clone() {
					child.borrow_mut().parent = node.borrow().parent.clone();
					temp = Some(Rc::new(RefCell::new(child.borrow().clone())));
				}
				node.borrow_mut().left = temp;
				temp = node.borrow().parent.clone();
			}
			temp = left_rotate(&mut temp);
			if let Some(node) = temp {
				if let Some(child) = node.borrow().right.clone() {
					child.borrow_mut().parent = node.borrow().parent.clone();
					new_tree = Some(Rc::new(RefCell::new(child.borrow().clone())));
				}
				node.borrow_mut().right = new_tree;
				new_tree = right_rotate(&mut node.borrow().parent.clone());
			}
		},
		// right right = 1 left rotate
		(2, 2) => {
			if let Some(node) = tree.clone() {
				if let Some(parent) = node.borrow().parent.clone() {
					if let Some(sib) = parent.borrow().left.clone() { sib.borrow_mut().parent = parent.borrow().parent.clone(); }
				}
				if let Some(parent) = node.borrow().parent.clone() { new_tree = parent.borrow().parent.clone(); }
			}
			new_tree = left_rotate(&mut new_tree);
		},
		(2, 1) => {
			// right left
			let mut temp: RedBlackTree = None;
			if let Some(node) = tree {
				if let Some(child) = node.borrow().right.clone() {
					child.borrow_mut().parent = node.borrow().parent.clone();
					temp = Some(Rc::new(RefCell::new(child.borrow().clone())));
				}
				node.borrow_mut().right = temp;
				temp = node.borrow().parent.clone();
			}
			temp = right_rotate(&mut temp);
			if let Some(node) = temp {
				if let Some(child) = node.borrow().left.clone() {
					child.borrow_mut().parent = node.borrow().parent.clone();
					new_tree = Some(Rc::new(RefCell::new(child.borrow().clone())));
				}
				node.borrow_mut().left = new_tree;
				new_tree = left_rotate(&mut node.borrow().parent.clone());
			}
		}
		_ => panic!("Error in checking rotations")
	}
	find_root(&mut new_tree)
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
			panic!("Something went wrong when inserting the node in the tree")
		},
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


/*
insert Functions
*/
fn insert(tree: &mut RedBlackTree, t: u32) -> RedBlackTree {
	let rtree = insert_helper(tree, t);
	let inserted_node = find_node(&mut rtree.clone(), t);
	balance_tree(&mut inserted_node.clone())
}

fn insert_helper(tree: &mut RedBlackTree, t: u32) -> RedBlackTree {
	match tree {
		Some(root) => {
			let node;
			if root.borrow().data > t {
				node = insert_helper(&mut root.borrow().left.clone(), t);
				//print_tree(&node.clone(), 0);
				root.borrow_mut().left = node;
				match root.borrow().left.clone() {
					Some(child) => {
						child.borrow_mut().parent = Some(Rc::new(RefCell::new(root.borrow().clone())));
						child.borrow().parent.clone()
					},
					None => panic!("Error: Failed to insert node")
				}
			} else {
				node = insert_helper(&mut root.borrow().right.clone(), t);
				root.borrow_mut().right = node;
				match root.borrow().right.clone() {
					Some(child) => {
						child.borrow_mut().parent = Some(Rc::new(RefCell::new(root.borrow().clone())));
						child.borrow().parent.clone()
					},
					None => panic!("Error: Failed to insert node")
				}
			}
		}
		None => new_rb(t,false)
	}
}

/*
	Delete Functions
*/
fn delete(tree: &mut RedBlackTree, t: u32) -> RedBlackTree{
	delete_helper(tree,t)
	//still have to balance properly
}

fn delete_helper(tree: &mut RedBlackTree, t: u32) -> RedBlackTree {
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
						}
						child.borrow_mut().parent = root.borrow().parent.clone();
					}
					return root.borrow().right.clone();
				} else if root.borrow().right.is_none() {
					if let Some(child) = root.borrow().left.clone() {
						if !child.borrow().color.is_red() || !root.borrow().color.is_red() {
							child.borrow_mut().color = NodeColor::Black;
						}
						child.borrow_mut().parent = root.borrow().parent.clone();
					}
					return root.borrow().left.clone();
				} else {
					let maximum = find_max(&mut root.borrow_mut().left);
					if let Some(max) = maximum {
						let temp =  max.borrow().data;
						let tempcol =  max.borrow().color;
						if let Some(clear) = max.borrow_mut().parent.clone() {
							if clear.borrow().data != root.borrow().data {
								let x = find_node(&mut Some(root.clone()), clear.borrow().data);
								if let Some(y) = x.clone() {
									root.borrow_mut().data = temp;
									y.borrow_mut().right = None;
									y.borrow_mut().color = tempcol;
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

/*
	Red Black Tree Functions
*/
fn in_order_transversal(tree: &RedBlackTree) {
	match tree {
		Some(node) => {
			in_order_transversal(&node.borrow().left);
			println!("data = {}, the color is {:?}", node.borrow().data,node.borrow().color);
			in_order_transversal(&node.borrow().right);
		}
		None => println!("Null Leaf"),
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

fn print_tree(tree: &RedBlackTree, indent: usize) {
	match tree {
		Some(leaf) => {
			print!("{:indent$}","",indent = indent);
			println!("data: {}, color: {:?},", leaf.borrow().data, leaf.borrow().color);
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

fn print_cmds() {
	println!("\n--------------------------------------------------------------------");
	println!("What do you want to do with the tree?");
	println!("Type i if you want to insert into the tree");
	println!("Type d if you want to delete data from the tree");
	println!("Type c if you want to know how many leaves are in the tree");
	println!("Type h if you want to know the height of the tree");
	println!("Type o if you want to print the tree in-order traversal");
	println!("Type e if you want to check if the tree is empty");
	println!("Type p if you want to print the tree itself with color and direction");
	println!("Type s if you want to know how many nodes the tree has");
	println!("Type exit to end the program");
	println!("--------------------------------------------------------------------");
	println!("Type your input : ");

}

/**
 * Command Line Interface functions
 */
fn run() {
	let mut root : u32 = 0;
	let mut input_string = String::new();
	let mut looping = true;
	let mut tree : RedBlackTree;
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
		tree = new_rb(root , true);

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
							tree = insert(&mut tree, i);
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

/*
	Tests
*/
#[cfg(test)]
mod tests {

	use super::*;

	fn setup() -> (RedBlackTree, RedBlackTree){
		let t = TreeNode {
			color: NodeColor::Black,
			data: 4,
			parent: None,
			left: None,
			right: None
		};
		let l = Some(Rc::new(RefCell::new(t)));
		let test = new_rb(4,true);
		(test,l)
	}
	fn cmp_tree(t1 : RedBlackTree, t2 : RedBlackTree) -> bool{
		match t1 {
			Some(x) => {
				match t2 {
					Some(y) => {
						
						if x.borrow().color.is_black() == y.borrow().color.is_black() {
							if x.borrow().data == y.borrow().data {
								
								if x.borrow().left == y.borrow().left {
									
									if x.borrow().right == y.borrow().right {
										
										if let Some(d) = x.borrow().parent.clone() {
											if let Some(f) = y.borrow().parent.clone() {
												println!("{}", f.borrow().data);
												return f.borrow().data == d.borrow().data; 
											}
										} else {
											if x.borrow().parent.is_none() && y.borrow().parent.is_none() {
												return true;
											}
										}
									}
								}
							}
						}
					},
					None => return false
				}
			},
			None => return false
		}
		false
	}
	#[test]
	fn test_create_tree() {

		let (test,l) = setup();
		assert!(cmp_tree(test, l));
	}

	#[test]
	fn test_insert() {
		let (mut test,l) = setup();
		test = insert(&mut test,5);
		match l {
			Some(x) => {
				let node = TreeNode {
					color: NodeColor::Red,
					data: 5,
					parent: Some(x.clone()),
					left: None,
					right: None
				};
				let tnode = Some(Rc::new(RefCell::new(node)));
				x.borrow_mut().right = tnode;
				
				match test {
					Some(t) => {
						assert!(cmp_tree(t.borrow().right.clone(), x.borrow().right.clone()));
					},
					None => {},
				}
			},
			None => {
				panic!("Tree node was not created");
			}
		}
		assert!(false);
	}
}