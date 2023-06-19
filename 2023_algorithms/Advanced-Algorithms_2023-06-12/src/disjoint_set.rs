use std::collections::HashMap;

struct DisjointSet {
    parent: HashMap<i32, i32>,
}

impl DisjointSet {
    fn new() -> DisjointSet {
        DisjointSet { parent: HashMap::new() }
    }

    fn find(&self, x: i32) -> i32 {
        match self.parent.get(&x) {
            None => x,
            Some(&y) => self.find(y),
        }
    }

    fn union(&mut self, x: i32, y: i32) -> bool {
        let x_root = self.find(x);
        let y_root = self.find(y);

        if x_root == y_root {
            return false;
        }

        if x_root < y_root {
            self.parent.insert(y_root, x_root);
        } else {
            self.parent.insert(x_root, y_root);
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_disjoint_set() {
        let mut dsu = DisjointSet::new();
        assert!(dsu.union(1, 2));
        assert!(dsu.union(2, 3));

        assert_eq!(dsu.find(1), 1);
        assert_eq!(dsu.find(2), 1);
        assert_eq!(dsu.find(3), 1);

        assert_eq!(dsu.find(1), dsu.find(3));
    }
}
