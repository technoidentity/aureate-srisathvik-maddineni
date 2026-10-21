pub struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

pub struct BinarySearchTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, value: T) {
        let mut current = &mut self.root;

        loop {
            match current {
                None => {
                    *current = Some(Box::new(Node {
                        value,
                        left: None,
                        right: None,
                    }));
                    return;
                }
                Some(node) => match value.cmp(&node.value) {
                    std::cmp::Ordering::Less => {
                        current = &mut node.left;
                    }
                    std::cmp::Ordering::Greater => {
                        current = &mut node.right;
                    }
                    std::cmp::Ordering::Equal => return,
                },
            }
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        let mut current = self.root.as_deref();

        while let Some(node) = current {
            if value < &node.value {
                current = node.left.as_deref();
            } else if value > &node.value {
                current = node.right.as_deref();
            } else {
                return true;
            }
        }

        false
    }

    pub fn inorder(&self) -> Vec<&T> {
        let mut result = Vec::new();
        Self::inorder_node(&self.root, &mut result);
        result
    }

    fn inorder_node<'a>(node: &'a Option<Box<Node<T>>>, result: &mut Vec<&'a T>) {
        if let Some(node) = node {
            Self::inorder_node(&node.left, result);
            result.push(&node.value);
            Self::inorder_node(&node.right, result);
        }
    }

    pub fn preorder(&self) -> Vec<&T> {
        let mut result = Vec::new();
        Self::preorder_node(&self.root, &mut result);
        result
    }

    fn preorder_node<'a>(node: &'a Option<Box<Node<T>>>, result: &mut Vec<&'a T>) {
        if let Some(node) = node {
            result.push(&node.value);
            Self::preorder_node(&node.left, result);
            Self::preorder_node(&node.right, result);
        }
    }

    pub fn postorder(&self) -> Vec<&T> {
        let mut result = Vec::new();
        Self::postorder_node(&self.root, &mut result);
        result
    }

    fn postorder_node<'a>(node: &'a Option<Box<Node<T>>>, result: &mut Vec<&'a T>) {
        if let Some(node) = node {
            Self::postorder_node(&node.left, result);
            Self::postorder_node(&node.right, result);
            result.push(&node.value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inserts_and_searches() {
        let mut tree = BinarySearchTree::new();
        tree.insert(10);
        tree.insert(5);
        tree.insert(15);

        assert!(tree.contains(&10));
        assert!(tree.contains(&5));
        assert!(!tree.contains(&20));
    }

    #[test]
    fn inorder_is_sorted() {
        let mut tree = BinarySearchTree::new();

        for value in [10, 5, 15, 3, 7] {
            tree.insert(value);
        }

        assert_eq!(tree.inorder(), vec![&3, &5, &7, &10, &15]);
    }

    #[test]
    fn traversals_work() {
        let mut tree = BinarySearchTree::new();

        for value in [10, 5, 15] {
            tree.insert(value);
        }

        assert_eq!(tree.preorder(), vec![&10, &5, &15]);
        assert_eq!(tree.postorder(), vec![&5, &15, &10]);
    }

    #[test]
    fn ignores_duplicates() {
        let mut tree = BinarySearchTree::new();
        tree.insert(10);
        tree.insert(10);

        assert_eq!(tree.inorder(), vec![&10]);
    }
}
