#[derive(Debug)]
struct BTreeNode {
    left: Box<Option<BTreeNode>>,
    right: Box<Option<BTreeNode>>,
    
    key: u32,
    val: String,
}

impl Clone for BTreeNode {
    fn clone(&self) -> Self {
        BTreeNode {
            left: self.left.clone(),
            right: self.right.clone(),

            key: self.key,
            val: self.val.clone(),
        }
    }
}

impl BTreeNode {
    fn new(key: u32, val: String) -> BTreeNode {
        BTreeNode {
            left: Box::new(None),
            right: Box::new(None),

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
            // キーが存在する場合は上書き
            if key == target_node.key {
                target_node.val = val;
            }
            // ノードの左側にノードを追加
            else if key < target_node.key {
                target_node.left = insert(target_node.left, key, val);
            }
            // ノードの右側にノードを追加
            else {
                target_node.right = insert(target_node.right, key, val);
            }
            Box::new(Some(target_node))
        }
    };
}

// ノードを検索する
fn find(node: &Box<Option<BTreeNode>>, key: u32) -> String {
    let node = find_node(node, key);
    return match *node {
        None => "No results".to_string(),
        Some(node) => node.val
    }
}

fn find_node(node: &Box<Option<BTreeNode>>, key: u32) -> Box<Option<BTreeNode>> {
    return match &**node {
        None => Box::new(None),
        Some(v) => {
            if key == v.key {
                let target_node = v.clone();
                Box::new(Some(target_node))
            }
            // ノードの左側を検索する
            else if key < v.key {
                find_node(&v.left, key)
            }
            // ノードの右側を検索する
            else {
                find_node(&v.right, key)
            }
        }
    }
}

// ノードを削除する
fn delete(node: &Box<Option<BTreeNode>>, key: u32) -> Box<Option<BTreeNode>> {
    // 削除のパターン
    // (1) 削除対象のノードが子を持たない
    //     → 削除対象のノードをNoneにする
    // (2) 削除対象のノードが1つの子を持つ
    //     → 削除対象のノードをノードが持つ子で置き換える
    // (3) 削除対象のノードが2つの子を持つ
    //     → 削除対象のノードを通りがかり順で次にくるノード
    Box::new(None)
}

fn main() {
    let mut tree = insert(Box::new(None), 10, "10".to_string());
    tree = insert(tree, 9, "9".to_string());
    tree = insert(tree, 8, "8".to_string());
    tree = insert(tree, 11, "11".to_string());
    tree = insert(tree, 13, "13".to_string());
    tree = insert(tree, 12, "12".to_string());
    tree = insert(tree, 11, "replace".to_string());

    println!("{:#?}", tree);

    let mut tree = Box::new(Some(BTreeNode::new(1, "abc".to_string())));
    //println!("{:#?}", tree);
    tree = insert(tree, 3, "hello".to_string());
    //println!("{:#?}", tree);
}

#[test]
fn test_find() {
    let mut tree = insert(Box::new(None), 1234, "aaa".to_string());
    tree = insert(tree, 1233, "bbb".to_string());
    tree = insert(tree, 1235, "ccc".to_string());

    assert_eq!("aaa", find(&tree, 1234));
    assert_eq!("bbb", find(&tree, 1233));
    assert_eq!("ccc", find(&tree, 1235));

    assert_eq!("No results", find(&tree, 3333));
}