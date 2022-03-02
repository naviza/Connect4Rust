use std::cell::RefCell;
use std::rc::Rc;

#[derive(Copy, Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
}

type Tree = Rc<RefCell<TreeNode<u32>>>;
type RedBlackTree= Option<Tree>;

#[derive(Clone)]
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
    fn is_red(self) -> bool{
        match self {
            NodeColor::Black => false,
            NodeColor::Red => true,
        }
    }

    fn flip_color(self) -> Self{
        match self {
            NodeColor::Black => NodeColor::Red,
            NodeColor::Red => NodeColor::Black,
        }
    }
}

fn new_rb(t: u32,if_root : bool) -> RedBlackTree {
    if if_root {
        let node = TreeNode::new_node(t);
        node.color.flip_color();
        let root = Some(Rc::new(RefCell::new(node)));
        return root;
    } else {
        Some(Rc::new(RefCell::new(TreeNode::new_node(t))))
    }
}

//fn assign(parent: &mut RedBlackTree, child: &mut RedBlackTree) {
//	match child {
//		Some(node) => node.borrow_mut().left = parent.clone(),
//		None => println!("Error: Failed to assign parent to child")
//	};
//	match parent {
//		Some(node) => node.borrow_mut().left = child.clone(),
//		None => println!("Error: Failed to assign child to parent")
//	}
//}

fn violations(tree: &mut RedBlackTree) {

}

fn right_rotate() {

}

fn left_rotate() {

}

fn balance_tree(tree: &mut Tree) {
    match &tree.borrow().parent {
        //if it is None, that means its the root
        Some(parent_node) => {
            //node is the parent
            //have to check the color of the parent
            if parent_node.borrow().color.is_red() {
                // have to check if the "uncle" is a red node
            }
        },
        None => tree.borrow().color = tree.borrow().color.flip_color(),
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

fn insert(tree: &mut RedBlackTree, t: u32) -> RedBlackTree {
    match tree {
        Some(root) => {
            let node;
            if root.borrow().data > t {
                node = insert(&mut root.borrow_mut().left, t);
                root.borrow_mut().left = node;
                match &mut root.borrow_mut().left {
                    Some(child) => {
                        child.borrow_mut().parent = Some(root.clone());
                        balance_tree(child); 
                    },
                    None => println!("Error: Failed to insert node")
                };
            } else {
                node = insert(&mut root.borrow_mut().right, t);
                root.borrow_mut().right = node;
                match &mut root.borrow_mut().right {
                    Some(child) => {
                        child.borrow_mut().parent = Some(root.clone());
                    },
                    None => println!("Error: Failed to insert node")
                };
            }
            Some(root.clone())

        }
        None => new_rb(t,false)
    }
}

fn delete(tree: &mut RedBlackTree, t: u32) {
	match tree {
		Some(root) => {
			if root.borrow().data == t {
				
			} else {
				delete(&mut root.borrow_mut().left, t);
				delete(&mut root.borrow_mut().right, t);
			}
		}
		None => {}
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

fn print_tree(tree: &RedBlackTree) {
	match tree {
		Some(leaf) => {
			println!("data: {}", leaf.borrow().data);
			print_tree(&leaf.borrow().left);
			print_tree(&leaf.borrow().right);
		}
		None => println!("NULL Leaf")
	};
}

fn test_tree() {
    let mut t: RedBlackTree = new_rb(5,true);
    insert(&mut t, 10);
    insert(&mut t, 20);
    println!("Printing Tree");
    print_tree(&t);
    println!();
    println!("Number of nodes: {}", size(&t));
    println!("Number of leaves: {}", count_leaves(&t));
    println!();
    println!("Printing tree in-order transversal");
    in_order_transversal(&t);
    println!();
}

fn main() {
    test_tree();
}
