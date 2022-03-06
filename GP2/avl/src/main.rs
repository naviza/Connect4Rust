



use std::cell::RefCell;
use std::rc::Rc;

type Tree = Rc<RefCell<TreeNode<u32>>>;
type AVLTree= Option<Tree>;
// type AVLTree= Option<Rc<RefCell<TreeNode<u32>>>>;

#[derive(Clone,PartialEq,Debug)]
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

fn right_rotate(node: &mut AVLTree) {
	match node {
		Some(main_node) => {
			let right_of_node = main_node.borrow().right.as_ref();
			match main_node.borrow_mut().right.clone(){ // get right of main node
				Some(right_node) => {
					match main_node.borrow_mut().parent.clone(){ // get parent
						Some(parent)=>{ 
							match parent.borrow_mut().parent.clone(){ // get parent's parent
								Some(parents_parent)=>{
									main_node.borrow_mut().right = Some(Rc::clone(&parent));
									main_node.borrow_mut().parent = Some(Rc::clone(&parents_parent));
									parent.borrow_mut().left = Some(Rc::clone(&right_node));
								},
								None => {}
							}
						},
						None => {}
					}
				},
				None => {}
			}
		},
		None => {}
	}
	// let left_of_node = node.borrow().left;
	// let mut right_of_node = node.right;
	// match node.parent{
	// 	None => {println!("Something went wrong.")},
	// 	Some(parent.borrow_mut()) => {
	// 		match parent.borrow_mut().left{
	// 			None => {}
	// 			Some (parent_left) => {}

	// 		}
	// 		// let left_of_parent = parent.borrow_mut().left;
	// 		// parent.borrow_mut().left = right_of_node;
	// 		// node.right = parent;
	// 	}
	// }
}

fn print_tree(tree: &AVLTree){
	match tree {
		None => {print!("nil");}
		Some(x) => {
			match &x.borrow().parent{
				None => {print!("{}->(", x.borrow().data);}
				Some(pair) => {print!("{}({})->(", x.borrow().data, pair.borrow().data);}
			}
			print_tree(&x.borrow().left);
			print!("),(");
			print_tree(&x.borrow().right);
			print!(")");
		}
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


fn main() {
    println!("Hello, world!");
    let mut tree = new_avl(5, true);
	// println!("{:?}", tree);
	// print_tree(&tree);
	// print_in_order(&tree);
	insert(&mut tree, 2);
	insert(&mut tree, 1);
	insert(&mut tree, 3);
	insert(&mut tree, 6);
	print_tree(&tree);
	println!(": {}", height(&tree));
	print_in_order(&tree);
	println!("We are done with the first one\n");

	let inserted_node = &mut find_node_help(&tree, 2);
	right_rotate(inserted_node);
	print_tree(&tree);
	println!(": {}", height(&tree));
	print_in_order(&tree);
	println!("");
    
    // let mut tree = Tree::Node{
    //     data: "5",
    //     left_child: Box::new(Tree::Empty),
    //     right_child: Box::new(Tree::Empty)
    // };
    // tree.insert_node("3");
    // tree.insert_node("4");
    // tree.insert_node("2");
    // tree.insert_node("8");
    // tree.insert_node("7");
    // tree.insert_node("9");
    // println!("{:?}", tree);
}

