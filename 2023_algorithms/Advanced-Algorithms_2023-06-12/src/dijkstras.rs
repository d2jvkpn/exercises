use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Copy, Clone, Eq, PartialEq)]
struct Edge {
    // target Edge id
    to: usize,
    distance: u32,
}

impl Edge {
    pub fn new(to: usize, distance: u32) -> Self {
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
fn dijkstra(graph: &Vec<Vec<Edge>>, start: usize) -> HashMap<usize, u32> {
    let mut distances = HashMap::with_capacity(graph.len());

    for val in graph {
        for (i, v) in val.iter().enumerate() {
            _ = distances.entry(i).or_insert_with(|| u32::max_value());
            _ = distances.entry(v.to).or_insert_with(|| u32::max_value());
        }
    }
    _ = distances.insert(start, 0);

    let mut heap = BinaryHeap::new();
    heap.push(Edge { to: start, distance: 0 });

    while let Some(Edge { to, distance: dist }) = heap.pop() {
        if dist > distances[&to] {
            continue;
        }

        for edge in &graph[to] {
            let next_dist = dist + edge.distance;

            if next_dist < distances[&edge.to] {
                distances.insert(edge.to, next_dist);
                heap.push(Edge { to: edge.to, distance: next_dist });
            }
        }
    }

    distances
}

// $ cargo test --lib -- dijkstras::tests::t_dijkstra --nocapture
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_dijkstra() {
        let graph = vec![
            vec![Edge::new(1, 7), Edge::new(2, 9), Edge::new(5, 14)],
            vec![Edge::new(0, 7), Edge::new(2, 10), Edge::new(3, 15)],
            vec![Edge::new(0, 9), Edge::new(1, 10), Edge::new(3, 11), Edge::new(5, 2)],
            vec![Edge::new(1, 15), Edge::new(2, 11), Edge::new(4, 6)],
            vec![Edge::new(3, 6), Edge::new(5, 9)],
            vec![Edge::new(0, 14), Edge::new(2, 2), Edge::new(4, 9)],
        ];

        //
        let start_node = 0;
        let distances = dijkstra(&graph, start_node);

        for (i, distance) in distances {
            if i != start_node {
                // println!("Distance from node {} to start node {}: {:?}", i, start_node, distance);
                println!("Distance {} -> {}: {:?}", start_node, i, distance);
            }
        }

        //
        let start_node = 2;
        let distances = dijkstra(&graph, start_node);

        for (i, distance) in distances {
            if i != start_node {
                println!("Distance {} -> {}: {:?}", start_node, i, distance);
            }
        }
    }
}
