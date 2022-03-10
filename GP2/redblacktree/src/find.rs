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