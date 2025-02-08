use std::collections::VecDeque;

struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Clone> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    // pre-order Travesal
    fn pre_order_trav(root: Option<&TreeNode<T>>, result: &mut Vec<T>) {
        // root -> left -> right
        if let Some(node) = root {
            result.push(node.value.clone()); // visit root
            Self::pre_order_trav(node.left.as_deref(), result); // left
            Self::pre_order_trav(node.right.as_deref(), result); // right
        }
    }

    // In-order Travesal
    fn in_order_trav(root: Option<&TreeNode<T>>, result: &mut Vec<T>) {
        // Left -> root -> right
        if let Some(node) = root {
            Self::in_order_trav(node.left.as_deref(), result); // left
            result.push(node.value.clone()); // visit root
            Self::in_order_trav(node.right.as_deref(), result); // right
        }
    }

    // Post-order Travesal
    fn post_order_trav(root: Option<&TreeNode<T>>, result: &mut Vec<T>) {
        // Left -> right -> root
        if let Some(node) = root {
            Self::post_order_trav(node.left.as_deref(), result); // left
            Self::post_order_trav(node.right.as_deref(), result); // right
            result.push(node.value.clone()); // visit root
        }
    }

    // BFS(BREADTH-FIRST SEARCH)
    fn bfs_trav(root: Option<&TreeNode<T>>, result: &mut Vec<T>) {
        let mut queue = VecDeque::new();
   
        if let Some(node) = root {
            queue.push_back(node);
        }

        while let Some(node) = queue.pop_front() {
            result.push(node.value.clone());

            if let Some(left) = &node.left {
                queue.push_back(left.as_ref());
            }

            if let Some(right) = &node.right {
                queue.push_back(right.as_ref());
            }
        }


    }
}

pub fn build_a_tree() {
    let mut root = TreeNode::new(10);
    root.left = Some(Box::new(TreeNode::new(20)));
    root.right = Some(Box::new(TreeNode::new(30)));
    root.left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(40)));
    root.left.as_mut().unwrap().right = Some(Box::new(TreeNode::new(50)));

    let mut result = Vec::new();
    TreeNode::pre_order_trav(Some(&root), &mut result);
    println!("pre-order: {:?}", result);

    result.clear();
    TreeNode::in_order_trav(Some(&root), &mut result);
    println!("In-order: {:?}", result);

    result.clear();
    TreeNode::post_order_trav(Some(&root), &mut result);
    println!("post-order: {:?}", result);

    result.clear();
    TreeNode::bfs_trav(Some(&root), &mut result);
    println!("bfs: {:?}", result);
    //      root
    //       10
    //      /  \
    //     20   30
    //    / \
    //   40  50
}
