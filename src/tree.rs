use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

pub struct Tree<T> {
    root: Rc<TreeNode<T>>,
    node_count: i32
}

impl<T> Tree<T> {
    pub fn new(val: T) -> Tree<T> {
        Tree {
            root: Tree::create_node(val),
            node_count: 1
        }
    }

    pub fn get_root(&self) -> Rc<TreeNode<T>> {
        Rc::clone(&self.root)
    }

    pub fn create_node(val: T) -> Rc<TreeNode<T>> {
        Rc::new(
            TreeNode {
                val,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![])
            }
        )
    }

    pub fn add_child(&mut self, parent: &Rc<TreeNode<T>>, val: T) {
        let child = Tree::create_node(val);
        *child.parent.borrow_mut() = Rc::downgrade(&parent);
        parent.children.borrow_mut().push(child);
        self.node_count += 1;
    }
}

#[derive(Debug)]
pub struct TreeNode<T> {
    pub val: T,
    parent: RefCell<Weak<TreeNode<T>>>,
    children: RefCell<Vec<Rc<TreeNode<T>>>>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_child() {
        let mut tree = Tree::new(5);
        let root = tree.get_root();
        tree.add_child(&root, 8);
        assert_eq!(2, tree.node_count);
        let child = root.children.borrow();
        let child = &child[0];
        let parent = child.parent.borrow().upgrade().unwrap();
        assert_eq!(5, parent.val);
        assert_eq!(8, child.val);
    }

}