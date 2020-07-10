#[derive(Debug)]
struct BTreeNode {
    left_node: Box<Option<BTreeNode>>,
    right_node: Box<Option<BTreeNode>>,
    
    key: u32,
    val: String,
}

impl Clone for BTreeNode {
    fn clone(&self) -> Self {
        BTreeNode {
            left_node: self.left_node.clone(),
            right_node: self.right_node.clone(),

            key: self.key,
            val: self.val.clone(),
        }
    }
}

impl BTreeNode {
    fn new(key: u32, val: String) -> BTreeNode {
        BTreeNode {
            left_node: Box::new(None),
            right_node: Box::new(None),

            key: key,
            val: val,
        }
    }
}

// ノードを追加する
fn insert(node: Box<Option<BTreeNode>>, key: u32, val: String) -> Box<Option<BTreeNode>> {
    return match *node {
        None => Box::new(Some(BTreeNode::new(key, val))),
        Some(v) => {
            let mut target_node = v.clone();
            if key < target_node.key {
                target_node.left_node = insert(target_node.left_node, key, val);
            }
            else {
                target_node.right_node = insert(target_node.right_node, key, val);
            }
            Box::new(Some(target_node))
        }
    };
}

// ノードを検索する
fn find(node: &Box<Option<BTreeNode>>, key: u32) -> String {
    return match &**node {
        None => "No results".to_string(),
        Some(v) => {
            if key == v.key {
                v.val.clone()
            }
            // ノードの左側を検索する
            else if key < v.key {
                find(&v.left_node, key)
            }
            // ノードの右側を検索する
            else {
                find(&v.right_node, key)
            }
        }
    }
}

fn main() {
}

#[test]
fn test_find() {
    let mut tree = insert(Box::new(None), 1234, "aaa".to_string());
    tree = insert(tree, 1233, "bbb".to_string());
    tree = insert(tree, 1235, "ccc".to_string());

    assert_eq!("aaa", find(&tree, 1234));
    assert_eq!("bbb", find(&tree, 1233));
    assert_eq!("ccc", find(&tree, 1235));
}