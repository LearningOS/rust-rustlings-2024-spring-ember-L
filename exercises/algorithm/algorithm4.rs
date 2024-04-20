/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord + Copy,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + Copy + Debug,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        if self.root.is_none() {
            // 如果树为空，则将新值作为根节点
            self.root = Some(Box::new(TreeNode::new(value)));
            return;
        }

        let mut cur = self.root.as_mut().unwrap();
        loop {
            if value < cur.value {
                // 如果插入值小于等于当前节点的值，则往左子树插入
                if let Some(ref mut left) = cur.left {
                    cur = left;
                } else {
                    // 如果左子树为空，则将新值插入为左子节点
                    cur.left = Some(Box::new(TreeNode::new(value)));
                    break;
                }
            } else if value > cur.value{
                // 如果插入值大于当前节点的值，则往右子树插入
                if let Some(ref mut right) = cur.right {
                    cur = right;
                } else {
                    // 如果右子树为空，则将新值插入为右子节点
                    cur.right = Some(Box::new(TreeNode::new(value)));
                    break;
                }
            } else {
                break;
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool
    where T: Clone
    {
        //TODO
        if self.root.is_none() {
            return false;
        }
        let mut cur = self.root.as_ref().unwrap();
        // 循环查找，越过叶节点后跳出
        loop {
            if value < cur.value {
                if let Some(ref left) = cur.left {
                    cur = left
                } else {
                    break;
                }
            } else if value > cur.value {
                if let Some(ref right) = cur.right {
                    cur = right
                } else {
                    break;
                }
            } else {
                return true;
            }
        }
        false
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    // fn insert(&mut self, value: T) 
    // {
    //     //TODO
    // }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


