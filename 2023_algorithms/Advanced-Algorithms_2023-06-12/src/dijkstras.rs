use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Edge {
    to: char,
    distance: u32,
}

impl Edge {
    pub fn new(to: char, distance: u32) -> Self {
        Self { to, distance }
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

// result.0: taget_id=index, distance=value
// result.1: steps
fn dijkstra(graph: &mut HashMap<char, Vec<Edge>>, start: char) -> HashMap<char, u32> {
    let mut distances = HashMap::with_capacity(graph.len());
    _ = distances.insert(start, 0);

    let mut heap = BinaryHeap::new();
    heap.push(Edge { to: start, distance: 0 });

    while let Some(Edge { to, distance }) = heap.pop() {
        let current = distances.entry(to).or_insert_with(|| u32::max_value());

        if &distance > current {
            continue;
        }

        for edge in graph.get(&to).unwrap_or(&vec![]) {
            let next_dist = distance + edge.distance;
            let next_to = edge.to;

            let current = distances.entry(next_to).or_insert_with(|| u32::max_value());

            if next_dist < *current {
                distances.insert(next_to, next_dist);
                heap.push(Edge { to: next_to, distance: next_dist });
            }
        }
    }

    distances
}

pub fn nodes2graph(nodes: &[(char, Vec<Edge>)]) -> HashMap<char, Vec<Edge>> {
    let mut grap = HashMap::new();

    for (node, edges) in nodes {
        edges.iter().for_each(|v| {
            let values = grap.entry(v.to).or_insert_with(|| vec![]);

            if values.iter().find(|v: &&Edge| &v.to == node).is_none() {
                values.push(Edge { to: *node, distance: v.distance });
            }
        });

        let node_values = grap.entry(*node).or_insert_with(|| vec![]);

        edges.iter().for_each(|v| {
            if node_values.iter().find(|v: &&Edge| v.to == *node).is_none() {
                node_values.push(Edge { to: v.to, distance: v.distance });
            }
        });
    }

    grap
}

// $ cargo test --lib -- dijkstras::tests::t_dijkstra --nocapture
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    #[should_panic]
    fn t_hashmap() {
        let mut map = HashMap::new();
        map.insert('a', 1);
        map.insert('b', 2);

        assert_eq!(map[&'a'], 1);
        assert_eq!(map[&'c'], 1); // panicked at 'no entry found for key'
    }

    #[test]
    fn t_dijkstra() {
        //
        let mut graph = HashMap::from([
            ('A', vec![Edge::new('B', 7), Edge::new('C', 9), Edge::new('F', 14)]),
            ('B', vec![Edge::new('C', 10), Edge::new('D', 15)]),
            ('C', vec![Edge::new('D', 11), Edge::new('F', 2)]),
            ('D', vec![Edge::new('B', 15), Edge::new('C', 11), Edge::new('E', 6)]),
            ('E', vec![Edge::new('D', 6), Edge::new('F', 9)]),
            ('F', vec![Edge::new('A', 14), Edge::new('C', 2), Edge::new('E', 9)]),
        ]);

        let start_node = 'A';
        let distances = dijkstra(&mut graph, start_node);

        println!(">>>");
        for (to, value) in &distances {
            if to != &start_node {
                println!("Distance {} -> {}: {:?}", start_node, to, value);
            }
        }

        assert_eq!(distances.get(&'D'), Some(&20));
        assert_eq!(distances.get(&'E'), Some(&20));
        assert_eq!(distances.get(&'F'), Some(&11));

        //
        let start_node = 'C';
        let distances = dijkstra(&mut graph, start_node);

        println!(">>>");
        for (to, value) in &distances {
            println!("Distance {} -> {}: {:?}", start_node, to, value);
        }

        assert_eq!(distances.get(&'E'), Some(&11));

        //
        let nodes = vec![
            ('A', vec![Edge::new('B', 7), Edge::new('C', 9), Edge::new('F', 14)]),
            ('B', vec![Edge::new('C', 10), Edge::new('D', 15)]),
            ('C', vec![Edge::new('D', 11), Edge::new('F', 2)]),
            ('D', vec![Edge::new('E', 6)]),
            ('E', vec![Edge::new('F', 9)]),
        ];

        let mut graph = nodes2graph(&nodes);

        let start_node = 'A';
        let distances = dijkstra(&mut graph, start_node);

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
