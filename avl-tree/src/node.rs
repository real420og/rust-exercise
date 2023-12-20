use std::cmp::{max, Ordering};

pub struct Node {
    key: i32,
    value: String,
    height: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(key: i32, value: String) -> Self {
        Node {
            key,
            value,
            height: 1,
            left: None,
            right: None,
        }
    }

    fn height_update(&mut self) {
        self.height = 1 + max(self.height_of(&self.left), self.height_of(&self.right))
    }

    fn balance_factor(&self) -> i32 {
        self.height_of(&self.left) - self.height_of(&self.right)
    }

    fn height_of(&self, node: &Option<Box<Node>>) -> i32 {
        node.as_ref().map_or(0, |n| n.height)
    }

    pub(crate) fn search(&self, target: i32) -> Option<&String> {
        match target.cmp(&self.key) {
            Ordering::Less => self.left.as_ref().map_or(None, |n| n.search(target)),
            Ordering::Greater => self.right.as_ref().map_or(None, |n| n.search(target)),
            Ordering::Equal => Some(&self.value),
        }
    }
}

pub fn insert(mut root: Box<Node>, key: i32, value: String) -> Option<Box<Node>> {
    match key.cmp(&root.key) {
        Ordering::Less => {
            root.left = insert_node(root.left.take(), key, value);
        }
        Ordering::Greater => {
            root.right = insert_node(root.right.take(), key, value);
        }
        Ordering::Equal => (),
    }

    root.height_update();

    let balance = root.balance_factor();

    if balance == 2 && key < root.left.as_ref().unwrap().key {
        return Some(right_rotate(root));
    }

    if balance == 2 && key > root.left.as_ref().unwrap().key {
        return Some(left_right_rotate(root));
    }

    if balance == -2 && key > root.right.as_ref().unwrap().key {
        return Some(left_rotate(root));
    }

    if balance == -2 && key < root.right.as_ref().unwrap().key {
        return Some(right_left_rotate(root));
    }

    Some(root)
}

fn right_left_rotate(mut pivot: Box<Node>) -> Box<Node> {
    pivot.right = Some(right_rotate(pivot.right.take().unwrap()));
    return left_rotate(pivot);
}

fn right_rotate(mut pivot: Box<Node>) -> Box<Node> {
    let mut root = pivot.left.take().unwrap();

    pivot.left = root.right.take();
    pivot.height_update();

    root.right = Some(pivot);
    root.height_update();

    root
}

fn left_right_rotate(mut pivot: Box<Node>) -> Box<Node> {
    pivot.left = Some(left_rotate(pivot.left.take().unwrap()));
    return right_rotate(pivot);
}

fn left_rotate(mut pivot: Box<Node>) -> Box<Node> {
    let mut root = pivot.right.take().unwrap();

    pivot.right = root.left.take();
    pivot.height_update();

    root.left = Some(pivot);
    root.height_update();

    root
}

fn insert_node(node: Option<Box<Node>>, key: i32, value: String) -> Option<Box<Node>> {
    match node {
        None => Some(Box::new(Node::new(key, value))),
        Some(n) => insert(n, key, value),
    }
}

pub fn preorder_traversal(node: &Option<Box<Node>>) {
    if let Some(n) = node {
        println!("{} {}", n.height, n.key);
        preorder_traversal(&n.left);
        preorder_traversal(&n.right);
    }
}
