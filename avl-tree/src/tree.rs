use crate::node::{insert, Node};

pub struct Tree {
    root: Option<Box<Node>>,
}

impl Tree {
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn insert(&mut self, key: i32, value: String) {
        match self.root.take() {
            Some(root) => self.root = insert(root, key, value),
            None => self.root = Some(Box::new(Node::new(key, value))),
        }
    }

    pub fn search(&self, target: i32) -> Option<&String> {
        self.root.as_ref().map_or(None, |n| n.search(target))
    }
}

#[test]
fn test_binary_search() {
    let mut tree = Tree::new();
    tree.insert(1, String::from("123"));
    tree.insert(2, String::from("456"));
    tree.insert(3, String::from("789"));
    tree.insert(4, String::from("321"));
    tree.insert(5, String::from("654"));
    tree.insert(6, String::from("987"));
    tree.insert(7, String::from("213"));
    tree.insert(8, String::from("546"));
    tree.insert(9, String::from("879"));
    tree.insert(10, String::from("312"));
    tree.insert(11, String::from("645"));
    tree.insert(11, String::from("978"));
    tree.insert(12, String::from("132"));
    tree.insert(13, String::from("465"));
    tree.insert(14, String::from("798"));
    tree.insert(15, String::from("231"));

    crate::node::preorder_traversal(&tree.root);
}
