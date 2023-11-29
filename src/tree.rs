use std::collections::HashMap;
use std::ops::Add;
use tree_node::TreeNode;

/// Represents a tree data structure.
pub struct Tree<T> {
    nodes: HashMap<usize, TreeNode<T>>,
    node_count: usize,
}

impl<T> Tree<T> {
    /// Create a new tree.
    ///
    /// # Arguments
    /// * `val` - value of the root node
    pub fn new(val: T) -> Tree<T> {
        let mut tree = Tree {
            nodes: HashMap::new(),
            node_count: 0,
        };

        tree.create_node(val);
        tree
    }

    fn create_node(&mut self, val: T) -> usize {
        let rc = TreeNode::new(val);
        self.nodes.insert(self.node_count, rc);
        self.node_count += 1;
        self.node_count - 1
    }

    pub fn get_val(&self, id: usize) -> &T {
        &self.nodes.get(&id).unwrap().val
    }

    pub fn get_mut_val(&mut self, id: usize) -> &mut T {
        &mut self.nodes.get_mut(&id).unwrap().val
    }

    pub fn get_parent_id(&self, id: usize) -> &Option<usize> {
        &self.nodes.get(&id).unwrap().get_parent_id()
    }

    pub fn get_child_ids(&self, id: usize) -> &Vec<usize> {
        &self.nodes.get(&id).unwrap().get_child_ids()
    }

    fn get_node(&self, id: usize) -> &TreeNode<T> {
        &self.nodes.get(&id).unwrap()
    }

    fn get_mut_node(&mut self, id: usize) -> &mut TreeNode<T> {
        self.nodes.get_mut(&id).unwrap()
    }

    /// Get the number of nodes in the tree.
    pub fn get_node_count(&self) -> usize {
        self.node_count
    }

    /// Adds a new node to the tree.
    ///
    /// # Arguments
    /// * `parent_id` - id of the parent node. The id of the root node is always 0.
    /// * `val` - value of the child node
    ///
    /// # Returns
    /// The id of the created child node.
    pub fn add_child(&mut self, parent_id: usize, val: T) -> usize {
        let child_id = self.create_node(val);
        let child = self.get_mut_node(child_id);
        child.set_parent(parent_id);
        let parent = self.get_mut_node(parent_id);
        parent.add_child(child_id);
        child_id
    }

    /// Aggregates values in the tree into a single value.
    ///
    /// # Arguments
    ///
    /// `start_id` - id of the node the aggregation should start from
    /// `f` - a closure returning the value that should be aggregated.
    ///
    /// # Returns
    /// The aggregated value.
    pub fn aggregate<F, R>(&self, start_id: usize, f: F) -> R
    where
        R: Add<Output = R>,
        F: FnOnce(&T) -> R + Copy,
    {
        let mut stack = vec![start_id];
        let mut value: Option<R> = None;

        while stack.len() > 0 {
            let current_node = stack.pop().unwrap();
            let current_node = self.get_node(current_node);
            if current_node.has_children() {
                for child in current_node.get_child_ids() {
                    stack.push(*child);
                }
            }
            match value {
                Some(val) => value = Some(val + f(&current_node.val)),
                None => value = Some(f(&current_node.val)),
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
        F: FnOnce(&T) -> R + Copy,
    {
        self.aggregate(0, f)
    }
}

mod tree_node {
    #[derive(Debug)]
    pub struct TreeNode<T> {
        pub val: T,
        parent: Option<usize>,
        children: Vec<usize>,
    }

    impl<T> TreeNode<T> {
        pub fn new(val: T) -> TreeNode<T> {
            TreeNode {
                val,
                parent: None,
                children: vec![],
            }
        }

        pub fn get_parent_id(&self) -> &Option<usize> {
            &self.parent
        }

        pub fn has_children(&self) -> bool {
            self.get_child_ids().len() > 0
        }

        pub fn get_child_ids(&self) -> &Vec<usize> {
            &self.children
        }

        pub fn set_parent(&mut self, parent: usize) {
            self.parent = Some(parent);
        }

        pub fn add_child(&mut self, child: usize) {
            self.children.push(child);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_child() {
        let mut tree = Tree::new(5);
        tree.add_child(0, 8);
        assert_eq!(2, tree.get_node_count());
        let child = tree.get_child_ids(0);
        let child = child[0];
        let parent = tree.get_parent_id(child).unwrap();
        assert_eq!(5, *tree.get_val(parent));
        assert_eq!(8, *tree.get_val(child));
    }

    #[test]
    fn test_struct_type() {
        #[allow(dead_code)]
        struct TestStruct {
            test_value: i32,
            test_vector: Vec<String>,
        }

        let tree = Tree::new(TestStruct {
            test_value: 1,
            test_vector: vec![],
        });
        assert_eq!(1, tree.get_val(0).test_value);
    }

    #[test]
    fn test_aggregate() {
        let mut tree = Tree::new(5);
        tree.add_child(0, 8);
        let child = tree.add_child(0, 12);
        tree.add_child(child, 20);
        assert_eq!(4, tree.get_node_count());
        assert_eq!(45, tree.aggregate_root(|node| *node));
        assert_eq!(32, tree.aggregate(child, |node| *node));
    }

    #[test]
    fn test_mut() {
        let mut tree = Tree::new(5);
        *tree.get_mut_val(0) = 8;
        assert_eq!(*tree.get_val(0), 8);
    }
}
