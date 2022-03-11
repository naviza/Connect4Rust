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


fn read_number() -> i32{
	let mut input_string = String::new();
	let mut trimmed = "";
	input_string = ("").to_string();
	io::stdin().read_line(&mut input_string).expect("failed to read line");

	trimmed = input_string.trim();

	match trimmed.parse::<i32>() {
		Ok(i) => {
			return i;
		}
		Err(..) => {
			println!("this was not an integer: {}", trimmed);
			return -2;
		}
	};
}

fn read_input() -> String {
	let mut input_string = String::new();

	input_string = ("").to_string();

	io::stdin().read_line(&mut input_string).expect("failed to read line");
	String::from(input_string.trim())
}

/**
 * Command Line Interface functions
 */
fn run(root : u32) {

	let mut tree: rb_tree::RedBlackTree = None;
	println!("Creating a new tree with the root data provided - {} ",root);

	tree = rb_tree::insert(&mut tree, root);

	loop {
		print_commands();

		let trimmed = &read_input()[..]; // converts String to &str

		match trimmed {
			"i" => {
				println!("What value do you want to insert into the tree:");
				let n = read_number();
				if n >= 0 {
					if let Ok(i) = u32::try_from(n) {
						tree = rb_tree::insert(&mut tree, i);
					}
				}
			}
			"d" => {
				println!("What value do you want to delete from the tree:");
				let n = read_number();
				if n >= 0 {
					if let Ok(i) = u32::try_from(n) {
						tree = rb_tree::delete(&mut tree, i);
					}
				}
			}

			"c" => println!(
				"The amount of leaves this tree has is {}",
				rb_tree::count_leaves(&tree)
			),

			"h" => println!("The tree has a height of {}", rb_tree::count_height(&tree)),
			"o" => {
				println!("Printing in-order transversal...");
				rb_tree::in_order_transversal(&tree);
			}
			"e" => {
				println!("Checking if the tree is empty...");
				if rb_tree::is_empty(&tree) {
					println!("The tree is empty");
				} else {
					println!("The tree is not empty");
				}
			}
			"p" => {
				println!("Printing the tree...");
				rb_tree::print_tree(&tree, 0);
			}

			"s" => {
				println!("Printing the size of the tree...");
				println!("The tree has {} nodes", rb_tree::size(&tree));
			}
			"exit" => {
				println!("Exiting the program");
				break;
			}

			_ => println!("The command you entered is not found, please try again"),
		}
	}
}

fn main() {
	let mut r : i32 = -2;

	loop {
		println!("What do you want your root for a new tree is (Enter a positive integer or -1 to exit) - ");
		let temp = read_number();
		if temp >= 0 {
			r = temp;
			break;
		} else if temp == -1{
			r = -1;
			println!("exiting program!");
			break;
		}
	}

	if r >= 0 { 
		if let Ok(root) = u32::try_from(r) {
			run(root);
		} else {
			panic!("Something went wrong");
		}
	} 
}
