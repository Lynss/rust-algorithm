use std::cell::RefCell;
use std::rc::Rc;
use TreeNode;

pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_some() {
        let inner = root.clone().unwrap();
        let mut root_tree = inner.borrow_mut();
        if key < root_tree.val {
            root_tree.left = delete_node(root_tree.left.clone(), key);
        } else if key > root_tree.val {
            root_tree.right = delete_node(root_tree.right.clone(), key);
        } else {
            if root_tree.left.is_none() {
                return root_tree.right.clone();
            } else if root_tree.right.is_none() {
                return root_tree.left.clone();
            } else {
                let find_max = |tree: Rc<RefCell<TreeNode>>| {
                    let mut max = tree.clone();
                    let mut max_val = max.borrow().val;
                    while max.borrow().right.is_some() {
                        let node = max.clone();
                        let node = node.borrow();
                        max = node.right.as_ref().unwrap().clone();
                        max_val = max.borrow().val;
                    }
                    max_val
                };
                let max = find_max(root_tree.left.clone().unwrap());
                root_tree.val = max;
                root_tree.left = delete_node(root_tree.left.clone(), max);
            }
        }
    }
    root
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_node() {
        let a = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
        })));
        let c = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
        })));
        let b = delete_node(a, 3);
        assert_eq!(c, b);
    }
}
