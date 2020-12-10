use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|el| el.parse().unwrap()).collect()
}

#[aoc(day10, part1)]
fn part1(input: &[usize]) -> usize {
    let mut input = input.to_vec();
    input.sort_unstable();
    let result = input.windows(2).fold(vec![1; 3], |mut acc, x| {
        acc[x[1] - x[0] - 1] += 1;
        acc
    });
    result[0] * result[2]
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct NodeId(usize);

#[derive(Debug)]
struct Node {
    id: NodeId,
    children: [Option<NodeId>; 3],
}

type Graph = HashMap<NodeId, Node>;

impl Node {
    fn paths(&self, graph: &Graph, mut cache: &mut HashMap<NodeId, usize>) -> usize {
        let mut sum = 0;
        if self.children.iter().all(Option::is_none) {
            sum += 1;
        } else {
            for child in self.children.iter().filter_map(Option::as_ref) {
                if let Some(cached) = cache.get(child) {
                    sum += cached;
                } else {
                    sum += graph[child].paths(&graph, &mut cache);
                }
            }
        }
        cache.insert(self.id, sum);
        sum
    }
}

/// Return a child node ID if it should be included, otherwise None.
fn child_if_included(node_id: NodeId, candidate: usize) -> Option<NodeId> {
    if candidate - node_id.0 <= 3 {
        Some(NodeId(candidate))
    } else {
        None
    }
}

/// Insert the final window into the graph.
fn insert_last(graph: &mut Graph, window: &[usize]) {
    graph.insert(
        NodeId(window[0]),
        Node {
            id: NodeId(window[0]),
            children: [
                child_if_included(NodeId(window[0]), window[1]),
                child_if_included(NodeId(window[0]), window[2]),
                None,
            ],
        },
    );
    graph.insert(
        NodeId(window[1]),
        Node {
            id: NodeId(window[1]),
            children: [child_if_included(NodeId(window[1]), window[2]), None, None],
        },
    );
    graph.insert(
        NodeId(window[2]),
        Node {
            id: NodeId(window[2]),
            children: [None, None, None],
        },
    );
}

#[aoc(day10, part2)]
fn part2(input: &[usize]) -> usize {
    // Represent adapters as nodes in graph; count number of paths from
    // start to finish. Can then use DFS with memoization.
    // Turns out there's a much sneakier way to do it, from
    // https://old.reddit.com/r/rust/comments/ka9nre/advent_of_code_2020_day_10/gf9gtnk/
    // For posterity (requires `#![feature(array_windows)]`):
    //
    //     let mut input = input.to_vec();
    //     input.push(0);
    //     input
    //         .array_windows()
    //         .collect::<Vec<_>>()
    //         .split(|[a, b]| b - a == 3)
    //         .map(|x| match x.len() {
    //             4 => 7,
    //             3 => 4,
    //             2 => 2,
    //             _ => 1,
    //         })
    //         .product()
    //
    // This DFS solution runs in 15 microseconds so isn't too bad.
    // The above solution runs in 350 ns so is much much faster!

    // Collect, add the initial node, sort, then add the last node.
    let mut input = input.to_vec();
    input.push(0);
    input.sort_unstable();
    input.push(input[input.len() - 1] + 3);

    // Create the graph by iterating over windows of length 4.
    let mut graph: HashMap<_, _> = input
        .windows(4)
        .map(|window| {
            let id = NodeId(window[0]);
            let node = Node {
                id,
                children: [
                    child_if_included(id, window[1]),
                    child_if_included(id, window[2]),
                    child_if_included(id, window[3]),
                ],
            };
            (id, node)
        })
        .collect();
    // Need to manually add the last set of nodes, annoyingly.
    let last = &input[(input.len() - 3)..(input.len())];
    insert_last(&mut graph, last);

    let mut cache = HashMap::with_capacity(graph.len());
    graph[&NodeId(0)].paths(&graph, &mut cache)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&parse_input(
                "16
10
15
5
1
11
7
19
6
12
4"
            )),
            7 * 5
        );
        assert_eq!(
            part1(&parse_input(
                "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
            )),
            22 * 10
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&parse_input(
                "16
10
15
5
1
11
7
19
6
12
4"
            )),
            8
        );
        assert_eq!(
            part2(&parse_input(
                "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
            )),
            19208
        );
    }
}
