use avl_tree::tree::Tree;

fn main() {
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

    let result = tree.search(15);
    match result {
        None => println!("Not found!"),
        Some(s) => println!("{}", s),
    }
}
