// 参考
// https://qiita.com/0yoyoyo/items/96705669f47ff8f9b70d
#[derive(Debug)]
enum BTreeNode {
    Operator {
        left_node: Box<BTreeNode>,
        right_node: Box<BTreeNode>,
    },
    Number {
        key: u32,
        val: u32,
    },
}

fn new_node(left_node: Box<BTreeNode>, right_node: Box<BTreeNode>) -> Box<BTreeNode> {
    let node = BTreeNode::Operator {
        left_node: left_node,
        right_node: right_node,
    };
    let node = Box::new(node);
    node
}

fn new_node_num(key: u32, val: u32) -> Box<BTreeNode> {
    let node = BTreeNode::Number {
        key: key,
        val: val,
    };
    let node = Box::new(node);
    node
}

fn main() {
    
}
