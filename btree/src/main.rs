#[derive(Debug)]
enum BTreeNode {
    Operator {
        left_node: Box<Option<BTreeNode>>,
        right_node: Box<Option<BTreeNode>>,
    },
    Number {
        key: u32,
        val: String,
    },
}

impl BTreeNode {
    fn new(key: u32, val: String) -> BTreeNode {
        BTreeNode::Operator {
            left_node: Box::new(None),
            right_node: Box::new(None),
        };

        BTreeNode::Number {
            key: key,
            val: val,
        }
    }
}

fn insert(node: BTreeNode, key: u32, val: String) {
    
}

fn find(key: u32) -> String {
    "".to_string()
}

fn main() {
    
}

#[test]
fn test_find() {
}