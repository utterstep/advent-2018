use std::{cmp::Reverse, collections::BinaryHeap, num::NonZeroUsize};

use crate::{graph::InstructionGraph, workers::Pool};

const LETTER_A: i32 = 'A' as i32;

/// Compute letter number (only uppercased ASCII)
///
/// ```
/// assert_eq!(letter_number('A'), 1);
/// assert_eq!(letter_number('Z'), 26);
/// ```
fn letter_number(letter: char) -> i32 {
    debug_assert!(letter <= 'Z');
    debug_assert!(letter >= 'A');
    letter as i32 - LETTER_A + 1
}

#[derive(Debug)]
pub(crate) struct PooledTraverser {
    graph: InstructionGraph,
    pool: Pool,
    delta: i32,
    tasks: BinaryHeap<Reverse<(i32, char)>>,
}

impl PooledTraverser {
    pub(crate) fn new(
        graph: InstructionGraph,
        work_price_delta: i32,
        n_workers: NonZeroUsize,
    ) -> Self {
        let tasks = graph
            .get_starting_nodes()
            .map(|node| Reverse((0, node)))
            .collect();

        Self {
            graph,
            pool: Pool::new(n_workers),
            delta: work_price_delta,
            tasks,
        }
    }

    fn work_duration(&self, work: char) -> i32 {
        letter_number(work) + self.delta
    }

    pub(crate) fn graph_finish_time(mut self) -> i32 {
        for (start_at, node) in &mut self {
            // TODO: replace with dbg! after Rust 1.32 release
            println!("{} will start at {}", node, start_at);
        }

        self.pool.free_at()
    }
}

impl Iterator for PooledTraverser {
    type Item = (i32, char);

    fn next(&mut self) -> Option<Self::Item> {
        let to_return = self.tasks.pop().map(|reversed| reversed.0);

        if let Some((start_at, node)) = to_return {
            let finish_time = self.pool.take_work(start_at, self.work_duration(node));

            self.tasks.extend(
                self.graph
                    .visit_node(node)
                    .into_iter()
                    .map(|node| Reverse((finish_time, node))),
            );
        }

        to_return
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_number() {
        assert_eq!(letter_number('A'), 1);
        assert_eq!(letter_number('Z'), 26);
    }

    #[test]
    fn test_order() {
        let mut graph = InstructionGraph::new();

        graph.add_link('C', 'F');
        graph.add_link('C', 'B');
        graph.add_link('B', 'G');

        assert_eq!(
            PooledTraverser::new(graph, 0, NonZeroUsize::new(2).unwrap()).collect::<Vec<_>>(),
            vec![(0, 'C'), (3, 'B'), (3, 'F'), (5, 'G')]
        )
    }

    #[test]
    fn test_graph_finish_time() {
        let mut graph = InstructionGraph::new();

        graph.add_link('C', 'F');
        graph.add_link('C', 'B');
        graph.add_link('B', 'G');

        assert_eq!(
            PooledTraverser::new(graph, 0, NonZeroUsize::new(2).unwrap()).graph_finish_time(),
            12
        );

        let mut graph = InstructionGraph::new();

        graph.add_link('C', 'F');
        graph.add_link('C', 'B');
        graph.add_link('B', 'G');

        assert_eq!(
            PooledTraverser::new(graph, 2, NonZeroUsize::new(2).unwrap()).graph_finish_time(),
            18
        );
    }
}
