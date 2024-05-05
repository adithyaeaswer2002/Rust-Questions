use std::cmp;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn max_depth(root: &Option<Box<TreeNode>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let left_depth = max_depth(&root.as_ref().unwrap().left);
    let right_depth = max_depth(&root.as_ref().unwrap().right);

    1 + cmp::max(left_depth, right_depth)
}

fn main() {
    // Construct a binary tree
    let mut root = TreeNode::new(1);
    root.left = Some(Box::new(TreeNode::new(2)));
    root.right = Some(Box::new(TreeNode::new(3)));
    root.left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(4)));
    root.left.as_mut().unwrap().right = Some(Box::new(TreeNode::new(5)));

    let max_depth = max_depth(&Some(Box::new(root)));
    println!("The depth of the binary tree is: {}", max_depth);
}
