#[derive(Debug)]
enum BTreeNode {
    Operator {
        left_node: Box<BTreeNode>,
        right_node: Box<BTreeNode>,
    },
    Number {
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

fn new_node_num(val: u32) -> Box<BTreeNode> {
    let node = BTreeNode::Number {
        val: val,
    };
    let node = Box::new(node);
    node
}

fn main() {
    
}
