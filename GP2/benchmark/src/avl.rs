



use std::cell::RefCell;
use std::rc::Rc;

type Tree = Rc<RefCell<TreeNode<u32>>>;
pub type AVLTree= Option<Tree>;
// type AVLTree= Option<Rc<RefCell<TreeNode<u32>>>>;

#[derive(Clone,PartialEq)]
pub struct TreeNode<T> {
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

fn height(tree: &AVLTree) -> u64 {
	match tree {
		None => {0}
		Some(node) => {
			let left_height = height(&node.borrow().left);
			let right_height = height(&node.borrow().left);
			if left_height > right_height{
				left_height + 1
			} else { // if equal, it doesn't matter which one is returned
				right_height + 1
			}
		}
	}
}

fn tree_is_empty(tree: AVLTree) -> bool {
	match tree {
		None => {true}
		Some(_) => {false}
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

fn print_in_order(tree: &AVLTree){
	match tree {
		None => {}
		Some(x) => {
			print_in_order(&x.borrow().left);
			print!("{}, ", x.borrow().data);
			print_in_order(&x.borrow_mut().right);
		}
	}
}

// fn right_rotate(node: &mut AVLTree) -> AVLTree {
// 	let root = node.clone();
// 	if let Some(main_node) = node {
// 		if let Some(parent) = main_node.borrow().parent.clone() {
// 			// let main_right = main_node.borrow().right.clone();
// 			parent.borrow_mut().left = main_node.borrow().right.clone();
// 			main_node.borrow_mut().right = Some(parent.clone());
// 			// Some(main_node.clone())
// 			root
// 		} else {
// 			panic!("node should have a parent")
// 		}
// 	} else {
// 		panic!("Cannot rotate on null node")
// 	}
// }

fn change_node_value(node: AVLTree) {
	match (node) {
		None => {},
		Some(main_node) => {
			let mut holder = main_node.borrow_mut();
			holder.data = 4;
		}
	}
}

fn right_rot(node: AVLTree) {
	match node {
		Some(main_node) => {
			let mut main_holder = main_node.borrow_mut();
			match &main_holder.left {
				Some(left_child) => {
					let mut left_holder = left_child.borrow_mut();
					match &left_holder.left  {
						None => {println!("node.left.left is None.");},
						Some(left_left_child) => {
							let mut left_left_holder = left_left_child.borrow_mut();
							// match &main_holder.parent {
							// 	None => {println!("node.parent is None.");},
							// 	Some(node_parent) => {
							// 		let mut parent = node_parent.borrow();
							// 	}
							// }
						}
					}
					println!("{}", left_holder.data);
				},
				None => {
					println!("Cannot rotate on this.");
				}
			}
		}, None => {
			println!("Why tf you rotating on nothing.");
		}
	}
}


// fn right_rotate(grandparent_tree : &mut AVLTree) -> AVLTree {
// 	// make grandparent is right child of the parent. right child of parent is the left child of the grandparent
// 	match grandparent_tree {
// 		Some(grandparent) => {
// 			// let color = grandparent.borrow().color;
// 			let parent = grandparent.borrow().parent.clone();
// 			// let copy = grandparent.borrow().left;
// 			match copy {
// 				Some(left_child) => {
// 					match &left_child.borrow_mut().right {
// 						Some(x) =>{
// 							x.borrow_mut().parent = left_child.borrow().parent.clone(); // currently the grandparent
// 						},
// 						None => {},
// 					}

// 					grandparent.borrow_mut().left = left_child.borrow().right.clone();
				
// 					// grandparent.borrow_mut().color = left_child.borrow().color;
// 					left_child.borrow_mut().parent = parent;
// 					// left_child.borrow_mut().color = color;
// 					grandparent.borrow_mut().parent = Some(Rc::new(RefCell::new(left_child.borrow().clone())));
// 					left_child.borrow_mut().right = Some(Rc::new(RefCell::new(grandparent.borrow().clone())));
// 					if let Some(value) = left_child.borrow().parent.clone() { value.borrow_mut().right = Some(Rc::new(RefCell::new(left_child.borrow().clone()))); }
// 					Some(Rc::new(RefCell::new(left_child.borrow().clone())))
// 				},
// 				None => panic!("there should be a left child")
// 			}
// 		},
// 		None=> panic!("Should be a grandparent")
// 	}
// }

// fn right_rotate(grandparent_tree : &mut AVLTree) -> AVLTree{
// 	// make grandparent is right child of the parent. right child of parent is the left child of the grandparent
// 	match grandparent_tree {
// 		Some(grandparent) => {
// 			let parent = grandparent.borrow().parent.clone();
// 			grandparent.borrow_mut().parent = grandparent.borrow().left.clone();
// 			match grandparent.borrow_mut().parent.clone() {
// 				Some(left_child) => {
// 					grandparent.borrow_mut().left = left_child.borrow().right.clone();
// 					left_child.borrow_mut().right = left_child.borrow().parent.clone();
// 					left_child.borrow_mut().parent = parent;
// 					grandparent.borrow().parent.clone()
// 				},
// 				None => panic!("there should be a left child")
// 			}
// 		},
// 		None=> panic!("Should be a grandparent")
// 	}
// }

// fn right_rotate(node: &mut AVLTree) -> AVLTree {
// 	match node {
// 		Some(main_node) => {
// 			let right_of_node = main_node.borrow_mut().right.as_ref();
// 			match main_node.borrow().right.clone(){ // get right of main node
// 				Some(right_node) => {
// 					match main_node.borrow().parent.clone(){ // get parent
// 						Some(parent)=>{ 
// 							match parent.borrow().parent.clone(){ // get parent's parent
// 								Some(parents_parent)=>{
// 									main_node.borrow_mut().right = Some(Rc::clone(&parent));
// 									main_node.borrow_mut().parent = Some(Rc::clone(&parents_parent));
// 									parent.borrow_mut().left = Some(Rc::clone(&right_node));
// 									Some(parent.clone())
// 								},
// 								None => {Some(main_node.clone())}
// 							}
// 						},
// 						None => {Some(main_node.clone())}
// 					}
// 				},
// 				None => {Some(main_node.clone())}
// 			}
// 		},
// 		None => {node.clone()}
// 	}
// }

fn print_tree(tree: &AVLTree){
	match tree {
		None => {print!("nil");}
		Some(x) => {
			match &x.borrow().parent{
				None => {print!("{}->(", x.borrow().data);}
				Some(pair) => {print!("{}({}, {})->(", x.borrow().data, pair.borrow().data, x.borrow().height);}
			}
			print_tree(&x.borrow().left);
			print!("),(");
			print_tree(&x.borrow().right);
			print!(")");
		}
	}
}

pub fn new_avl(t: u32, if_root : bool) -> AVLTree {
    if if_root {
        let node = TreeNode::new_node(t);
        let root = Some(Rc::new(RefCell::new(node)));
        return root;
    } else {
        Some(Rc::new(RefCell::new(TreeNode::new_node(t))))
    }
}

pub fn insert_node(tree: &mut AVLTree, t: u32) -> AVLTree {
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
	// balance_tree(inserted_node)
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


// fn main() {
//     println!("Hello, world!");
//     let mut tree = new_avl(6, true);
// 	// println!("{:?}", tree);
// 	// print_tree(&tree);
// 	// print_in_order(&tree);
// 	insert(&mut tree, 4);
// 	insert(&mut tree, 3);
// 	insert(&mut tree, 5);
// 	insert(&mut tree, 7);
// 	// insert(&mut tree, 3);
// 	// insert(&mut tree, 6);
// 	print_tree(&tree);
// 	println!(": {}", height(&tree));
// 	print_in_order(&tree);
// 	println!("We are done with the first one\n");

// 	let inserted_node = &mut find_node_help(&tree, 4);
// 	// print_tree(&inserted_node);
// 	// println!(": {}", height(&inserted_node));
// 	// println!("Print from 5 done");

// 	left_rotate(inserted_node);
// 	print_tree(&tree);
// 	println!(": {}\n", height(&tree));

// 	tree = delete(&mut tree, 4);
// 	print_tree(&tree);
// 	// print_in_order(&tree);
// 	// println!("");
    
//     // let mut tree = Tree::Node{
//     //     data: "5",
//     //     left_child: Box::new(Tree::Empty),
//     //     right_child: Box::new(Tree::Empty)
//     // };
//     // tree.insert_node("3");
//     // tree.insert_node("4");
//     // tree.insert_node("2");
//     // tree.insert_node("8");
//     // tree.insert_node("7");
//     // tree.insert_node("9");
//     // println!("{:?}", tree);
// }

