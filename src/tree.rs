use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeNode<T: Clone + PartialEq + Eq> {
    pub id: usize,
    pub value: T,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl<T: Clone + PartialEq + Eq> TreeNode<T> {
    pub fn new(id: usize, value: T, parent: Option<usize>) -> Self {
        Self { id, value, parent, children: vec![] }
    }
}

pub struct Tree<T: Clone + PartialEq + Eq> {
    pub head: Option<usize>,
    nodes: Vec<TreeNode<T>>,
}

impl<T: Clone + PartialEq + Eq> Tree<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            nodes: vec![],
        }
    }

    pub fn with_head(head: T) -> Self {
        let head = TreeNode::new(0, head, None);
        let nodes = vec![head];
        Self {
            head: Some(0),
            nodes,
        }
    }

    pub fn add_child_to_node(&mut self, value: T, parent: usize) -> Option<usize> {
        if self.nodes.len() > parent {
            let node = TreeNode::new(self.nodes.len(), value, Some(parent));
            let output = self.nodes.len();
            self.nodes.push(node);
            self.nodes[parent].children.push(output);
            Some(output)
        } else {
            // The parent specified doesn't exist
            None
        }
    }

    pub fn path_to_node(&self, node: usize) -> Option<Vec<usize>> {
        if self.nodes.len() > node {
            let mut path = vec![];
            let mut next = node;
            loop {
                path.push(next);
                if let Some(parent) = self.nodes[next].parent {
                    next = parent;
                } else {
                    break;
                }
            }
            Some(path.into_iter().rev().collect())
        } else {
            // The node specified doesn't exist
            None
        }
    }

    pub fn leaves(&self) -> Vec<usize> {
        self.nodes.iter().filter(|n| n.children.len() == 0).map(|n| n.id).collect()
    }

    pub fn leaf_values(&self) -> Vec<&T> {
        let ids = self.leaves();
        let mut output = vec![];
        for id in ids {
            if let Some(node) = self.nodes.get(id) {
                output.push(&node.value);
            }
        }
        output
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn iter(&self) -> TreeIter<T> {
        TreeIter::new(&self)
    }
}

pub struct TreeIter<'a, T: Clone + PartialEq + Eq> {
    tree: &'a Tree<T>,
    idx: Option<usize>,
}

impl<'a, T: Clone + PartialEq + Eq> TreeIter<'a, T> {
    pub fn new(tree: &'a Tree<T>) -> Self {
        Self {
            tree,
            idx: tree.head,
        }
    }
}

/*
impl<'a, T: Clone + PartialEq + Eq> Iterator for TreeIter<'a, T> {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current = self.tree.nodes[self.idx];

        // Go up to next place we have choices
        let mut last = self.idx;
        loop {
            if let Some(parent) = current.parent {
                self.idx = parent;
                current = self.tree.nodes[parent];
                let mut child_iter = current.children.iter();
                loop {
                    let temp = child_iter.next();
                    if temp == Some(self.idx) || temp == None {
                        break;
                    }
                }
                let next = child_iter.next();
                if next.is_some() {
                    self.idx = next;
                    return next;
                }
            } else {
                // You're at the head
                return None;
            }
        }
    }
}
*/

#[cfg(test)]
mod tests {
    use super::Tree;

    #[test]
    fn len() {
        let tree = Tree::<usize>::new();
        assert_eq!(tree.len(), 0);

        let value = 0_usize;
        let mut tree = Tree::with_head(value);
        assert_eq!(tree.len(), 1);

        let new_value = 12;
        let node1 = tree.add_child_to_node(new_value, 0).unwrap();
        assert_eq!(tree.len(), 2);

        let node2 = tree.add_child_to_node(2, node1).unwrap();
        let node3 = tree.add_child_to_node(3, node2).unwrap();
        let node4 = tree.add_child_to_node(4, 0).unwrap();
        let node5 = tree.add_child_to_node(5, node3).unwrap();

        let path1 = tree.path_to_node(node5);
        let path2 = tree.path_to_node(node4);
        assert_eq!(path1, Some(vec![0, node1, node2, node3, node5]));
        assert_eq!(path2, Some(vec![0, node4]));
        assert_eq!(tree.leaves(), vec![node4, node5]);
    }
}
