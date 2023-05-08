struct TreeNode<T> {
    pub left: Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>,
    pub key: T,
}

impl<T> TreeNode<T> {
    pub fn new(key: T) -> Self {
        TreeNode {
            left: None,
            right: None,
            key,
        }
    }
    pub fn left(mut self, node: TreeNode<T>) -> Self {
        self.left = Some(Box::new(node));
        self
    }
    pub fn right(mut self, node: TreeNode<T>) -> Self {
        self.right = Some(Box::new(node));
        self
    }
}

fn main() {
    let node: TreeNode<i32> = TreeNode::new(1)
        .left(TreeNode::new(2))
        .right(TreeNode::new(3));

    assert!(node.key == 1);

    match node.left {
        Some(node) => println!("node left value: {}", node.key),
        None => println!("node is empty"),
    }
}
