use std::io;
mod rb_tree;

fn print_commands() {
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
	let mut tree : rb_tree::RedBlackTree;
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
		tree = rb_tree::new_rb(root , true);

		while trimmed != "exit" {
			print_commands();
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
							tree = rb_tree::insert(&mut tree, i);
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
							tree = rb_tree::delete(&mut tree, i);
						},
						Err(..) => {
							println!("this was not an integer: {}", trimmed)
						},
					};
				},
	
				"c" => println!("The amount of leaves this tree has is {}", rb_tree::count_leaves(&tree)),
	
				"h" => println!("The tree has a height of {}",rb_tree::count_height(&tree)),
	
				"o" => {
					println!("Printing in-order transversal...");
					rb_tree::in_order_transversal(&tree);
				},
	
				"e" => {
					println!("Checking if the tree is empty...");
					if rb_tree::is_empty(&tree) {
						println!("The tree is empty");
					} else {
						println!("The tree is not empty");
					}
				},
	
				"p" => {
					println!("Printing the tree...");
					rb_tree::print_tree(&tree, 0);
				},

				"s" => {
					println!("Printing the size of the tree...");
					println!("The tree has {} nodes", rb_tree::size(&tree));
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