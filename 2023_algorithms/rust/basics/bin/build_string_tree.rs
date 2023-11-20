use basics::tree::{traversal::breath_first_search, tree::build_string_tree};

fn main() {
    let tree = build_string_tree();

    println!("==> Tree: {:?}", tree);
    println!("==> breath_first_search: {:?}", breath_first_search(&tree.root));
}
