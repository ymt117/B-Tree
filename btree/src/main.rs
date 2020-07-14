#[derive(Debug)]
struct BTreeNode {
    left:   Box<Option<BTreeNode>>,
    right:  Box<Option<BTreeNode>>,
    
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

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone();
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
        Some(node) => format!("Found key: {} val: {}", node.key, node.val)
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

fn remove(node: Box<Option<BTreeNode>>, key: u32) -> Box<Option<BTreeNode>> {
    return match *node {
        None => Box::new(None),
        Some(v) => {
            let target_node = v.clone();
            remove_node(Box::new(Some(target_node)), v, false, key)
        }
    }
}

// ノードを削除する
fn remove_node(node: Box<Option<BTreeNode>>, mut parent: BTreeNode, is_left: bool, key: u32) -> Box<Option<BTreeNode>> {
    // 削除のパターン
    // (1) 削除対象のノードが子を持たない
    //     → 削除対象のノードをNoneにする
    // (2) 削除対象のノードが1つの子を持つ
    //     → 削除対象のノードをノードが持つ子で置き換える
    // (3) 削除対象のノードが2つの子を持つ
    //     → 削除対象のノードを通りがかり順で次にくるノード(X)で置き換える
    //       そのあとXは削除する
    return match *node {
        None => Box::new(None),
        Some(v) => {
            let mut target_node = v.clone();
            // 削除対象のノードを検索
            if key == target_node.key {
                // 削除する処理を書く
                // (1)の場合
                if target_node.left.is_none() && target_node.right.is_none() {
                    // target_nodeをNoneで置き換える
                    if is_left {
                        parent.left = Box::new(None);
                        match *parent.left {
                            None => return Box::new(None),
                            _ => panic!()
                        }
                    }
                    else {
                        parent.right = Box::new(None);
                        match *parent.right {
                            None => return Box::new(None),
                            _ => panic!()
                        }
                    }
                }
                // (2)の場合
                else if target_node.left.is_none() {
                    // target_nodeをtaget_nodeの子(right)で置き換える
                    let child_node = target_node.clone();
                    match &*child_node.right {
                        None => panic!(),
                        Some(x) => target_node.clone_from(&x),
                    }
                }
                else if target_node.right.is_none() {
                    // target_nodeをtarget_nodeの子（left）で置き換える
                    let child_node = target_node.clone();
                    match &*child_node.left {
                        None => panic!(),
                        Some(x) => target_node.clone_from(&x),
                    }
                }
                // (3)の場合
                else {

                }
            }
            // 削除対象のノードが現在のノードより左側にあるとき
            else if key < target_node.key {
                // 左側の子ノードを引数としてremove関数を再帰的に呼ぶ
                let parent = target_node.clone();
                target_node.left = remove_node(target_node.left, parent, true, key);
            }
            // 削除対象のノードが現在のノードより右側にあるとき
            else {
                let parent = target_node.clone();
                target_node.right = remove_node(target_node.right, parent, false, key);
            }
            Box::new(Some(target_node))
        }
    }
}

fn main() {
    // テスト用ツリーを作成
    let foo = vec![4, 10, 2, 5, 1, 3, 8, 11, 7, 9, 15];
    let mut tree = insert(Box::new(None), 6, "6".to_string());
    for i in foo {
        tree = insert(tree, i, i.to_string());
    }
    //println!("{:#?}", tree);

    //println!("Remove tree");
    tree = remove(tree, 1);
    println!("{:#?}", tree);
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