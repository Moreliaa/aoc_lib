use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;
use tree_node::TreeNode;

type Node<T> = Rc<TreeNode<T>>;

pub mod tree {
    use super::*;
    pub struct Tree<T> {
        root: Node<T>,
        node_count: i32
    }

    impl<T> Tree<T> {
        pub fn new(val: T) -> Tree<T> {
            Tree {
                root: Tree::create_node(val),
                node_count: 1
            }
        }

        pub fn get_root(&self) -> Node<T> {
            Rc::clone(&self.root)
        }

        pub fn get_node_count(&self) -> i32 {
            self.node_count
        }

        pub fn create_node(val: T) -> Node<T> {
            Rc::new(TreeNode::new(val))
        }

        pub fn add_child(&mut self, parent: &Node<T>, val: T) -> Node<T> {
            let child = Tree::create_node(val);
            child.set_parent(parent);
            parent.add_child(Rc::clone(&child));
            self.node_count += 1;
            child
        }
    }
}

mod tree_node {
    use super::*;

    #[derive(Debug)]
    pub struct TreeNode<T> {
        pub val: T,
        parent: RefCell<Weak<TreeNode<T>>>,
        children: RefCell<Vec<Node<T>>>
    }

    impl<T> TreeNode<T> {
        pub fn new(val: T) -> TreeNode<T> {
            TreeNode {
                val,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![])
            }
        }

        pub fn get_parent(&self) -> Option<Node<T>> {
            self.parent.borrow().upgrade()
        }

        pub fn get_mut_parent(&self) -> Option<Node<T>> {
            self.parent.borrow_mut().upgrade()
        }

        pub fn get_children(&self) -> Vec<Node<T>> {
            self.children.borrow().to_vec()
        }

        pub fn get_mut_children(&self) -> Vec<Node<T>> {
            self.children.borrow_mut().to_vec()
        }

        pub fn set_parent(&self, parent: &Node<T>) {
            *self.parent.borrow_mut() = Rc::downgrade(&parent);
        }

        pub fn add_child(&self, child: Node<T>) {
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

    #[test]
    fn test_struct_type() {
        #[allow(dead_code)]
        struct TestStruct {
            test_value: i32,
            test_vector: Vec<String>
        }

        let tree = tree::Tree::new(TestStruct {
            test_value: 1,
            test_vector: vec![],
        });
        assert_eq!(1, tree.get_root().val.test_value);
    }
}