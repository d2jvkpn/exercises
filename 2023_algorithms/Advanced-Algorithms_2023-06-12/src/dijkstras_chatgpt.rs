// author: ChatGPT
use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    // target node id
    id: usize,
    distance: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// result.0: taget_id=index, distance=value
// result.1: steps
fn dijkstra(graph: &Vec<Vec<(usize, i32)>>, start: usize) -> (Vec<i32>, usize) {
    let mut distances = vec![i32::max_value(); graph.len()];
    distances[start] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(Node { id: start, distance: 0 });

    let mut steps = 0;

    while let Some(Node { id, distance }) = heap.pop() {
        if distance > distances[id] {
            continue;
        }

        for &(to, weight) in &graph[id] {
            let next_distance = distance + weight;
            if next_distance < distances[to] {
                steps += 1;
                distances[to] = next_distance;
                heap.push(Node { id: to, distance: next_distance });
            }
        }
    }

    (distances, steps)
}

// $ cargo test --lib -- t_dijkstra --nocapture
mod test {
    use super::*;

    #[test]
    fn t_dijkstra() {
        let graph = vec![
            vec![(1, 7), (2, 9), (5, 14)],
            vec![(0, 7), (2, 10), (3, 15)],
            vec![(0, 9), (1, 10), (3, 11), (5, 2)],
            vec![(1, 15), (2, 11), (4, 6)],
            vec![(3, 6), (5, 9)],
            vec![(0, 14), (2, 2), (4, 9)],
        ];

        //
        let start_node = 0;
        let distances = dijkstra(&graph, start_node);

        for (i, distance) in distances.0.iter().enumerate() {
            if i != start_node {
                // println!("Distance from node {} to start node {}: {:?}", i, start_node, distance);
                println!("Distance {} -> {}: {:?}", start_node, i, distance);
            }
        }

        //
        let start_node = 2;
        let distances = dijkstra(&graph, start_node);

        for (i, distance) in distances.0.iter().enumerate() {
            if i != start_node {
                println!("Distance {} -> {}: {:?}", start_node, i, distance);
            }
        }
    }
}
