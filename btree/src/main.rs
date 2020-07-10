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

fn find(key: u32) -> String {
    "".to_string()
}

fn main() {
    let tree = BTreeNode::new(1234, "aaa".to_string());
    println!("{:#?}", tree);
}

#[test]
fn test_insert() {

}

#[test]
fn test_find() {
}