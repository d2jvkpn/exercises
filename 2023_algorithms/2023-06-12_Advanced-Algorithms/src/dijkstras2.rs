use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Edge {
    from: char,
    to: char,
    distance: u32,
}

impl Edge {
    pub fn new(from: char, to: char, distance: u32) -> Self {
        Self { from, to, distance }
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(edges: &[Edge], start: char, directed: bool) -> HashMap<char, u32> {
    let mut distances = HashMap::new();
    _ = distances.insert(start, 0);

    let mut heap = BinaryHeap::new();
    heap.push(Edge { from: start, to: start, distance: 0 });

    while let Some(Edge { from: _, to, distance }) = heap.pop() {
        let current = distances.entry(to).or_insert_with(|| u32::max_value());

        if &distance > current {
            continue;
        }

        for edge in find_edges(edges, to, directed) {
            let next_dist = distance + edge.distance;
            let next_to = edge.to;

            let current = distances.entry(next_to).or_insert_with(|| u32::max_value());

            if next_dist < *current {
                distances.insert(next_to, next_dist);
                heap.push(Edge { from: start, to: next_to, distance: next_dist });
            }
        }
    }

    distances
}

fn find_edges(edges: &[Edge], from: char, directed: bool) -> Vec<Edge> {
    let mut vec = vec![];

    edges.iter().for_each(|v| {
        if v.from == from {
            vec.push(Edge { from, to: v.to, distance: v.distance });
        } else if !directed && v.to == from {
            vec.push(Edge { from, to: v.from, distance: v.distance });
        }
    });

    vec
}

// $ cargo test --lib -- dijkstras2::tests::t_dijkstra --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_dijkstra() {
        let edges = vec![
            Edge::new('A', 'B', 7),
            Edge::new('A', 'C', 9),
            Edge::new('A', 'F', 14),
            Edge::new('B', 'C', 10),
            Edge::new('B', 'D', 15),
            Edge::new('C', 'D', 11),
            Edge::new('C', 'F', 2),
            Edge::new('D', 'E', 6),
            Edge::new('E', 'F', 9),
        ];

        let start_node = 'A';
        let distances = dijkstra(&edges, start_node, false);

        println!(">>>");
        // dbg!(&nodes);
        for (to, value) in &distances {
            println!("Distance {} -> {}: {:?}", start_node, to, value);
        }

        assert_eq!(distances.get(&'D'), Some(&20));
        assert_eq!(distances.get(&'E'), Some(&20));
        assert_eq!(distances.get(&'F'), Some(&11));
    }
}
