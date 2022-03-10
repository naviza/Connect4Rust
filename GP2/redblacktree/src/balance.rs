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
	}
	
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