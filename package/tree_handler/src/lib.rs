use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

/*
 * 树工具
 */
pub struct TreeHandler<'a> {
    roots: Vec<TreeNodeHandler>,
    // 是否能够用&str当key
    parent_map: HashMap<String, &'a mut TreeNodeHandler>,
}

impl TreeHandler<'_> {
    pub fn new<'a>() -> TreeHandler<'a> {
        TreeHandler {
            roots: vec![], /*  */
            parent_map: HashMap::new(),
        }
    }

    /**
     * 添加根节点
     */
    pub fn add_root<'a>(&'a mut self, node: TreeNodeHandler) {
        self.roots.push(node);
        let mut node = self.roots.last_mut().unwrap();
        let a = node.id.clone();
        let ab = node;
        self.parent_map.insert(node.id.clone(), node);
    }

    /**
     * 获取可操作引用的节点
     */
    pub fn get_mut_unwrap(&mut self, id: &str) -> &mut TreeNodeHandler {
        let node = self.parent_map.get_mut(id);
        node.unwrap()
    }
}

/*
 * 树节点操作
 */
#[derive(Debug)]
pub struct TreeNodeHandler {
    id: String,
    label: String,
}

impl TreeNodeHandler {
    pub fn new(id: String, label: String) -> TreeNodeHandler {
        TreeNodeHandler { id, label }
    }
}

#[test]
fn test() {
    let mut tree_handler = TreeHandler::new();
    tree_handler.add_root(TreeNodeHandler::new(
        "rademo".to_string(),
        "rademo".to_string(),
    ));

    let mut root = tree_handler.get_mut_unwrap("rademo");

    println!("{:?}", root);
    root.label = "测试修改".to_string();
    println!("{:?}", root);
}
