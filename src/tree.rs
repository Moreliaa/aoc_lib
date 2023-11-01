use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

pub mod tree {
    use super::*;
    use tree_node::TreeNode;
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

        pub fn get_node_count(&self) -> i32 {
            self.node_count
        }

        pub fn create_node(val: T) -> Rc<TreeNode<T>> {
            Rc::new(TreeNode::new(val))
        }

        pub fn add_child(&mut self, parent: &Rc<TreeNode<T>>, val: T) {
            let child = Tree::create_node(val);
            child.set_parent(parent);
            parent.add_child(child);
            self.node_count += 1;
        }
    }
}

mod tree_node {
    use super::*;

    #[derive(Debug)]
    pub struct TreeNode<T> {
        pub val: T,
        parent: RefCell<Weak<TreeNode<T>>>,
        children: RefCell<Vec<Rc<TreeNode<T>>>>
    }

    impl<T> TreeNode<T> {
        pub fn new(val: T) -> TreeNode<T> {
            TreeNode {
                val,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![])
            }
        }

        pub fn get_parent(&self) -> Option<Rc<TreeNode<T>>> {
            self.parent.borrow().upgrade()
        }

        pub fn get_mut_parent(&self) -> Option<Rc<TreeNode<T>>> {
            self.parent.borrow_mut().upgrade()
        }

        pub fn get_children(&self) -> Vec<Rc<TreeNode<T>>> {
            self.children.borrow().to_vec()
        }

        pub fn get_mut_children(&self) -> Vec<Rc<TreeNode<T>>> {
            self.children.borrow_mut().to_vec()
        }

        pub fn set_parent(&self, parent: &Rc<TreeNode<T>>) {
            *self.parent.borrow_mut() = Rc::downgrade(&parent);
        }

        pub fn add_child(&self, child: Rc<TreeNode<T>>) {
            self.children.borrow_mut().push(child);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_child() {
        let mut tree = tree::Tree::new(5);
        let root = tree.get_root();
        tree.add_child(&root, 8);
        assert_eq!(2, tree.get_node_count());
        let child = root.get_children();
        let child = &child[0];
        let parent = child.get_parent().unwrap();
        assert_eq!(5, parent.val);
        assert_eq!(8, child.val);
    }
}