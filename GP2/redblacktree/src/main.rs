use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
}

type Tree = Rc<RefCell<TreeNode<u32>>>;
type RedBlackTree= Option<Tree>;

struct TreeNode<T> {
    pub color: NodeColor,
    pub data: T,
    pub parent: RedBlackTree,
    left: RedBlackTree,
    right: RedBlackTree,
}

impl NodeColor {
    pub fn red() -> Self{
        NodeColor::Red
    }
    pub fn black() -> Self{
        NodeColor::Black
    }
}

impl<T> TreeNode<T> {
    
    pub fn new(t: T) -> Self {
        TreeNode {
            color : NodeColor::Black,
            data : t,
            parent : None,
            left : None,
            right : None,
        }
    }
    /* pub fn insert(self, t: T) {
        let node = TreeNode {
            color : NodeColor::Black,
            data : t,
            parent : None,
            left : None,
            right : None,
        };
        let root = self; // The first root */
}

fn insert<T> (insert_tree: RedBlackTree, t : T) {
    match insert_tree {
        Some(tree) => {

        },
        None =>{
            let newnode = TreeNode::new(t);
        },
    }   

    
}

fn main() {
    let rootnode : TreeNode<u32>;
    let r : RedBlackTree;
}
