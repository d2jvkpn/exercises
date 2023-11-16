#[cfg(test)]
mod tests {
    use super::super::{binary_tree::Tree, node::Node, traversal::*};

    #[test]
    fn t1_binary_tree() {
        let mut tree = Tree::new();
        tree.push(10);
        println!("tree: {:?}", tree);

        tree.push(5).push(1);
        tree.push(12);
        tree.push(4).push(6).push(8);

        assert_eq!(tree.count(), 7);
        assert!(tree.remove(1));
        assert!(!tree.remove(13));
        assert_eq!(tree.count(), 6);
        println!("tree: {:?}", tree);

        // println!("tree.bfs: {:?}", tree.bfs_vector());
        assert_eq!(tree.bfs_vector(), vec![10, 5, 12, 4, 6, 8]);
    }

    #[test]
    fn t2_binary_tree() {
        let mut tree: Tree<usize>;
        let mut slice: &[usize];
        tree = Tree::new();

        // tree.push(8).push(3).push(10).push(1).push(6).push(14).push(4).push(7).push(13).push(19);
        tree.clear();
        slice = &[8, 3, 10, 1, 6, 14, 4, 7, 13, 19];
        tree.push_slice(slice);
        // println!("==> bfs: {:?}", tree.bfs());
        assert_eq!(tree.bfs_vector(), vec![8, 3, 10, 1, 6, 14, 4, 7, 13, 19]);
        assert_eq!(tree.get_range(10, 30), vec![10, 13, 14, 19]);

        print!("==> inorder_do print: ");
        tree.inorder(|v: &usize| print!("{}, ", v));
        println!("");

        assert_eq!(tree.count(), slice.len());
        assert_eq!(tree.root_value(), Some(8));

        tree.remove(8);
        assert_eq!(tree.count(), slice.len() - 1);
        assert_eq!(tree.root_value(), Some(10));
        assert_eq!(inorder_recur_a(&tree.root), vec![1, 3, 4, 6, 7, 10, 13, 14, 19]);
        // println!("==> 1. {:?}, remove {}, inorder: {:?}", slice, 8, inorder_recur_a(&tree.root));

        tree.clear();
        slice = &[8, 10, 14];
        tree.push_slice(slice);
        tree.remove(8);
        assert_eq!(tree.count(), slice.len() - 1);
        assert_eq!(tree.root_value(), Some(10));
        assert_eq!(inorder_recur_a(&tree.root), vec![10, 14]);
        // println!("==> 2. {:?}, remove {}, inorder: {:?}", slice, 8, inorder_recur_a(&tree.root));

        tree.clear();
        slice = &[8, 10, 14];
        tree.push_slice(slice);
        tree.remove(10);
        assert_eq!(tree.count(), slice.len() - 1);
        assert_eq!(tree.root_value(), Some(8));
        assert_eq!(inorder_recur_a(&tree.root), vec![8, 14]);
        // println!("==> 3. {:?}, remove {}, inorder: {:?}", slice, 10, inorder_recur_a(&tree.root));

        tree.clear();
        slice = &[8];
        tree.push_slice(slice);
        tree.remove(8);
        assert_eq!(tree.count(), slice.len() - 1);
        assert_eq!(tree.root_value(), None);
        assert_eq!(inorder_recur_a(&tree.root), vec![]);
        // println!("==> 4. {:?}, remove {}, inorder: {:?}", slice, 8, inorder_recur_a(&tree.root));

        tree.clear();
        slice = &[8, 3];
        tree.push_slice(slice);
        tree.remove(8);
        assert_eq!(tree.count(), slice.len() - 1);
        assert_eq!(tree.root_value(), Some(3));
        assert_eq!(inorder_recur_a(&tree.root), vec![3]);
        // println!("==> 5. {:?}, remove {}, inorder: {:?}", slice, 8, inorder_recur_a(&tree.root));
    }

    #[test]
    fn t3_binary_tree() {
        let mut tree: Tree<usize> = Tree::new();
        let slice: &[usize] = &[8, 10, 14];

        tree.push_slice(slice);

        let _: Node<usize> = Node::new(42);
    }
}
