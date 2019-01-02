use std::{cmp::Reverse, collections::BinaryHeap};

use crate::graph::InstructionGraph;

#[derive(Debug)]
pub(crate) struct SimpleTraverser {
    graph: InstructionGraph,
    to_iterate: BinaryHeap<Reverse<char>>,
}

impl Iterator for SimpleTraverser {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let to_return = self.to_iterate.pop().map(|reversed| reversed.0);

        if let Some(to_return) = to_return {
            self.to_iterate
                .extend(self.graph.visit_node(to_return).into_iter().map(Reverse));
        }

        to_return
    }
}

impl From<InstructionGraph> for SimpleTraverser {
    fn from(graph: InstructionGraph) -> Self {
        let to_iterate = graph.get_starting_nodes().map(Reverse).collect();

        SimpleTraverser { graph, to_iterate }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order() {
        let mut graph = InstructionGraph::new();

        graph.add_link('C', 'F');
        graph.add_link('C', 'B');
        graph.add_link('B', 'G');

        assert_eq!(
            SimpleTraverser::from(graph).collect::<Vec<_>>(),
            vec!['C', 'B', 'F', 'G']
        )
    }
}
