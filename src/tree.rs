use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;
use tree_node::TreeNode;
use std::ops::Add;

/// A reference counting pointer to a single node in a tree.
type Node<T> = Rc<TreeNode<T>>;

/// Represents a tree data structure.
pub struct Tree<T> {
    root: Node<T>,
    node_count: usize
}

impl<T> Tree<T> {
    /// Create a new tree.
    /// 
    /// # Arguments
    /// * `val` - value of the root node
    pub fn new(val: T) -> Tree<T> {
        Tree {
            root: Tree::create_node(val),
            node_count: 1
        }
    }

    /// Get a reference counting pointer to the root node of the tree.
    pub fn get_root(&self) -> Node<T> {
        Rc::clone(&self.root)
    }

    /// Get the number of nodes in the tree.
    pub fn get_node_count(&self) -> usize {
        self.node_count
    }

    /// Adds a new node to the tree.
    /// 
    /// # Arguments
    /// * `parent` - reference to the parent node
    /// * `val` - value of the child node
    /// 
    /// # Returns
    /// A reference counting pointer to the created child node.
    pub fn add_child(&mut self, parent: &Node<T>, val: T) -> Node<T> {
        let child = Tree::create_node(val);
        child.set_parent(parent);
        parent.add_child(Rc::clone(&child));
        self.node_count += 1;
        child
    }

    /// Aggregates values in the tree into a single value.
    /// 
    /// # Arguments
    /// 
    /// `start_node` - a reference to the node the aggregation should start from
    /// `f` - a closure returning the value that should be aggregated.
    /// 
    /// # Returns
    /// The aggregated value.
    pub fn aggregate<F, R>(&self, start_node: &Node<T>, f: F) -> R 
    where
        R: Add<Output = R>,
        F: FnOnce(Node<T>) -> R + Copy
    {
        let mut stack = vec![Rc::clone(&start_node)];
        let mut value: Option<R> = None;

        while stack.len() > 0 {
            let current_node = stack.pop().unwrap();
            if current_node.has_children() {
                for child in current_node.get_children() {
                    stack.push(Rc::clone(&child));
                }
            }
            match value {
                Some(val) => value = Some(val + f(current_node)),
                None => value = Some(f(current_node))
            }
        }
        value.unwrap()
    }

    /// Aggregates values in the tree into a single value, starting from the root node.
    /// 
    /// # Arguments
    /// 
    /// `f` - a closure returning the value that should be aggregated
    /// 
    /// # Returns
    /// The aggregated value.
    pub fn aggregate_root<F, R>(&self, f: F) -> R 
    where
        R: Add<Output = R>,
        F: FnOnce(Node<T>) -> R + Copy
    {
        self.aggregate(&self.get_root(), f)
    }

    fn create_node(val: T) -> Node<T> {
        Rc::new(TreeNode::new(val))
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

        pub fn has_children(&self) -> bool {
            self.get_children().len() > 0
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
        let mut tree = Tree::new(5);
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

        let tree = Tree::new(TestStruct {
            test_value: 1,
            test_vector: vec![],
        });
        assert_eq!(1, tree.get_root().val.test_value);
    }

    #[test]
    fn test_aggregate() {
        let mut tree = Tree::new(5);
        let root = tree.get_root();
        tree.add_child(&root, 8);
        let child = tree.add_child(&root, 12);
        tree.add_child(&child, 20);
        assert_eq!(4, tree.get_node_count());
        assert_eq!(45, tree.aggregate_root(|node| node.val));
        assert_eq!(32, tree.aggregate(&child, |node| node.val));
    }
}