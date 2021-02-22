use ::linkedlists::bst::BST;
fn main() {
    let mut tree = BST::new();
    tree.insert(10);
    assert_eq!(tree.search(&10), true);
    tree.insert(12);
    assert_eq!(tree.search(&12), true);
    tree.insert(8);
    assert_eq!(tree.search(&8), true);
    assert_eq!(tree.search(&11), false);
    tree.insert(20);
    assert_eq!(tree.search(&20), true);
    tree.inorder();
    tree.pre_order();
    tree.post_order();
    println!("{:?}", tree.to_vec_as_ref().unwrap());
}
