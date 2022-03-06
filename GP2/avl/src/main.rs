



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
			} else {
				node = insert_node(&mut root.borrow_mut().right, t);
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
		None => new_avl(t,false)
	}
}

fn insert(tree: &mut AVLTree, t: u32) -> AVLTree {
	let rtree = &mut insert_node(tree, t);
	let inserted_node = &mut find_node(rtree, t);
	// balance_tree(inserted_node)
}


fn main() {
    println!("Hello, world!");
    let mut tree = new_avl(5, true);
    
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

