#[cfg(test)]
mod tests {

	use rb_tree;

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
	fn cmp_tree(t1 : rb_tree::RedBlackTree, t2 : rb_tree::RedBlackTree) -> bool{
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